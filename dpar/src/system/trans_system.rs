use std::fmt::Debug;
use std::hash::Hash;

use serde::de::DeserializeOwned;
use serde::Serialize;

use guide::Guide;
use numberer::Numberer;
use system::{DependencySet, ParserState};

pub trait TransitionSystem {
    type T: Transition;
    type O: Guide<T = Self::T>;

    fn is_terminal(state: &ParserState) -> bool;
    fn oracle(gold_dependencies: &DependencySet) -> Self::O;
    fn transitions(&self) -> &Numberer<Self::T>;
    fn transitions_mut(&mut self) -> &mut Numberer<Self::T>;
}

pub trait Transition: Clone + Debug + Eq + Hash + Serialize + DeserializeOwned {
    type S: TransitionSystem;

    fn is_possible(&self, state: &ParserState) -> bool;
    fn apply(&self, state: &mut ParserState);
}
