use system::{ParserState, TransitionSystem};
use Result;

mod hdf5;
pub use self::hdf5::HDF5Collector;

mod trainer;
pub use self::trainer::GreedyTrainer;

pub trait InstanceCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::T, state: &ParserState) -> Result<()>;
}
