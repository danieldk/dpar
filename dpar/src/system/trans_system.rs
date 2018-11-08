use std::fmt::Debug;
use std::hash::Hash;

use serde::de::DeserializeOwned;
use serde::Serialize;

use guide::Guide;
use lookup::Lookup;
use system::{DependencySet, ParserState};

pub trait TransitionSystem {
    type Transition: Transition;
    type Oracle: Guide<Transition = Self::Transition>;

    fn is_terminal(state: &ParserState) -> bool;
    fn oracle(gold_dependencies: &DependencySet) -> Self::Oracle;
    fn transitions(&self) -> &Lookup<Self::Transition>;
}

pub trait Transition: Clone + Debug + Eq + Hash + Serialize + DeserializeOwned {
    type S: TransitionSystem;

    fn is_possible(&self, state: &ParserState) -> bool;
    fn apply(&self, state: &mut ParserState);
}
