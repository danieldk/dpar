extern crate conllx;
extern crate dpar;
#[macro_use]
extern crate dpar_utils;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process;

use conllx::{HeadProjectivizer, Projectivize, ReadSentence};
use dpar::train::HDF5Collector;
use dpar::features::InputVectorizer;
use dpar::system::{sentence_to_dependencies, ParserState};
use dpar::systems::{ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem,
                    StackSwapSystem};
use dpar::train::GreedyTrainer;
use getopts::Options;
use stdinout::Input;

use dpar_utils::{Config, OrExit, Result, SerializableTransitionSystem, TomlRead};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] CONFIG DATA OUTPUT.HDF5", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).or_exit();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() != 3 {
        print_usage(&program, opts);
        return;
    }

    let config_file = File::open(&matches.free[0]).or_exit();
    let mut config = Config::from_toml_read(config_file).or_exit();
    config.relativize_paths(&matches.free[0]).or_exit();

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit());

    train(&config, reader, &matches.free[2]).or_exit();
}

fn train<R>(config: &Config, reader: conllx::Reader<R>, hdf5_filename: &str) -> Result<()>
where
    R: BufRead,
{
    match config.parser.system.as_ref() {
        "arceager" => train_with_system::<R, ArcEagerSystem>(config, reader, hdf5_filename),
        "archybrid" => train_with_system::<R, ArcHybridSystem>(config, reader, hdf5_filename),
        "arcstandard" => train_with_system::<R, ArcStandardSystem>(config, reader, hdf5_filename),
        "stackproj" => train_with_system::<R, StackProjectiveSystem>(config, reader, hdf5_filename),
        "stackswap" => train_with_system::<R, StackSwapSystem>(config, reader, hdf5_filename),
        _ => {
            stderr!("Unsupported transition system: {}", config.parser.system);
            process::exit(1);
        }
    }
}

fn train_with_system<R, S>(
    config: &Config,
    reader: conllx::Reader<R>,
    hdf5_filename: &str,
) -> Result<()>
where
    R: BufRead,
    S: SerializableTransitionSystem,
{
    let lookups = config.lookups.load_lookups()?;
    let inputs = config.parser.load_inputs()?;
    let vectorizer = InputVectorizer::new(lookups, inputs);
    let system: S = load_transition_system_or_new(&config)?;
    let collector = HDF5Collector::new(
        system,
        hdf5_filename,
        vectorizer,
        config.parser.train_batch_size,
    )?;
    let mut trainer = GreedyTrainer::new(collector);
    let projectivizer = HeadProjectivizer::new();

    for sentence in reader.sentences() {
        let sentence = if config.parser.pproj {
            projectivizer.projectivize(&sentence?)?
        } else {
            sentence?
        };

        let dependencies = sentence_to_dependencies(&sentence).or_exit();

        let mut state = ParserState::new(&sentence);
        trainer.parse_state(&dependencies, &mut state)?;
    }

    write_transition_system(&config, trainer.collector().transition_system())
}

fn load_transition_system_or_new<T>(config: &Config) -> Result<T>
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

fn write_transition_system<T>(config: &Config, system: &T) -> Result<()>
where
    T: SerializableTransitionSystem,
{
    let transitions_path = Path::new(&config.parser.transitions);
    if transitions_path.exists() {
        return Ok(());
    }

    let mut f = File::create(transitions_path)?;
    system.to_cbor_write(&mut f)?;
    Ok(())
}
