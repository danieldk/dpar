//! Feature extractors
//!
//! This module contains functionality to extract a feature vector from
//! a `ParserState`. The `AddressedValue` instances are used to refer
//! to specific parts of the parser state, such as *'the part-of-speech
//! tag of the next word on the buffer'*. Each such addressed value
//! can be seen as a feature functions. Multiple addressed values
//! (`AddressedValues`) can be combined and used by an `InputVectorizer`
//! to extract feature vectors from parser states.

pub mod addr;

mod input_layers;
pub use self::input_layers::{AddressedValues, InputVector, InputVectorizer, Layer, LayerLookups};

mod lookup;
pub use self::lookup::{
    BoxedLookup, Embeddings, Lookup, LookupResult, LookupTable, LookupType, MutableLookupTable,
};

pub(crate) mod parse_addr;

#[cfg(test)]
mod addr_tests;

#[cfg(test)]
mod parse_addr_tests;
