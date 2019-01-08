use std::collections::HashSet;

use conllx::Sentence;
use failure::Error;

mod dependency;
pub use self::dependency::{Dependency, DependencySet};

mod parser_state;
pub use self::parser_state::ParserState;

mod trans_system;
pub use self::trans_system::{AttachmentAddr, Transition, TransitionLookup, TransitionSystem};

pub fn sentence_to_dependencies(sentence: &Sentence) -> Result<DependencySet, Error> {
    let mut dependencies = HashSet::new();

    for (idx, token) in sentence.iter().enumerate() {
        let head = token
            .head()
            .ok_or(format_err!("Token {} has no head", idx))?;
        let head_rel = token
            .head_rel()
            .ok_or(format_err!("Token {} has no head relation", idx))?;

        dependencies.insert(Dependency {
            head: head,
            relation: head_rel.to_owned(),
            dependent: idx + 1,
        });
    }

    Ok(dependencies)
}
