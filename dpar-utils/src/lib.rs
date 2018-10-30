extern crate conllx;

extern crate dpar;

#[macro_use]
extern crate error_chain;

extern crate failure;

extern crate indicatif;

extern crate ordered_float;

extern crate protobuf;

extern crate serde;

extern crate serde_cbor;

#[macro_use]
extern crate serde_derive;

extern crate tensorflow;

extern crate tf_embed;

extern crate tf_proto;

extern crate toml;

mod config;
pub use config::{Config, Lookup, Lookups, Model, Parser, Train};

mod error;
pub use error::*;

mod progress;
pub use progress::FileProgress;

mod serialization;
pub use serialization::{CborRead, CborWrite, SerializableTransitionSystem, TomlRead};

mod stored_table;
pub use stored_table::StoredLookupTable;

#[macro_use]
mod util;

mod or_exit;
pub use or_exit::OrExit;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod config_tests;
