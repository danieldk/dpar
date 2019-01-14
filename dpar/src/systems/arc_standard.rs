use std::collections::HashMap;

use guide::Guide;
use system::{
    Dependency, DependencySet, ParserState, Transition, TransitionLookup, TransitionSystem,
};

use features::addr::Source;
use system::AttachmentAddr;
use systems::util::dep_head_mapping;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct ArcStandardSystem {
    transitions: TransitionLookup<ArcStandardTransition>,
}

impl ArcStandardSystem {
    pub fn new() -> Self {
        ArcStandardSystem {
            transitions: TransitionLookup::default(),
        }
    }
}

impl Default for ArcStandardSystem {
    fn default() -> Self {
        ArcStandardSystem::new()
    }
}

impl TransitionSystem for ArcStandardSystem {
    type Transition = ArcStandardTransition;
    type Oracle = ArcStandardOracle;

    const ATTACHMENT_ADDRS: [AttachmentAddr; 2] = [
        AttachmentAddr {
            head: Source::Buffer(0),
            dependent: Source::Stack(0),
        },
        AttachmentAddr {
            head: Source::Stack(0),
            dependent: Source::Buffer(0),
        },
    ];

    fn is_terminal(state: &ParserState) -> bool {
        state.buffer().is_empty()
    }

    fn oracle(gold_dependencies: &DependencySet) -> Self::Oracle {
        ArcStandardOracle::new(gold_dependencies)
    }

    fn transitions(&self) -> &TransitionLookup<Self::Transition> {
        &self.transitions
    }
}

/// Arc-standard transition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ArcStandardTransition {
    LeftArc(String),
    RightArc(String),
    Shift,
}

impl Transition for ArcStandardTransition {
    type S = ArcStandardSystem;

    fn is_possible(&self, state: &ParserState) -> bool {
        match self {
            &ArcStandardTransition::LeftArc(_) => {
                !state.stack().len() > 1 && !state.buffer().is_empty()
            }
            &ArcStandardTransition::RightArc(_) => {
                !state.stack().is_empty() && !state.buffer().is_empty()
            }
            &ArcStandardTransition::Shift => !state.buffer().is_empty(),
        }
    }

    fn apply(&self, state: &mut ParserState) {
        let stack_size = state.stack().len();

        match self {
            &ArcStandardTransition::LeftArc(ref rel) => {
                let head = state.buffer()[0];
                let dependent = state.stack()[stack_size - 1];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.stack_mut().pop();
            }
            &ArcStandardTransition::RightArc(ref rel) => {
                let dependent = state.buffer()[0];
                let head = state.stack()[stack_size - 1];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.buffer_mut()[0] = state.stack()[stack_size - 1];
                state.stack_mut().pop();
            }
            &ArcStandardTransition::Shift => {
                let next = state.buffer()[0];
                state.stack_mut().push(next);
                state.buffer_mut().remove(0);
            }
        }
    }
}

/// Arc-standard system oracle.
pub struct ArcStandardOracle {
    dependencies: HashMap<usize, Dependency>,
}

impl ArcStandardOracle {
    pub fn new(gold_dependencies: &DependencySet) -> ArcStandardOracle {
        ArcStandardOracle {
            dependencies: dep_head_mapping(gold_dependencies),
        }
    }

    fn needed_for_attachment(&self, state: &ParserState, token: usize) -> bool {
        for buf_token in state.buffer() {
            if let Some(dep) = self.dependencies.get(&buf_token) {
                if dep.head == token {
                    return true;
                }
            }
        }

        false
    }
}

impl Guide for ArcStandardOracle {
    type Transition = ArcStandardTransition;

    fn best_transition(&mut self, state: &ParserState) -> ArcStandardTransition {
        let stack = &state.stack();
        let buffer = &state.buffer();

        if !stack.is_empty() {
            let stack_tip = stack[stack.len() - 1];
            let buffer_head = buffer[0];

            if let Some(dep) = self.dependencies.get(&stack_tip) {
                let la = ArcStandardTransition::LeftArc(dep.relation.clone());

                if dep.head == buffer_head && la.is_possible(state) {
                    return la;
                }
            }

            if let Some(dep) = self.dependencies.get(&buffer_head) {
                let ra = ArcStandardTransition::RightArc(dep.relation.clone());

                if dep.head == stack_tip
                    && ra.is_possible(state)
                    && !self.needed_for_attachment(state, buffer_head)
                {
                    return ra;
                }
            }
        }

        ArcStandardTransition::Shift
    }
}
