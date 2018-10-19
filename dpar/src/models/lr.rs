//! Learning rate functions.

/// Trait for learning rate schedules.
///
/// A learning rate schedule determines the learning rate
/// at a given epoch.
pub trait LearningRateSchedule {
    /// Compute the learning rate for an epoch.
    fn learning_rate(&self, epoch: usize) -> f32;
}

/// Constant learning rate schedule.
///
/// This schedule uses the same learning rate for every epoch.
pub struct ConstantLearningRate(f32);

impl ConstantLearningRate {
    /// Construct a constant learning reate.
    pub fn new(lr: f32) -> Self {
        assert!(lr > 0.0, "Learning rate must be a positive value");

        ConstantLearningRate(lr)
    }
}

impl LearningRateSchedule for ConstantLearningRate {
    fn learning_rate(&self, _epoch: usize) -> f32 {
        self.0
    }
}

/// Exponential decay learning rate schedule.
///
/// This schedule starts at an initial learning rate, which decays
/// exponentionally over time. To be specific, the learning rate is
/// calculated as follows:
///
/// *lr = initial_lr * decay_rate ^ (epoch / decay_steps)*
pub struct ExponentialDecay {
    initial_lr: f32,
    decay_rate: f32,
    decay_steps: usize,
    staircase: bool,
}

impl ExponentialDecay {
    /// Construct an exponential decay schedule.
    ///
    /// If `staircase` is true, the exponent of the decay is
    /// computed using integer division. This has the effect that
    /// the learning rate only changes every `decay_steps` steps.
    pub fn new(initial_lr: f32, decay_rate: f32, decay_steps: usize, staircase: bool) -> Self {
        assert!(
            initial_lr > 0.0,
            "The initial learning rate must be a positive value."
        );
        assert!(
            decay_rate > 0.0 && decay_rate < 1.0,
            "The decay rate must be in (0, 1)."
        );
        assert!(
            decay_steps > 0,
            "The number decay steps should be non-zero."
        );

        ExponentialDecay {
            initial_lr,
            decay_rate,
            decay_steps,
            staircase,
        }
    }
}

impl LearningRateSchedule for ExponentialDecay {
    fn learning_rate(&self, epoch: usize) -> f32 {
        let exponent = if self.staircase {
            (epoch / self.decay_steps) as f32
        } else {
            epoch as f32 / self.decay_steps as f32
        };

        self.initial_lr * self.decay_rate.powf(exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstantLearningRate, ExponentialDecay, LearningRateSchedule};

    #[test]
    pub fn constant_lr() {
        let constant = ConstantLearningRate(0.1);
        assert_relative_eq!(constant.learning_rate(0), 0.1);
        assert_relative_eq!(constant.learning_rate(1), 0.1);
        assert_relative_eq!(constant.learning_rate(5), 0.1);
        assert_relative_eq!(constant.learning_rate(15), 0.1);
        assert_relative_eq!(constant.learning_rate(25), 0.1);
    }

    #[test]
    pub fn exponential_decay_lr() {
        let decay1 = ExponentialDecay::new(0.1, 0.2, 10, true);
        assert_relative_eq!(decay1.learning_rate(0), 0.1);
        assert_relative_eq!(decay1.learning_rate(1), 0.1);
        assert_relative_eq!(decay1.learning_rate(5), 0.1);
        assert_relative_eq!(decay1.learning_rate(15), 0.02);
        assert_relative_eq!(decay1.learning_rate(25), 0.004);

        let decay2 = ExponentialDecay::new(0.1, 0.2, 10, false);
        assert_relative_eq!(decay2.learning_rate(0), 0.1);
        assert_relative_eq!(decay2.learning_rate(1), 0.085133992);
        assert_relative_eq!(decay2.learning_rate(5), 0.044721359);
        assert_relative_eq!(decay2.learning_rate(15), 0.008944271);
        assert_relative_eq!(decay2.learning_rate(25), 0.001788854);
    }
}
