use system::{ParserState, TransitionSystem};
use Result;

mod trainer;
pub use self::trainer::GreedyTrainer;

pub trait InstanceCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<()>;
}
