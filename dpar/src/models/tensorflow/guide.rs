use enum_map::EnumMap;
use tensorflow::Tensor;

use system::{ParserState, TransitionSystem};

use guide::{BatchGuide, Guide};

use super::{LayerTensors, TensorflowModel};

impl<T> Guide for TensorflowModel<T>
where
    T: TransitionSystem,
{
    type Transition = T::Transition;

    fn best_transition(&mut self, state: &ParserState) -> Self::Transition {
        self.best_transitions(&[state]).remove(0)
    }
}

impl<T> BatchGuide for TensorflowModel<T>
where
    T: TransitionSystem,
{
    type Transition = T::Transition;

    fn best_transitions(&mut self, states: &[&ParserState]) -> Vec<Self::Transition> {
        if states.is_empty() {
            return Vec::new();
        }

        // Allocate batch tensors.
        let mut input_tensors = LayerTensors(EnumMap::new());
        for (layer, size) in self.vectorizer().layer_sizes() {
            input_tensors[layer] = Tensor::new(&[states.len() as u64, size as u64]).into();
        }

        // Fill tensors.
        for (idx, state) in states.iter().enumerate() {
            self.vectorizer()
                .realize_into(state, &mut input_tensors.to_instance_slices(idx));
        }

        self.predict(states, &input_tensors)
    }
}
