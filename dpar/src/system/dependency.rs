use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Dependency {
    pub head: usize,
    pub relation: String,
    pub dependent: usize,
}

pub type DependencySet = HashSet<Dependency>;
