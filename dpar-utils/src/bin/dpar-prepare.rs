extern crate conllx;
extern crate dpar;
extern crate dpar_utils;
extern crate failure;
extern crate getopts;
#[macro_use]
extern crate serde_derive;
extern crate stdinout;
extern crate toml;

use std::collections::BTreeSet;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process;

use conllx::{DisplaySentence, HeadProjectivizer, Projectivize, ReadSentence};
use dpar::features::addr::Layer::Char;
use dpar::features::{AddressedValues, InputVectorizer, Layer, Lookup};
use dpar::system::{sentence_to_dependencies, ParserState};
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use dpar::train::{GreedyTrainer, NoopCollector};
use failure::{err_msg, Error};
use getopts::Options;
use stdinout::{Input, OrExit, Output};

use dpar_utils::{Config, SerializableTransitionSystem, TomlRead};

/// Ad-hoc shapes structure, which can be used to construct the
/// Tensorflow parsing graph.
#[derive(Serialize)]
struct Shapes {
    batch_size: usize,
    tokens: usize,
    tags: usize,
    deprels: usize,
    features: usize,
    chars: usize,
    deprel_embeds: usize,
    n_features: usize,
    n_labels: usize,
    prefix_len: usize,
    suffix_len: usize,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] CONFIG TRAIN_DATA SHAPES", program);
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

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let config_file = File::open(&matches.free[0]).or_exit("Cannot open configuration file", 1);
    let mut config =
        Config::from_toml_read(config_file).or_exit("Cannot read configuration file as TOML", 1);
    config
        .relativize_paths(&matches.free[0])
        .or_exit("Cannot relativize paths in the configuration file", 1);

    let input = Input::from(matches.free.get(1));
    let treebank_reader = conllx::Reader::new(input.buf_read().or_exit("Cannot open treebank", 1));
    let output = Output::from(matches.free.get(2));
    let shapes_writer = output.write().or_exit("Cannot create shape file", 1);

    prepare(&config, treebank_reader, shapes_writer).or_exit("Cannot prepare parser data", 1);
}

fn prepare<R, W>(
    config: &Config,
    treebank_reader: conllx::Reader<R>,
    shapes_write: W,
) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    let prepare_fun: Box<Fn(_, _, _) -> Result<_, _>> = match config.parser.system.as_ref() {
        "arceager" => Box::new(prepare_with_system::<R, W, ArcEagerSystem>),
        "archybrid" => Box::new(prepare_with_system::<R, W, ArcHybridSystem>),
        "arcstandard" => Box::new(prepare_with_system::<R, W, ArcStandardSystem>),
        "stackproj" => Box::new(prepare_with_system::<R, W, StackProjectiveSystem>),
        "stackswap" => Box::new(prepare_with_system::<R, W, StackSwapSystem>),
        _ => {
            eprintln!("Unsupported transition system: {}", config.parser.system);
            process::exit(1);
        }
    };

    prepare_fun(config, treebank_reader, shapes_write)
}

fn prepare_with_system<R, W, S>(
    config: &Config,
    treebank_reader: conllx::Reader<R>,
    shapes_write: W,
) -> Result<(), Error>
where
    R: BufRead,
    S: SerializableTransitionSystem,
    W: Write,
{
    let lookups = config.lookups.create_lookups()?;
    let inputs = config.parser.load_inputs()?;
    let association_strenghts = config.parser.load_associations()?;
    let vectorizer = InputVectorizer::new(lookups, inputs, association_strenghts);
    let system: S = S::default();
    let collector = NoopCollector::new(system, vectorizer)?;
    let mut trainer = GreedyTrainer::new(collector);
    let projectivizer = HeadProjectivizer::new();

    for sentence in treebank_reader.sentences() {
        let sentence = if config.parser.pproj {
            projectivizer.projectivize(&sentence?)?
        } else {
            sentence?
        };

        let dependencies = sentence_to_dependencies(&sentence).or_exit(
            format!(
                "Cannot extract dependencies from sentence:\n{}",
                DisplaySentence(&sentence)
            ),
            1,
        );

        let mut state = ParserState::new(&sentence);
        trainer.parse_state(&dependencies, &mut state)?;
    }

    write_transition_system(&config, trainer.collector().transition_system())?;

    write_shapes(config, trainer, shapes_write)
}

/// Write shape TOML.
fn write_shapes<W, S>(
    config: &Config,
    trainer: GreedyTrainer<S, NoopCollector<S>>,
    mut shapes_write: W,
) -> Result<(), Error>
where
    W: Write,
    S: SerializableTransitionSystem,
{
    let vectorizer = trainer.collector().input_vectorizer();
    let layer_sizes = vectorizer.layer_sizes();
    let layer_lookups = vectorizer.layer_lookups();

    let (prefix_len, suffix_len) = affix_lengths(vectorizer.layer_addrs())?;

    let shapes = Shapes {
        batch_size: config.parser.train_batch_size,
        tokens: layer_sizes[Layer::Token],
        tags: layer_sizes[Layer::Tag],
        deprels: layer_sizes[Layer::DepRel],
        features: layer_sizes[Layer::Feature],
        chars: layer_sizes[Layer::Char],
        deprel_embeds: layer_lookups
            .layer_lookup(Layer::DepRel)
            .map(Lookup::len)
            .unwrap_or(0),
        n_features: layer_lookups
            .layer_lookup(Layer::Feature)
            .map(Lookup::len)
            .unwrap_or(0),
        n_labels: trainer.collector().transition_system().transitions().len(),
        prefix_len,
        suffix_len,
    };

    write!(
        shapes_write,
        "{}",
        toml::to_string(&shapes).or_exit("Cannot convert shape data to TOML", 1)
    );

    Ok(())
}

fn affix_lengths(addrs: &AddressedValues) -> Result<(usize, usize), Error> {
    let mut prefix_lens = BTreeSet::new();
    let mut suffix_lens = BTreeSet::new();

    for addr in &addrs.0 {
        if let Char(prefix_len, suffix_len) = addr.layer {
            prefix_lens.insert(prefix_len);
            suffix_lens.insert(suffix_len);
        }
    }

    if prefix_lens.len() != 1 || suffix_lens.len() != 1 {
        Err(err_msg(
            "Models with varying prefix or suffix lengths are not supported",
        ))
    } else {
        Ok((
            prefix_lens.into_iter().next().unwrap(),
            suffix_lens.into_iter().next().unwrap(),
        ))
    }
}

fn write_transition_system<T>(config: &Config, system: &T) -> Result<(), Error>
where
    T: SerializableTransitionSystem,
{
    let transitions_path = Path::new(&config.parser.transitions);
    let mut f = File::create(transitions_path)?;
    system.to_cbor_write(&mut f)?;
    Ok(())
}
