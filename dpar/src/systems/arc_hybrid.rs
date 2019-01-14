use std::collections::HashMap;

use guide::Guide;
use system::{
    Dependency, DependencySet, ParserState, Transition, TransitionLookup, TransitionSystem,
};

use features::addr::Source;
use system::AttachmentAddr;
use systems::util::dep_head_mapping;

/// The arc-hybrid transition system.
///
/// See: Marco Kuhlmann, et al., Dynamic Programming Algorithms for Transition-Based
/// Dependency Parsers, 2011.
#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct ArcHybridSystem {
    transitions: TransitionLookup<ArcHybridTransition>,
}

impl ArcHybridSystem {
    pub fn new() -> Self {
        ArcHybridSystem {
            transitions: TransitionLookup::default(),
        }
    }
}

impl Default for ArcHybridSystem {
    fn default() -> Self {
        ArcHybridSystem::new()
    }
}

impl TransitionSystem for ArcHybridSystem {
    type Transition = ArcHybridTransition;
    type Oracle = ArcHybridOracle;

    const ATTACHMENT_ADDRS: [AttachmentAddr; 2] = [
        AttachmentAddr {
            head: Source::Buffer(0),
            dependent: Source::Stack(0),
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
        ArcHybridOracle::new(gold_dependencies)
    }

    fn transitions(&self) -> &TransitionLookup<Self::Transition> {
        &self.transitions
    }
}

/// Stack-projective transition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ArcHybridTransition {
    LeftArc(String),
    RightArc(String),
    Shift,
}

impl Transition for ArcHybridTransition {
    type S = ArcHybridSystem;

    fn is_possible(&self, state: &ParserState) -> bool {
        match self {
            &ArcHybridTransition::LeftArc(_) => {
                state.stack().len() > 1 && !state.buffer().is_empty()
            }
            &ArcHybridTransition::RightArc(_) => state.stack().len() > 1,
            &ArcHybridTransition::Shift => !state.buffer().is_empty(),
        }
    }

    fn apply(&self, state: &mut ParserState) {
        let stack_len = state.stack().len();

        match self {
            &ArcHybridTransition::LeftArc(ref rel) => {
                let head = state.buffer()[0];
                let dependent = state.stack_mut().remove(stack_len - 1);

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });
            }
            &ArcHybridTransition::RightArc(ref rel) => {
                let dependent = state.stack()[stack_len - 1];
                let head = state.stack()[stack_len - 2];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.stack_mut().pop();
            }
            &ArcHybridTransition::Shift => {
                let next = state.buffer_mut().remove(0);
                state.stack_mut().push(next);
            }
        }
    }
}

pub struct ArcHybridOracle {
    dependencies: HashMap<usize, Dependency>,
}

impl ArcHybridOracle {
    pub fn new(gold_dependencies: &DependencySet) -> ArcHybridOracle {
        ArcHybridOracle {
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

impl Guide for ArcHybridOracle {
    type Transition = ArcHybridTransition;

    fn best_transition(&mut self, state: &ParserState) -> ArcHybridTransition {
        assert!(
            !state.stack().is_empty(),
            "Impossible configuration (empty stack)"
        );
        assert!(
            !ArcHybridSystem::is_terminal(state),
            "Call of best_transition on a terminal configuration"
        );

        let stack = &state.stack();
        let buffer = &state.buffer();

        let stack0 = stack[stack.len() - 1];

        if !buffer.is_empty() {
            let buffer_head = buffer[0];

            if let Some(dep) = self.dependencies.get(&stack0) {
                let la = ArcHybridTransition::LeftArc(dep.relation.clone());

                if dep.head == buffer_head && la.is_possible(state) {
                    return la;
                }
            }
        }

        if stack.len() > 1 {
            let stack1 = stack[stack.len() - 2];

            if let Some(dep) = self.dependencies.get(&stack0) {
                let ra = ArcHybridTransition::RightArc(dep.relation.clone());

                if dep.head == stack1
                    && ra.is_possible(state)
                    && !self.needed_for_attachment(state, stack0)
                {
                    return ra;
                }
            }
        }

        ArcHybridTransition::Shift
    }
}
