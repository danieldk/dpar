use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use conllx::{DisplaySentence, HeadProjectivizer, Projectivize, ReadSentence, Token};
use dpar::features::InputVectorizer;
use dpar::models::lr::LearningRateSchedule;
use dpar::models::tensorflow::{TensorflowModel, TrainCollector};
use dpar::system::{sentence_to_dependencies, DependencySet, ParserState};
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use dpar::train::GreedyTrainer;
use failure::Error;
use getopts::Options;
use indicatif::{ProgressBar, ProgressStyle};
use stdinout::OrExit;

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
    let vectorizer = InputVectorizer::new(lookups, inputs);

    eprintln!("Reading training data...");
    let train_data = read_data(&config, reader).or_exit("Cannot read training data", 1);

    let input_file = File::open(&matches.free[2]).or_exit("Cannot open validation treebank", 1);
    let reader = conllx::Reader::new(BufReader::new(
        FileProgress::new(input_file).or_exit("Cannot create progress bar", 1),
    ));
    eprintln!("Reading validation data...");
    let validation_data = read_data(&config, reader).or_exit("Cannot read validation data", 1);

    train(&config, vectorizer, train_data, validation_data).or_exit("Training failed", 1);
}

fn train(
    config: &Config,
    vectorizer: InputVectorizer,
    train_data: (Vec<Vec<Token>>, Vec<DependencySet>),
    validation_data: (Vec<Vec<Token>>, Vec<DependencySet>),
) -> Result<(), Error> {
    let train_fun: Box<dyn Fn(_, _, _, _) -> Result<_, _>> = match config.parser.system.as_ref() {
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

    train_fun(config, vectorizer, train_data, validation_data)
}

fn train_with_system<S>(
    config: &Config,
    vectorizer: InputVectorizer,
    train_data: (Vec<Vec<Token>>, Vec<DependencySet>),
    validation_data: (Vec<Vec<Token>>, Vec<DependencySet>),
) -> Result<(), Error>
where
    S: SerializableTransitionSystem,
{
    let system: S = load_transition_system_or_new(&config)?;

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
    let mut last_acc = 0.0;

    let mut lr_schedule = config.train.lr_schedule();

    for epoch in 0.. {
        let lr = lr_schedule.learning_rate(epoch, last_acc);

        let (loss, acc) = run_epoch(config, &mut model, &train_data, true, lr)?;
        eprintln!(
            "Epoch {} (train, lr: {}): loss: {:.4}, acc: {:.4}",
            epoch, lr, loss, acc
        );
        model
            .save(format!("epoch-{}", epoch))
            .or_exit(format!("Cannot save model for epoch {}", epoch), 1);

        let (loss, acc) = run_epoch(config, &mut model, &validation_data, false, lr)?;

        last_acc = acc;
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
    config: &Config,
    model: &mut TensorflowModel<S>,
    data: &(Vec<Vec<Token>>, Vec<DependencySet>),
    is_training: bool,
    lr: f32,
) -> Result<(f32, f32), Error>
where
    S: SerializableTransitionSystem,
{
    let epoch_type = if is_training { "train" } else { "validation" };

    let progress = ProgressBar::new(data.0.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template(&format!("{{bar}} {} batch {{pos}}/{{len}}", epoch_type)),
    );

    let collector = TrainCollector::new(model, config.parser.train_batch_size, is_training, lr);
    let mut trainer = GreedyTrainer::new(collector);

    for (sentence, dependency_set) in data.0.iter().zip(data.1.iter()) {
        let mut parser_state = ParserState::new(&sentence);
        trainer.parse_state(dependency_set, &mut parser_state)?;
        progress.inc(1);
    }

    let perf = trainer.into_collector().finish();

    progress.finish();

    Ok((perf.loss, perf.accuracy))
}

fn read_data<R>(
    config: &Config,
    reader: conllx::Reader<R>,
) -> Result<(Vec<Vec<Token>>, Vec<DependencySet>), Error>
where
    R: BufRead,
{
    let projectivizer = HeadProjectivizer::new();

    let mut sentences = Vec::new();
    let mut gold_dependencies = Vec::new();
    for sentence in reader.sentences() {
        let sentence: Vec<_> = if config.parser.pproj {
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

        sentences.push(sentence);
        gold_dependencies.push(dependencies);
    }

    Ok((sentences, gold_dependencies))
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
