pub mod arc_eager;
pub use self::arc_eager::ArcEagerSystem;

pub mod arc_hybrid;
pub use self::arc_hybrid::ArcHybridSystem;

pub mod arc_standard;
pub use self::arc_standard::ArcStandardSystem;

pub mod stack_projective;
pub use self::stack_projective::StackProjectiveSystem;

pub mod stack_swap;
pub use self::stack_swap::StackSwapSystem;

mod util;

#[cfg(test)]
mod system_tests;
