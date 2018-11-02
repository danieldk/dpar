use failure::Error;

use system::{ParserState, TransitionSystem};

mod trainer;
pub use self::trainer::GreedyTrainer;

mod noop;
pub use self::noop::NoopCollector;

pub trait InstanceCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<(), Error>;
}
