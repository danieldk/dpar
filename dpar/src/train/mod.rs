use failure::Error;

use crate::system::{ParserState, TransitionSystem};

mod trainer;
pub use self::trainer::GreedyTrainer;

mod noop;
pub use self::noop::NoopCollector;

pub trait InstanceCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState<'_>) -> Result<(), Error>;
}
