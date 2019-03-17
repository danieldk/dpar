use tensorflow::Tensor;

use guide::{BatchGuide, Guide};
use models::tensorflow::{InstanceSlices, LayerTensors, TensorflowModel};
use system::{ParserState, TransitionSystem};

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
        let embed_size = self.vectorizer().embedding_layer_size();
        let mut embed_tensors = Tensor::new(&[states.len() as u64, embed_size as u64]);

        let mut input_tensors = LayerTensors::new();
        for (layer, size) in self.vectorizer().lookup_layer_sizes() {
            input_tensors[layer] = Tensor::new(&[states.len() as u64, size as u64]).into();
        }

        // Fill tensors.
        for (idx, state) in states.iter().enumerate() {
            let embed_offset = embed_size * idx;
            self.vectorizer().realize_into(
                state,
                &mut embed_tensors[embed_offset..embed_offset + embed_size],
                &mut input_tensors.to_instance_slices(idx),
            );
        }

        self.predict(states, &embed_tensors, &input_tensors)
    }
}
