//! Dependency parsers
//!
//! This module defines the `Parse` and `ParseBatch` traits for dependency
//! parsers. A greedy (linear-time) parser is also provided.

use conllx::Sentence;

use failure::Error;
use system::DependencySet;

/// A dependency parser without batch processing.
pub trait Parse {
    /// Parse a sentence, returning the dependency relations.
    fn parse(&mut self, sentence: &Sentence) -> Result<DependencySet, Error>;
}

/// A dependency parser with batch processing.
pub trait ParseBatch {
    /// Parse a batch of sentence, returning the dependency relations.
    fn parse_batch(&mut self, sentences: &[Sentence]) -> Result<Vec<DependencySet>, Error>;
}

mod greedy_parser;
pub use self::greedy_parser::GreedyParser;
