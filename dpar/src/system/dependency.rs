use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dependency {
    pub head: usize,
    pub dependent: usize,
    pub relation: String,
}

pub type DependencySet = HashSet<Dependency>;
