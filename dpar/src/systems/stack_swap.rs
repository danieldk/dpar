use std::collections::HashMap;

use petgraph::graph::node_index;
use petgraph::visit::Dfs;
use petgraph::{Directed, Graph};

use guide::Guide;
use system::{
    Dependency, DependencySet, ParserState, Transition, TransitionLookup, TransitionSystem,
};

use features::addr::Source;
use system::AttachmentAddr;
use systems::util::dep_head_mapping;

/// The stack-swap transition system for non-projective parsing.
///
/// The stack-swap transition-system is similar to the stack-projective system,
/// but adds a swap transition that allows it to produce non-projective trees.
///
/// This system is described in:
///
/// Joakim Nivre, Non-projective dependency parsing in expected linear time, 2009
#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct StackSwapSystem {
    transitions: TransitionLookup<StackSwapTransition>,
}

impl StackSwapSystem {
    pub fn new() -> Self {
        StackSwapSystem {
            transitions: TransitionLookup::default(),
        }
    }
}

impl Default for StackSwapSystem {
    fn default() -> Self {
        StackSwapSystem::new()
    }
}

impl TransitionSystem for StackSwapSystem {
    type Transition = StackSwapTransition;
    type Oracle = StackSwapOracle;

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
        StackSwapOracle::new(gold_dependencies)
    }

    fn transitions(&self) -> &TransitionLookup<Self::Transition> {
        &self.transitions
    }
}

/// Stack-projective transition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum StackSwapTransition {
    LeftArc(String),
    RightArc(String),
    Shift,
    Swap,
}

impl Transition for StackSwapTransition {
    type S = StackSwapSystem;

    fn is_possible(&self, state: &ParserState) -> bool {
        let stack_len = state.stack().len();

        match self {
            &StackSwapTransition::LeftArc(_) => stack_len > 2,
            &StackSwapTransition::RightArc(_) => stack_len > 1,
            &StackSwapTransition::Shift => !state.buffer().is_empty(),
            &StackSwapTransition::Swap => {
                stack_len > 2 && state.stack()[stack_len - 1] > state.stack()[stack_len - 2]
            }
        }
    }

    fn apply(&self, state: &mut ParserState) {
        let stack_len = state.stack().len();

        match self {
            &StackSwapTransition::LeftArc(ref rel) => {
                let head = state.stack()[stack_len - 1];
                let dependent = state.stack_mut().remove(stack_len - 2);

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });
            }
            &StackSwapTransition::RightArc(ref rel) => {
                let head = state.stack()[stack_len - 2];
                let dependent = state.stack_mut().remove(stack_len - 1);

                state.add_dependency(Dependency {
                    head: head,
                    relation: rel.clone(),
                    dependent: dependent,
                });
            }
            &StackSwapTransition::Shift => {
                let next = state.buffer_mut().remove(0);
                state.stack_mut().push(next);
            }
            &StackSwapTransition::Swap => {
                let swap_token = state.stack_mut().remove(stack_len - 2);
                state.buffer_mut().insert(0, swap_token);
            }
        }
    }
}

pub struct StackSwapOracle {
    dependencies: HashMap<usize, Dependency>,
    projective_order: Vec<usize>,
}

impl StackSwapOracle {
    pub fn new(gold_dependencies: &DependencySet) -> StackSwapOracle {
        StackSwapOracle {
            dependencies: dep_head_mapping(gold_dependencies),
            projective_order: extract_projective_order(gold_dependencies),
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

        for stack_token in state.stack() {
            if let Some(dep) = self.dependencies.get(&stack_token) {
                if dep.head == token {
                    return true;
                }
            }
        }

        false
    }
}

impl Guide for StackSwapOracle {
    type Transition = StackSwapTransition;

    fn best_transition(&mut self, state: &ParserState) -> StackSwapTransition {
        let stack = &state.stack();
        let stack_len = stack.len();

        if stack_len > 1 {
            let stack0 = stack[stack_len - 1];
            let stack1 = stack[stack_len - 2];

            if let Some(dep) = self.dependencies.get(&stack1) {
                let la = StackSwapTransition::LeftArc(dep.relation.clone());

                if dep.head == stack0
                    && la.is_possible(state)
                    && !self.needed_for_attachment(state, stack1)
                {
                    return la;
                }
            }

            if let Some(dep) = self.dependencies.get(&stack0) {
                let ra = StackSwapTransition::RightArc(dep.relation.clone());

                if dep.head == stack1
                    && ra.is_possible(state)
                    && !self.needed_for_attachment(state, stack0)
                {
                    return ra;
                }
            }

            if self.projective_order[stack0] < self.projective_order[stack1] {
                return StackSwapTransition::Swap;
            }
        }

        StackSwapTransition::Shift
    }
}

fn dependencies_to_graph(dependencies: &DependencySet) -> Graph<(), (), Directed> {
    let edges = dependencies.iter().map(|dependency| {
        (
            node_index(dependency.head),
            node_index(dependency.dependent),
        )
    });

    Graph::from_edges(edges)
}

fn extract_projective_order(dependencies: &DependencySet) -> Vec<usize> {
    // First get the tokens in projective order.
    let mut projective_order = Vec::with_capacity(dependencies.len());
    let dep_graph = dependencies_to_graph(dependencies);
    let mut dfs = Dfs::new(&dep_graph, node_index(0));

    while let Some(node) = dfs.next(&dep_graph) {
        projective_order.push(node);
    }

    // Now create an array that specifies for each token its 'rank'
    // in the projective order. Fixme: there is the assumption here
    // that every token occurs in the dependency graph.
    let mut order_mapping = vec![0; projective_order.len()];
    for (idx, token) in projective_order.iter().enumerate() {
        order_mapping[token.index()] = idx
    }

    order_mapping
}
