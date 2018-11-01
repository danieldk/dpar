use system::{ParserState, TransitionSystem};
use Result;

mod trainer;
pub use self::trainer::GreedyTrainer;

mod noop;
pub use self::noop::NoopCollector;

pub trait InstanceCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<()>;
}
