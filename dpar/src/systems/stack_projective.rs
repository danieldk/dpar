use std::collections::HashMap;

use guide::Guide;
use system::{
    Dependency, DependencySet, ParserState, Transition, TransitionLookup, TransitionSystem,
};

use features::addr::Source;
use system::AttachmentAddr;
use systems::util::dep_head_mapping;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct StackProjectiveSystem {
    transitions: TransitionLookup<StackProjectiveTransition>,
}

impl StackProjectiveSystem {
    pub fn new() -> Self {
        StackProjectiveSystem {
            transitions: TransitionLookup::default(),
        }
    }
}

impl Default for StackProjectiveSystem {
    fn default() -> Self {
        StackProjectiveSystem::new()
    }
}

impl TransitionSystem for StackProjectiveSystem {
    type Transition = StackProjectiveTransition;
    type Oracle = StackProjectiveOracle;

    const ATTACHMENT_ADDRS: [AttachmentAddr; 2] = [
        AttachmentAddr {
            head: Source::Stack(0),
            dependent: Source::Stack(1),
        },
        AttachmentAddr {
            head: Source::Stack(1),
            dependent: Source::Stack(0),
        },
    ];

    fn is_terminal(state: &ParserState) -> bool {
        state.buffer().is_empty() && state.stack().len() == 1
    }

    fn oracle(gold_dependencies: &DependencySet) -> Self::Oracle {
        StackProjectiveOracle::new(gold_dependencies)
    }

    fn transitions(&self) -> &TransitionLookup<Self::Transition> {
        &self.transitions
    }
}

/// Stack-projective transition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum StackProjectiveTransition {
    LeftArc(String),
    RightArc(String),
    Shift,
}

impl Transition for StackProjectiveTransition {
    type S = StackProjectiveSystem;

    fn is_possible(&self, state: &ParserState) -> bool {
        match self {
            &StackProjectiveTransition::LeftArc(_) => state.stack().len() > 2,
            &StackProjectiveTransition::RightArc(_) => state.stack().len() > 1,
            &StackProjectiveTransition::Shift => !state.buffer().is_empty(),
        }
    }

    fn apply(&self, state: &mut ParserState) {
        let stack_size = state.stack().len();

        match self {
            &StackProjectiveTransition::LeftArc(ref rel) => {
                let head = state.stack()[stack_size - 1];
                let dependent = state.stack()[stack_size - 2];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.stack_mut().remove(stack_size - 2);
            }
            &StackProjectiveTransition::RightArc(ref rel) => {
                let dependent = state.stack()[stack_size - 1];
                let head = state.stack()[stack_size - 2];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.stack_mut().pop();
            }
            &StackProjectiveTransition::Shift => {
                let next = state.buffer_mut().remove(0);
                state.stack_mut().push(next);
            }
        }
    }
}

pub struct StackProjectiveOracle {
    dependencies: HashMap<usize, Dependency>,
}

impl StackProjectiveOracle {
    pub fn new(gold_dependencies: &DependencySet) -> StackProjectiveOracle {
        StackProjectiveOracle {
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

impl Guide for StackProjectiveOracle {
    type Transition = StackProjectiveTransition;

    fn best_transition(&mut self, state: &ParserState) -> StackProjectiveTransition {
        let stack = &state.stack();

        if stack.len() > 1 {
            let stack0 = stack[stack.len() - 1];
            let stack1 = stack[stack.len() - 2];

            if let Some(dep) = self.dependencies.get(&stack1) {
                let la = StackProjectiveTransition::LeftArc(dep.relation.clone());

                if dep.head == stack0 && la.is_possible(state) {
                    return la;
                }
            }

            if let Some(dep) = self.dependencies.get(&stack0) {
                let ra = StackProjectiveTransition::RightArc(dep.relation.clone());

                if dep.head == stack1
                    && ra.is_possible(state)
                    && !self.needed_for_attachment(state, stack0)
                {
                    return ra;
                }
            }
        }

        StackProjectiveTransition::Shift
    }
}
