use std::collections::HashMap;

use system::{Dependency, DependencySet};

pub fn dep_head_mapping(dependencies: &DependencySet) -> HashMap<usize, Dependency> {
    let mut mapping = HashMap::new();

    for dep in dependencies {
        mapping.insert(dep.dependent, dep.clone());
    }

    mapping
}
