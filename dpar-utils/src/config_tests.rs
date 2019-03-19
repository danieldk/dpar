use std::fs::File;

use lazy_static::*;

use super::{Config, Lookup, Lookups, Model, Parser, TomlRead, Train};

lazy_static! {
    static ref BASIC_PARSER_CHECK: Config = Config {
        parser: Parser {
            pproj: true,
            system: String::from("stackproj"),
            inputs: String::from("parser.inputs"),
            transitions: String::from("parser.transitions"),
            train_batch_size: 8192,
            parse_batch_size: 8192,
        },
        model: Model {
            graph: String::from("parser.graph"),
            parameters: String::from("params"),
            intra_op_parallelism_threads: 2,
            inter_op_parallelism_threads: 2,
        },
        train: Train {
            initial_lr: 0.05.into(),
            lr_scale: 0.5.into(),
            lr_patience: 5,
            patience: 10,
        },
        lookups: Lookups {
            word: Some(Lookup::Embedding {
                filename: String::from("word-vectors.bin"),
                op: String::from("model/tokens"),
                embed_op: String::from("model/token_embeds"),
            }),
            tag: Some(Lookup::Embedding {
                filename: String::from("tag-vectors.bin"),
                op: String::from("model/tags"),
                embed_op: String::from("model/tag_embeds"),
            }),
            deprel: Some(Lookup::Table {
                filename: String::from("deprels.lookup"),
                op: String::from("model/deprels"),
            }),
            feature: Some(Lookup::Table {
                filename: String::from("features.lookup"),
                op: String::from("model/features"),
            }),
        }
    };
}

#[test]
fn test_parse_config() {
    let f = File::open("testdata/basic-parse.conf").unwrap();
    let config = Config::from_toml_read(f).unwrap();
    assert_eq!(*BASIC_PARSER_CHECK, config);
}
