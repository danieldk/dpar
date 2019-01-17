use tensorflow::Tensor;

use features::Layer;
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
        let mut input_lookup_tensors = LayerTensors::new();
        for (layer, size) in self.vectorizer().layer_sizes() {
            input_lookup_tensors[layer] = Tensor::new(&[states.len() as u64, size as u64]).into();
        }

        let n_deprel_embeds = self
            .vectorizer()
            .layer_lookups()
            .layer_lookup(Layer::DepRel)
            .unwrap()
            .len();
        let mut input_non_lookup_tensors = Tensor::new(&[
            states.len() as u64,
            (n_deprel_embeds * T::ATTACHMENT_ADDRS.len()) as u64,
        ]);

        // Fill tensors.
        for (idx, state) in states.iter().enumerate() {
            self.vectorizer().realize_into(
                state,
                &mut input_lookup_tensors.to_instance_slices(idx),
                &mut input_non_lookup_tensors,
                &T::ATTACHMENT_ADDRS,
            );
        }

        self.predict(states, &input_lookup_tensors, &input_non_lookup_tensors)
    }
}
