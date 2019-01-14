use std::collections::HashMap;

use guide::Guide;
use system::{
    Dependency, DependencySet, ParserState, Transition, TransitionLookup, TransitionSystem,
};

use features::addr::Source;
use system::AttachmentAddr;
use systems::util::dep_head_mapping;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct ArcEagerSystem {
    transitions: TransitionLookup<ArcEagerTransition>,
}

impl ArcEagerSystem {
    pub fn new() -> Self {
        ArcEagerSystem {
            transitions: TransitionLookup::default(),
        }
    }
}

impl Default for ArcEagerSystem {
    fn default() -> Self {
        ArcEagerSystem::new()
    }
}

/// The ArcEager transition system.
///
/// See: Joakim Nivre, Incrementality in Deterministic Dependency Parsing, 2004
impl TransitionSystem for ArcEagerSystem {
    type Transition = ArcEagerTransition;
    type Oracle = ArcEagerOracle;

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
        ArcEagerOracle::new(gold_dependencies)
    }

    fn transitions(&self) -> &TransitionLookup<Self::Transition> {
        &self.transitions
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ArcEagerTransition {
    LeftArc(String),
    RightArc(String),
    Shift,
    Reduce,
}

impl Transition for ArcEagerTransition {
    type S = ArcEagerSystem;

    fn is_possible(&self, state: &ParserState) -> bool {
        let stack_len = state.stack().len();
        let buffer_len = state.buffer().len();

        match self {
            &ArcEagerTransition::LeftArc(_) => {
                stack_len > 1
                    && buffer_len > 0
                    && state.head(state.stack()[stack_len - 1]).is_none()
            }
            &ArcEagerTransition::RightArc(_) => stack_len > 0 && buffer_len > 0,
            &ArcEagerTransition::Shift => buffer_len > 0,
            &ArcEagerTransition::Reduce => {
                stack_len > 0 && state.head(state.stack()[stack_len - 1]).is_some()
            }
        }
    }

    fn apply(&self, state: &mut ParserState) {
        let stack_len = state.stack().len();

        match self {
            &ArcEagerTransition::LeftArc(ref rel) => {
                let head = state.buffer()[0];
                let dependent = state.stack_mut().remove(stack_len - 1);

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });
            }
            &ArcEagerTransition::RightArc(ref rel) => {
                let dependent = state.buffer_mut().remove(0);
                let head = state.stack()[stack_len - 1];

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });

                state.stack_mut().push(dependent);
            }
            &ArcEagerTransition::Shift => {
                let next = state.buffer_mut().remove(0);
                state.stack_mut().push(next);
            }
            &ArcEagerTransition::Reduce => {
                state.stack_mut().pop();
            }
        }
    }
}

pub struct ArcEagerOracle {
    dependencies: HashMap<usize, Dependency>,
}

impl ArcEagerOracle {
    pub fn new(gold_dependencies: &DependencySet) -> ArcEagerOracle {
        ArcEagerOracle {
            dependencies: dep_head_mapping(gold_dependencies),
        }
    }

    fn next_attached(&self, state: &ParserState) -> bool {
        let stack_tip = state.stack()[state.stack().len() - 1];
        let buffer_head = state.buffer()[0];

        for (dependent, relation) in &self.dependencies {
            if *dependent == buffer_head && relation.head < stack_tip {
                return true;
            }

            if *dependent < stack_tip && relation.head == buffer_head {
                return true;
            }
        }

        return false;
    }
}

impl Guide for ArcEagerOracle {
    type Transition = ArcEagerTransition;

    fn best_transition(&mut self, state: &ParserState) -> ArcEagerTransition {
        assert!(
            !state.buffer().is_empty(),
            "Applying oracle to terminal configuration"
        );

        let stack = &state.stack();
        let buffer = &state.buffer();
        let stack_len = stack.len();

        if stack_len > 0 {
            let stack_tip = stack[stack.len() - 1];
            let buffer_head = buffer[0];

            // Left-Arc or Right-Arc is the next transition if:
            //
            // - The transition itself is possible.
            // - Following the transition would introduce a dependency
            //   that corresponds to the gold standard.
            if let Some(dep) = self.dependencies.get(&stack_tip) {
                let la = ArcEagerTransition::LeftArc(dep.relation.clone());

                if dep.head == buffer_head && la.is_possible(state) {
                    return la;
                }
            }

            if let Some(dep) = self.dependencies.get(&buffer_head) {
                let ra = ArcEagerTransition::RightArc(dep.relation.clone());

                if dep.head == stack_tip && ra.is_possible(state) {
                    return ra;
                }
            }

            // reduce is the next transition if:
            //
            // - reduce is possible.
            // - The next token in the buffer is in a dependency relation with a token that precedes
            //   the token on the tip of the stack.
            let r = ArcEagerTransition::Reduce;
            if r.is_possible(state) && self.next_attached(state) {
                return r;
            }
        }

        ArcEagerTransition::Shift
    }
}
