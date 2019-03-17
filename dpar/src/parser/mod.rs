//! Dependency parsers
//!
//! This module defines the `Parse` and `ParseBatch` traits for dependency
//! parsers. A greedy (linear-time) parser is also provided.

use conllx::Token;

use crate::system::DependencySet;
use failure::Error;

/// A dependency parser without batch processing.
pub trait Parse {
    /// Parse a sentence, returning the dependency relations.
    fn parse(&mut self, sentence: &[Token]) -> Result<DependencySet, Error>;
}

/// A dependency parser with batch processing.
pub trait ParseBatch {
    /// Parse a batch of sentence, returning the dependency relations.
    fn parse_batch<S>(&mut self, sentences: &[S]) -> Result<Vec<DependencySet>, Error>
    where
        S: AsRef<[Token]>;
}

mod greedy_parser;
pub use self::greedy_parser::GreedyParser;
