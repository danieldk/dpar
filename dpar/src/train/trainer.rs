use std::marker::PhantomData;

use failure::Error;

use guide::Guide;
use system::{DependencySet, ParserState, Transition, TransitionSystem};
use train::InstanceCollector;

pub struct GreedyTrainer<T, C>
where
    C: InstanceCollector<T>,
    T: TransitionSystem,
{
    collector: C,
    transition_type: PhantomData<T>,
}

impl<T, C> GreedyTrainer<T, C>
where
    C: InstanceCollector<T>,
    T: TransitionSystem,
{
    pub fn new(collector: C) -> Self {
        GreedyTrainer {
            collector: collector,
            transition_type: PhantomData,
        }
    }

    pub fn collector(&self) -> &C {
        &self.collector
    }

    pub fn into_collector(self) -> C {
        self.collector
    }

    pub fn parse_state(
        &mut self,
        gold_dependencies: &DependencySet,
        state: &mut ParserState,
    ) -> Result<(), Error> {
        let mut oracle = T::oracle(gold_dependencies);

        while !T::is_terminal(state) {
            let next_transition = oracle.best_transition(state);
            self.collector.collect(&next_transition, state)?;
            next_transition.apply(state);
        }

        Ok(())
    }
}
