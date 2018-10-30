use std::fs::File;

use super::{Config, Lookup, Lookups, Model, Parser, TomlRead, Train};

lazy_static! {
    static ref BASIC_PARSER_CHECK: Config = Config {
        parser: Parser {
            pproj: true,
            system: String::from("stackproj"),
            inputs: String::from("parser.inputs"),
            transitions: String::from("parser.transitions"),
            train_batch_size: 8000,
            parse_batch_size: 4000,
        },
        model: Model {
            graph: String::from("model.bin"),
            parameters: String::from("params"),
            intra_op_parallelism_threads: 4,
            inter_op_parallelism_threads: 6,
        },
        lookups: Lookups {
            word: Some(Lookup::Embedding {
                filename: String::from("word-vectors.bin"),
                normalize: true,
                op: String::from("word_op"),
                embed_op: String::from("word_embed_op"),
            }),
            tag: Some(Lookup::Embedding {
                filename: String::from("tag-vectors.bin"),
                normalize: true,
                op: String::from("tag_op"),
                embed_op: String::from("tag_embed_op"),
            }),
            deprel: Some(Lookup::Embedding {
                filename: String::from("deprel-vectors.bin.real"),
                normalize: false,
                op: String::from("deprel_op"),
                embed_op: String::from("deprel_embed_op"),
            }),

            chars: None,
            feature: None
        },
        train: Train {
            initial_lr: 0.05.into(),
            decay_rate: 0.95.into(),
            decay_steps: 10,
            staircase: true,
            patience: 5,
        }
    };
}

#[test]
fn test_parse_config() {
    let f = File::open("testdata/basic-parse.conf").unwrap();
    let config = Config::from_toml_read(f).unwrap();
    assert_eq!(*BASIC_PARSER_CHECK, config);
}
