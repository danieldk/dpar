use std::collections::HashSet;

use conllx::Sentence;

use {ErrorKind, Result};

mod dependency;
pub use self::dependency::{Dependency, DependencySet};

mod parser_state;
pub use self::parser_state::ParserState;

mod trans_system;
pub use self::trans_system::{Transition, TransitionLookup, TransitionSystem};

pub fn sentence_to_dependencies(sentence: &Sentence) -> Result<DependencySet> {
    let mut dependencies = HashSet::new();

    for (idx, token) in sentence.as_tokens().iter().enumerate() {
        let head = try!(
            token
                .head()
                .ok_or(ErrorKind::ParseError(format!("Token {} has no head", idx)))
        );
        let head_rel = try!(token.head_rel().ok_or(ErrorKind::ParseError(format!(
            "Token {} has no head relation",
            idx
        ))));

        dependencies.insert(Dependency {
            head: head,
            relation: head_rel.to_owned(),
            dependent: idx + 1,
        });
    }

    Ok(dependencies)
}
