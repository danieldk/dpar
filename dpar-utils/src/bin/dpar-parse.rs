extern crate conllx;
extern crate dpar;
extern crate dpar_utils;
extern crate failure;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

use conllx::{Deprojectivize, HeadProjectivizer, ReadSentence, Sentence, WriteSentence};
use dpar::features::InputVectorizer;
use dpar::guide::BatchGuide;
use dpar::models::tensorflow::{LayerOps, TensorflowModel};
use dpar::parser::{GreedyParser, ParseBatch};
use dpar::system::{DependencySet, TransitionSystem};
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use failure::Error;
use getopts::Options;
use stdinout::{Input, OrExit, Output};

use dpar_utils::{Config, SerializableTransitionSystem, TomlRead};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] CONFIG [INPUT]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).or_exit("Cannot parse options", 1);

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, &opts);
        return;
    }

    let config_file = File::open(&matches.free[0]).or_exit("Cannot open configuration file", 1);
    let mut config =
        Config::from_toml_read(config_file).or_exit("Cannot read configuration file as TOML", 1);
    config
        .relativize_paths(&matches.free[0])
        .or_exit("Cannot relativize paths in the configuration file", 1);

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot open treebank", 1));

    let output = Output::from(matches.free.get(2));
    let writer = conllx::Writer::new(BufWriter::new(
        output
            .write()
            .or_exit("Cannot open output treebank for writing", 1),
    ));

    parse(&config, reader, writer).or_exit("Parsing failed", 1);
}

fn parse<R, W>(
    config: &Config,
    reader: conllx::Reader<R>,
    writer: conllx::Writer<W>,
) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    let parse_fun: Box<Fn(_, _, _) -> Result<_, _>> = match config.parser.system.as_ref() {
        "arceager" => Box::new(parse_with_system::<R, W, ArcEagerSystem>),
        "archybrid" => Box::new(parse_with_system::<R, W, ArcHybridSystem>),
        "arcstandard" => Box::new(parse_with_system::<R, W, ArcStandardSystem>),
        "stackproj" => Box::new(parse_with_system::<R, W, StackProjectiveSystem>),
        "stackswap" => Box::new(parse_with_system::<R, W, StackSwapSystem>),
        _ => {
            eprintln!("Unsupported transition system: {}", config.parser.system);
            process::exit(1);
        }
    };

    parse_fun(config, reader, writer)
}

fn parse_with_system<R, W, S>(
    config: &Config,
    reader: conllx::Reader<R>,
    writer: conllx::Writer<W>,
) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
    S: SerializableTransitionSystem,
{
    let inputs = config.parser.load_inputs()?;
    let lookups = config.lookups.load_lookups()?;
    let layer_ops = config.lookups.layer_ops();
    let vectorizer = InputVectorizer::new(lookups, inputs);
    let system: S = load_system_generic(config)?;
    let guide = load_model(&config, system, vectorizer, &layer_ops)?;
    let parser = GreedyParser::new(guide);

    let mut n_sents = 0;
    let start = Instant::now();

    {
        let mut sent_proc = SentProcessor::new(
            parser,
            config.parser.pproj,
            config.parser.parse_batch_size,
            writer,
        );

        for sentence in reader.sentences() {
            let mut sentence = sentence.or_exit("Cannot read sentence", 1);
            sent_proc
                .process(sentence)
                .or_exit("Cannot parse sentence", 1);
            n_sents += 1;
        }
    }

    let elapsed = start.elapsed();
    let elapsed_sec = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000f32;

    eprintln!(
        "Parsed {} sentences in {:.1}s ({:.0} sents/s)",
        n_sents,
        elapsed_sec,
        n_sents as f32 / elapsed_sec
    );

    Ok(())
}

struct SentProcessor<G, W>
where
    G: BatchGuide,
    W: Write,
{
    parser: GreedyParser<G>,
    projectivizer: Option<HeadProjectivizer>,
    writer: conllx::Writer<W>,
    batch_size: usize,
    batch_sents: Vec<Sentence>,
}

impl<G, W> SentProcessor<G, W>
where
    G: BatchGuide,
    W: Write,
{
    pub fn new(
        parser: GreedyParser<G>,
        projectivize: bool,
        batch_size: usize,
        writer: conllx::Writer<W>,
    ) -> Self {
        let projectivizer = if projectivize {
            Some(HeadProjectivizer::new())
        } else {
            None
        };
        SentProcessor {
            parser,
            projectivizer,
            writer,
            batch_size,
            batch_sents: Vec::new(),
        }
    }

    pub fn process(&mut self, sent: Sentence) -> Result<(), Error> {
        self.batch_sents.push(sent);

        if self.batch_sents.len() == self.batch_size {
            self.parse_batch()?;
        }

        Ok(())
    }

    fn parse_batch(&mut self) -> Result<(), Error> {
        let dependencies = self
            .parser
            .parse_batch(&self.batch_sents)
            .or_exit("Batch parsing failed", 1);
        update_sentences(&mut self.batch_sents, dependencies);

        for sentence in &self.batch_sents {
            if let Some(ref projectivizer) = self.projectivizer {
                self.writer
                    .write_sentence(&projectivizer.deprojectivize(&sentence)?)?;
            } else {
                self.writer.write_sentence(&sentence)?;
            }
        }

        self.batch_sents.clear();

        Ok(())
    }
}

impl<G, W> Drop for SentProcessor<G, W>
where
    G: BatchGuide,
    W: Write,
{
    fn drop(&mut self) {
        if !self.batch_sents.is_empty() {
            match self.parse_batch() {
                Ok(_) => (),
                Err(e) => eprintln!("Could not parse last batch: {}", e),
            }
        }
    }
}

fn load_model<T>(
    config: &Config,
    system: T,
    vectorizer: InputVectorizer,
    layer_ops: &LayerOps<String>,
) -> Result<TensorflowModel<T>, Error>
where
    T: TransitionSystem,
{
    Ok(TensorflowModel::load_graph_with_weights(
        &config.model.config_to_protobuf()?,
        &config.model.read_graph()?,
        &config.model.parameters,
        system,
        vectorizer,
        &layer_ops,
    )?)
}

fn load_system_generic<T>(config: &Config) -> Result<T, Error>
where
    T: SerializableTransitionSystem,
{
    let transitions_path = Path::new(&config.parser.transitions);

    eprintln!("Loading transitions from: {:?}", transitions_path);

    let f = File::open(transitions_path)?;
    let system = T::from_cbor_read(f)?;

    Ok(system)
}

fn update_sentences(sentences: &mut [Sentence], dependencies: Vec<DependencySet>) {
    assert_eq!(sentences.len(), dependencies.len());

    for (sentence, dep_set) in sentences.iter_mut().zip(dependencies) {
        // Clear any existing annotations.
        for mut token in sentence.iter_mut() {
            token.set_head(None);
            let none: Option<&str> = None;
            token.set_head_rel(none);
        }

        for dependency in dep_set {
            let idx = dependency.dependent - 1;
            sentence[idx].set_head(Some(dependency.head));
            sentence[idx].set_head_rel(Some(dependency.relation));
        }
    }
}
