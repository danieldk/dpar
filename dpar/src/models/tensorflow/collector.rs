use enum_map::EnumMap;
use tensorflow::Tensor;

use features::{InputVectorizer, Layer};
use models::tensorflow::{CopyBatches, InstanceSlices, LayerTensors, TensorWrap};
use system::ParserState;
use system::TransitionSystem;
use train::InstanceCollector;

use Result;

/// Collect gold-standard instances into Tensorflow tensors.
///
/// This data type implements the `Collector` trait, collecting instances
/// into Tensorflow tensors. The collected instances can be used to train
/// or validate a ternsorflow model.
///
/// After all instances are collected, the `into_data` method can be used
/// to get the collected tensors. The last batch will be resized to the
/// number of instances collected into the last batch.
pub struct TensorCollector<T> {
    transition_system: T,
    vectorizer: InputVectorizer,
    batch_size: usize,
    inputs: Vec<LayerTensors<i32>>,
    labels: Vec<Tensor<i32>>,
    instance_idx: usize,
}

impl<T> TensorCollector<T> {
    /// Construct a tensor collector.
    ///
    /// The tensor collector will use the given transition system, parser state
    /// vectorizer, and batch size.
    pub fn new(transition_system: T, vectorizer: InputVectorizer, batch_size: usize) -> Self {
        TensorCollector {
            transition_system,
            vectorizer,
            batch_size,
            inputs: Vec::new(),
            labels: Vec::new(),
            instance_idx: 0,
        }
    }

    /// Resize the last batch to the number of instances in that batch.
    fn resize_last_batch(&mut self) {
        if self.instance_idx == 0 {
            return;
        }

        let last_size = self.instance_idx;

        let old_inputs = self.inputs.pop().expect("No batches");
        self.inputs.push(old_inputs.copy_batches(last_size as u64));

        let old_labels = self.labels.pop().expect("No batches");
        self.labels.push(old_labels.copy_batches(last_size as u64));
    }

    /// Get the collected tensors.
    ///
    /// This method returns the collected label and vectorized parser state
    /// tensors. Each tensor is `batch_size` in its first dimension, except
    /// the last label/layer tensors, which is sized to the number of instances
    /// of the last batch.
    pub fn into_data(mut self) -> (Vec<Tensor<i32>>, Vec<LayerTensors<i32>>) {
        self.resize_last_batch();

        (self.labels, self.inputs)
    }

    /// Get the transition system of the collector.
    pub fn transition_system(&self) -> &T {
        &self.transition_system
    }

    /// Construct net layer batch tensors.
    ///
    /// Each tensor has shape `[batch_size, layer_size]`.
    fn new_layer_tensors(&self, batch_size: usize) -> LayerTensors<i32> {
        let layer_sizes = self.vectorizer.layer_sizes();

        let mut layers: EnumMap<Layer, TensorWrap<i32>> = EnumMap::new();
        for (layer, tensor) in &mut layers {
            *tensor = TensorWrap(Tensor::new(&[batch_size as u64, layer_sizes[layer] as u64]));
        }

        layers
    }
}

impl<T> InstanceCollector<T> for TensorCollector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<()> {
        // Lazily add a new batch tensor.
        if self.instance_idx == 0 {
            let layer_tensors = self.new_layer_tensors(self.batch_size);
            self.inputs.push(layer_tensors);
            self.labels.push(Tensor::new(&[self.batch_size as u64]));
        }

        let batch = self.labels.len() - 1;

        let label = self.transition_system.transitions().lookup(t.clone());
        self.labels[batch][self.instance_idx] = label as i32;

        self.vectorizer.realize_into(
            state,
            &mut self.inputs[batch].to_instance_slices(self.instance_idx),
        );

        self.instance_idx += 1;
        if self.instance_idx == self.batch_size {
            self.instance_idx = 0;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use conllx::{Sentence, Token};

    use features::addr::{AddressedValue, Layer, Source};
    use features::{
        self, AddressedValues, InputVectorizer, LayerLookups, Lookup, MutableLookupTable,
    };
    use system::{ParserState, Transition};
    use systems::stack_projective::{StackProjectiveSystem, StackProjectiveTransition};
    use train::InstanceCollector;

    use super::TensorCollector;

    #[test]
    fn collect_zero() {
        let collector = test_collector();
        let (labels, inputs) = collector.into_data();
        assert_eq!(labels.len(), 0);
        assert_eq!(inputs.len(), 0);
    }

    #[test]
    fn collect_two() {
        let sent = Sentence::new(vec![Token::new("een"), Token::new("test")]);
        let mut state = ParserState::new(&sent);

        let mut collector = test_collector();
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::LeftArc("FOO".into()), &state)
            .unwrap();
        let (labels, inputs) = collector.into_data();

        // There should be one batch.
        assert_eq!(labels.len(), 1);
        assert_eq!(inputs.len(), 1);

        // Check batch shapes.
        assert_eq!(labels[0].dims(), &[2]);
        assert_eq!(inputs[0][features::Layer::Token].dims(), &[2, 2]);

        // Check batch contents.
        assert_eq!(&*labels[0], &[1, 2]);
        assert_eq!(inputs[0][features::Layer::Token].as_ref(), &[1, 2, 2, 3]);
    }

    #[test]
    fn collect_three() {
        let sent = Sentence::new(vec![
            Token::new("een"),
            Token::new("collector"),
            Token::new("test"),
        ]);
        let mut state = ParserState::new(&sent);

        let mut collector = test_collector();
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::LeftArc("FOO".into()), &state)
            .unwrap();
        let (labels, inputs) = collector.into_data();

        // There should be two batches.
        assert_eq!(labels.len(), 2);
        assert_eq!(inputs.len(), 2);

        // Check batch shapes.
        assert_eq!(labels[0].dims(), &[2]);
        assert_eq!(inputs[0][features::Layer::Token].dims(), &[2, 2]);
        assert_eq!(labels[1].dims(), &[1]);
        assert_eq!(inputs[1][features::Layer::Token].dims(), &[1, 2]);

        // Check batch contents.
        assert_eq!(&*labels[0], &[1, 1]);
        assert_eq!(inputs[0][features::Layer::Token].as_ref(), &[1, 2, 2, 3]);
        assert_eq!(&*labels[1], &[2]);
        assert_eq!(inputs[1][features::Layer::Token].as_ref(), &[3, 4]);
    }

    fn test_collector() -> TensorCollector<StackProjectiveSystem> {
        let stack0 = AddressedValue {
            address: vec![Source::Stack(0)],
            layer: Layer::Token,
        };

        let buffer0 = AddressedValue {
            address: vec![Source::Buffer(0)],
            layer: Layer::Token,
        };

        let mut lookups = LayerLookups::new();
        let table: Box<Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::Token, table);

        let vectorizer = InputVectorizer::new(lookups, AddressedValues(vec![stack0, buffer0]));

        TensorCollector::new(StackProjectiveSystem::new(), vectorizer, 2)
    }
}
