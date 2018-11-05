extern crate colored;
extern crate conllx;
extern crate dpar;
extern crate dpar_utils;
extern crate failure;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::io::{BufRead, BufWriter, Write};
use std::process;

use colored::*;
use conllx::{DisplaySentence, HeadProjectivizer, Projectivize, ReadSentence};
use dpar::guide::Guide;
use dpar::system::{sentence_to_dependencies, ParserState, Transition, TransitionSystem};
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use failure::Error;
use getopts::Options;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] SYSTEM [INPUT]", program);
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

    if matches.free.len() < 1 || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot open treebank", 1));

    let output = Output::from(matches.free.get(2));
    let writer = BufWriter::new(output.write().or_exit("Cannot create transition output", 1));

    parse(&matches.free[0], reader, writer).or_exit("Cannot print transitions", 1);
}

fn parse<R, W>(system: &str, reader: conllx::Reader<R>, writer: BufWriter<W>) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    match system {
        "arceager" => parse_with_system::<R, W, ArcEagerSystem>(reader, writer),
        "archybrid" => parse_with_system::<R, W, ArcHybridSystem>(reader, writer),
        "arcstandard" => parse_with_system::<R, W, ArcStandardSystem>(reader, writer),
        "stackproj" => parse_with_system::<R, W, StackProjectiveSystem>(reader, writer),
        "stackswap" => parse_with_system::<R, W, StackSwapSystem>(reader, writer),
        _ => {
            eprintln!("Unsupported transition system: {}", system);
            process::exit(1);
        }
    }
}

fn parse_with_system<R, W, S>(
    reader: conllx::Reader<R>,
    mut writer: BufWriter<W>,
) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
    S: TransitionSystem,
{
    let projectivizer = HeadProjectivizer::new();

    for sentence in reader.sentences() {
        let sentence = projectivizer.projectivize(&sentence?)?;

        let gold_dependencies = sentence_to_dependencies(&sentence).or_exit(
            format!(
                "Cannot extract gold dependencies:\n{}",
                DisplaySentence(&sentence)
            ),
            1,
        );
        let mut oracle = S::oracle(&gold_dependencies);

        let mut state = ParserState::new(&sentence);

        // Print initial state.
        print_tokens(&mut writer, &state, Source::Stack)?;
        print_tokens(&mut writer, &state, Source::Buffer)?;

        while !S::is_terminal(&state) {
            let next_transition = oracle.best_transition(&state);
            next_transition.apply(&mut state);

            // Print transition and state.
            writeln!(writer, "{}", format!("{:?}", next_transition).purple())?;
            print_tokens(&mut writer, &state, Source::Stack)?;
            print_tokens(&mut writer, &state, Source::Buffer)?;
        }
    }

    Ok(())
}

enum Source {
    Buffer,
    Stack,
}

fn print_tokens<W>(writer: &mut W, state: &ParserState, source: Source) -> Result<(), Error>
where
    W: Write,
{
    let prefix = match source {
        Source::Buffer => "Buffer",
        Source::Stack => "Stack",
    };

    let indices = match source {
        Source::Buffer => state.buffer(),
        Source::Stack => state.stack(),
    };

    writeln!(
        writer,
        "{}: {}",
        prefix,
        indices
            .iter()
            .map(|&idx| state.tokens()[idx])
            .collect::<Vec<_>>()
            .join(", ")
    )?;

    Ok(())
}
