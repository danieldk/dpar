use failure::Error;

use features::InputVectorizer;
use system::{ParserState, TransitionSystem};
use train::InstanceCollector;

/// No-op collector.
///
/// The no-op collector implements the `InstanceCollector` trait, but
/// does not actually collect any data. However, since this collector
/// vectorizes parser states, it can be used for initializing lookup
/// tables (as a side-effect of collection).
pub struct NoopCollector<T> {
    transition_system: T,
    vectorizer: InputVectorizer,
}

impl<T> NoopCollector<T>
where
    T: TransitionSystem,
{
    /// Create a new `NoopCollector`.
    pub fn new(transition_system: T, vectorizer: InputVectorizer) -> Result<Self, Error> {
        Ok(NoopCollector {
            transition_system: transition_system,
            vectorizer: vectorizer,
        })
    }

    /// Get the vectorizer that is associated with the collector.
    pub fn input_vectorizer(&self) -> &InputVectorizer {
        &self.vectorizer
    }

    /// Get the transition system that is associated with the collector.
    pub fn transition_system(&self) -> &T {
        &self.transition_system
    }
}

impl<T> InstanceCollector<T> for NoopCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<(), Error> {
        self.transition_system.transitions().lookup(t.clone());
        self.vectorizer.realize(state, &T::ATTACHMENT_ADDRS);
        Ok(())
    }
}
