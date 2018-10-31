//! Parser guides
//!
//! A parser uses a guide to decide which transition to us given a particular
//! parser state. This module defines traits for parser guides, `Guide` and
//! `BatchGuide`, and provides an implementation using Tensorflow in the
//! `Tensorflow` submodule.

use system::{ParserState, Transition};

/// Guide for parsers without batch processing
pub trait Guide {
    type Transition: Transition;

    /// Returns the best (permissible) transition given the current parser
    /// state.
    fn best_transition(&mut self, state: &ParserState) -> Self::Transition;
}

/// Guide for parsers with batch processing
pub trait BatchGuide {
    type Transition: Transition;

    /// Returns the best (permissible) transitions for a slice of parser
    /// states. The transitions are returned in the same order as the
    /// parser states. So, the *0th* transition is the best transition
    /// for the *0th* parser state.
    fn best_transitions(&mut self, states: &[&ParserState]) -> Vec<Self::Transition>;
}
