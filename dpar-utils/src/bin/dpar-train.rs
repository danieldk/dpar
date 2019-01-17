#[macro_use]
extern crate itertools;
extern crate conllx;
extern crate dpar;
extern crate dpar_utils;
extern crate failure;
extern crate getopts;
extern crate indicatif;
extern crate stdinout;
extern crate tensorflow;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use conllx::{DisplaySentence, HeadProjectivizer, Projectivize, ReadSentence};
use dpar::features::InputVectorizer;
use dpar::models::lr::LearningRateSchedule;
use dpar::models::tensorflow::{LayerTensors, TensorCollector, TensorflowModel};
use dpar::system::{sentence_to_dependencies, ParserState};
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use dpar::train::GreedyTrainer;
use failure::Error;
use getopts::Options;
use indicatif::{ProgressBar, ProgressStyle};
use stdinout::OrExit;
use tensorflow::Tensor;

use dpar_utils::{Config, FileProgress, SerializableTransitionSystem, TomlRead};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] CONFIG TRAIN_DATA VALID_DATA", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).or_exit("Cannot parse options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() != 3 {
        print_usage(&program, opts);
        return;
    }

    let config_file = File::open(&matches.free[0]).or_exit("Cannot open configuration file", 1);
    let mut config =
        Config::from_toml_read(config_file).or_exit("Cannot read configuration file as TOML", 1);
    config
        .relativize_paths(&matches.free[0])
        .or_exit("Cannot relativize paths in the configuration file", 1);

    let input_file = File::open(&matches.free[1]).or_exit("Cannot open training treebank", 1);
    let reader = conllx::Reader::new(BufReader::new(
        FileProgress::new(input_file).or_exit("Cannot create progress bar", 1),
    ));

    let lookups = config
        .lookups
        .load_lookups()
        .or_exit("Cannot load lookups", 1);
    let inputs = config
        .parser
        .load_inputs()
        .or_exit("Cannot load lookups", 1);
    let association_strengths = config
        .parser
        .load_associations()
        .or_exit("Cannot load association strengths", 1);
    let vectorizer = InputVectorizer::new(lookups, inputs, association_strengths);

    eprintln!("Vectorizing training data...");
    let (train_labels, train_lookup_inputs, train_non_lookup_inputs) =
        collect_data(&config, &vectorizer, reader).or_exit("Tensor collection failed", 1);

    let input_file = File::open(&matches.free[2]).or_exit("Cannot open validation treebank", 1);
    let reader = conllx::Reader::new(BufReader::new(
        FileProgress::new(input_file).or_exit("Cannot create progress bar", 1),
    ));
    eprintln!("Vectorizing validation data...");
    let (validation_labels, validation_lookup_inputs, validation_non_lookup_inputs) =
        collect_data(&config, &vectorizer, reader).or_exit("Tensor collection failed", 1);

    train(
        &config,
        vectorizer,
        train_labels,
        train_lookup_inputs,
        train_non_lookup_inputs,
        validation_labels,
        validation_lookup_inputs,
        validation_non_lookup_inputs,
    ).or_exit("Training failed", 1);
}

fn train(
    config: &Config,
    vectorizer: InputVectorizer,
    train_labels: Vec<Tensor<i32>>,
    train_lookup_inputs: Vec<LayerTensors<i32>>,
    train_non_lookup_inputs: Vec<Tensor<f32>>,
    validation_labels: Vec<Tensor<i32>>,
    validation_lookup_inputs: Vec<LayerTensors<i32>>,
    validation_non_lookup_inputs: Vec<Tensor<f32>>,
) -> Result<(), Error> {
    let train_fun: Box<Fn(_, _, _, _, _, _, _, _) -> Result<_, _>> =
        match config.parser.system.as_ref() {
            "arceager" => Box::new(train_with_system::<ArcEagerSystem>),
            "archybrid" => Box::new(train_with_system::<ArcHybridSystem>),
            "arcstandard" => Box::new(train_with_system::<ArcStandardSystem>),
            "stackproj" => Box::new(train_with_system::<StackProjectiveSystem>),
            "stackswap" => Box::new(train_with_system::<StackSwapSystem>),
            _ => {
                eprintln!("Unsupported transition system: {}", config.parser.system);
                process::exit(1);
            }
        };

    train_fun(
        config,
        vectorizer,
        train_labels,
        train_lookup_inputs,
        train_non_lookup_inputs,
        validation_labels,
        validation_lookup_inputs,
        validation_non_lookup_inputs,
    )
}

fn train_with_system<S>(
    config: &Config,
    vectorizer: InputVectorizer,
    train_labels: Vec<Tensor<i32>>,
    train_lookup_inputs: Vec<LayerTensors<i32>>,
    train_non_lookup_inputs: Vec<Tensor<f32>>,
    validation_labels: Vec<Tensor<i32>>,
    validation_lookup_inputs: Vec<LayerTensors<i32>>,
    validation_non_lookup_inputs: Vec<Tensor<f32>>,
) -> Result<(), Error>
where
    S: SerializableTransitionSystem,
{
    let system = S::default();
    let mut model = TensorflowModel::load_graph(
        &config
            .model
            .config_to_protobuf()
            .or_exit("Cannot convert Tensorflow configuration to protobuf", 1),
        &config
            .model
            .read_graph()
            .or_exit("Cannot read Tensorflow graph", 1),
        system,
        vectorizer,
        &config.lookups.layer_ops(),
    )?;

    let mut best_epoch = 0;
    let mut best_acc = 0.0;

    let lr_schedule = config.train.lr_schedule();

    for epoch in 0.. {
        let lr = lr_schedule.learning_rate(epoch);

        let (loss, acc) = run_epoch(
            &mut model,
            &train_labels,
            &train_lookup_inputs,
            &train_non_lookup_inputs,
            true,
            lr,
        );
        eprintln!(
            "Epoch {} (train, lr: {}): loss: {:.4}, acc: {:.4}",
            epoch, lr, loss, acc
        );
        model
            .save(format!("epoch-{}", epoch))
            .or_exit(format!("Cannot save model for epoch {}", epoch), 1);

        let (_, acc) = run_epoch(
            &mut model,
            &validation_labels,
            &validation_lookup_inputs,
            &validation_non_lookup_inputs,
            false,
            lr,
        );

        if acc > best_acc {
            best_epoch = epoch;
            best_acc = acc;
        }

        eprintln!(
            "Epoch {} (validation): loss: {:.4}, acc: {:.4}, best epoch: {}, best acc: {:.4}",
            epoch, loss, acc, best_epoch, best_acc
        );

        if epoch - best_epoch == config.train.patience {
            eprintln!(
                "Lost my patience! Best epoch: {} with accuracy: {:.4}",
                best_epoch, acc
            );
            break;
        }
    }

    Ok(())
}

fn run_epoch<S>(
    model: &mut TensorflowModel<S>,
    labels: &[Tensor<i32>],
    lookup_inputs: &[LayerTensors<i32>],
    non_lookup_inputs: &[Tensor<f32>],
    is_training: bool,
    lr: f32,
) -> (f32, f32)
where
    S: SerializableTransitionSystem,
{
    let epoch_type = if is_training { "train" } else { "validation" };

    let mut instances = 0;
    let mut loss = 0f32;
    let mut acc = 0f32;

    let progress = ProgressBar::new(labels.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template(&format!("{{bar}} {} batch {{pos}}/{{len}}", epoch_type)),
    );
    for (labels, lookup_inputs, non_lookup_inputs) in izip!(
        labels.iter(),
        lookup_inputs.iter(),
        non_lookup_inputs.iter()
    ) {
        let batch_perf = if is_training {
            model.train(lookup_inputs, non_lookup_inputs, labels, lr)
        } else {
            model.validate(lookup_inputs, non_lookup_inputs, labels)
        };

        loss += batch_perf.loss * labels.dims()[0] as f32;
        acc += batch_perf.accuracy * labels.dims()[0] as f32;
        instances += labels.dims()[0];
        progress.inc(1);
    }
    progress.finish();

    loss /= instances as f32;
    acc /= instances as f32;

    (loss, acc)
}

fn collect_data<R>(
    config: &Config,
    vectorizer: &InputVectorizer,
    reader: conllx::Reader<R>,
) -> Result<(Vec<Tensor<i32>>, Vec<LayerTensors<i32>>, Vec<Tensor<f32>>), Error>
where
    R: BufRead,
{
    let collect_fun: Box<Fn(_, _, _) -> Result<_, _>> = match config.parser.system.as_ref() {
        "arceager" => Box::new(collect_with_system::<R, ArcEagerSystem>),
        "archybrid" => Box::new(collect_with_system::<R, ArcHybridSystem>),
        "arcstandard" => Box::new(collect_with_system::<R, ArcStandardSystem>),
        "stackproj" => Box::new(collect_with_system::<R, StackProjectiveSystem>),
        "stackswap" => Box::new(collect_with_system::<R, StackSwapSystem>),
        _ => {
            eprintln!("Unsupported transition system: {}", config.parser.system);
            process::exit(1);
        }
    };

    collect_fun(config, vectorizer, reader)
}

fn collect_with_system<R, S>(
    config: &Config,
    vectorizer: &InputVectorizer,
    reader: conllx::Reader<R>,
) -> Result<(Vec<Tensor<i32>>, Vec<LayerTensors<i32>>, Vec<Tensor<f32>>), Error>
where
    R: BufRead,
    S: SerializableTransitionSystem,
{
    let system: S = load_transition_system_or_new(&config)?;
    let collector = TensorCollector::new(system, &vectorizer, config.parser.train_batch_size);
    let mut trainer = GreedyTrainer::new(collector);
    let projectivizer = HeadProjectivizer::new();

    for sentence in reader.sentences() {
        let sentence = if config.parser.pproj {
            projectivizer.projectivize(&sentence?)?
        } else {
            sentence?
        };

        let dependencies = sentence_to_dependencies(&sentence).or_exit(
            format!(
                "Cannot convert sentence to dependencies:\n{}",
                DisplaySentence(&sentence)
            ),
            1,
        );

        let mut state = ParserState::new(&sentence);
        trainer.parse_state(&dependencies, &mut state)?;
    }

    Ok(trainer.into_collector().into_data())
}

fn load_transition_system_or_new<T>(config: &Config) -> Result<T, Error>
where
    T: SerializableTransitionSystem,
{
    let transitions_path = Path::new(&config.parser.transitions);
    if !transitions_path.exists() {
        return Ok(T::default());
    }

    println!("Loading transitions from: {:?}", transitions_path);

    let f = File::open(transitions_path)?;
    let system = T::from_cbor_read(f)?;

    Ok(system)
}
