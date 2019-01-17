use enum_map::EnumMap;
use tensorflow::Tensor;

use failure::Error;
use features::{InputVectorizer, Layer};
use models::tensorflow::{CopyBatches, InstanceSlices, LayerTensors, TensorWrap};
use system::ParserState;
use system::TransitionSystem;
use train::InstanceCollector;

/// Collect gold-standard instances into Tensorflow tensors.
///
/// This data type implements the `Collector` trait, collecting instances
/// into Tensorflow tensors. The collected instances can be used to train
/// or validate a ternsorflow model.
///
/// After all instances are collected, the `into_data` method can be used
/// to get the collected tensors. The last batch will be resized to the
/// number of instances collected into the last batch.
pub struct TensorCollector<'a, T> {
    transition_system: T,
    vectorizer: &'a InputVectorizer,
    batch_size: usize,
    lookup_inputs: Vec<LayerTensors<i32>>,
    labels: Vec<Tensor<i32>>,
    non_lookup_inputs: Vec<Tensor<f32>>,
    instance_idx: usize,
}

impl<'a, T> TensorCollector<'a, T>
where
    T: TransitionSystem,
{
    /// Construct a tensor collector.
    ///
    /// The tensor collector will use the given transition system, parser state
    /// vectorizer, and batch size.
    pub fn new(transition_system: T, vectorizer: &'a InputVectorizer, batch_size: usize) -> Self {
        TensorCollector {
            transition_system,
            vectorizer,
            batch_size,
            lookup_inputs: Vec::new(),
            labels: Vec::new(),
            non_lookup_inputs: Vec::new(),
            instance_idx: 0,
        }
    }

    /// Resize the last batch to the number of instances in that batch.
    fn resize_last_batch(&mut self) {
        if self.instance_idx == 0 {
            return;
        }

        let last_size = self.instance_idx;

        let old_lookup_inputs = self.lookup_inputs.pop().expect("No batches");
        self.lookup_inputs
            .push(old_lookup_inputs.copy_batches(last_size as u64));

        let old_labels = self.labels.pop().expect("No batches");
        self.labels.push(old_labels.copy_batches(last_size as u64));

        let old_non_lookup_inputs = self.non_lookup_inputs.pop().expect("No batches");
        self.non_lookup_inputs
            .push(old_non_lookup_inputs.copy_batches(last_size as u64));
    }

    /// Get the collected tensors.
    ///
    /// This method returns the collected label and vectorized parser state
    /// tensors. Each tensor is `batch_size` in its first dimension, except
    /// the last label/layer tensors, which is sized to the number of instances
    /// of the last batch.
    pub fn into_data(mut self) -> (Vec<Tensor<i32>>, Vec<LayerTensors<i32>>, Vec<Tensor<f32>>) {
        self.resize_last_batch();

        (self.labels, self.lookup_inputs, self.non_lookup_inputs)
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

impl<'a, T> InstanceCollector<T> for TensorCollector<'a, T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState) -> Result<(), Error> {
        // Lazily add a new batch tensor.

        let n_deprel_embeds = self
            .vectorizer
            .layer_lookups()
            .layer_lookup(Layer::DepRel)
            .unwrap()
            .len();

        if self.instance_idx == 0 {
            let layer_tensors = self.new_layer_tensors(self.batch_size);
            self.lookup_inputs.push(layer_tensors);
            self.labels.push(Tensor::new(&[self.batch_size as u64]));
            self.non_lookup_inputs.push(Tensor::new(&[
                self.batch_size as u64,
                (n_deprel_embeds * T::ATTACHMENT_ADDRS.len()) as u64,
            ]));
        }

        let batch = self.labels.len() - 1;

        let label = self.transition_system.transitions().lookup(t.clone());
        self.labels[batch][self.instance_idx] = label as i32;

        let n_non_lookup_inputs = n_deprel_embeds * T::ATTACHMENT_ADDRS.len();
        self.vectorizer.realize_into(
            state,
            &mut self.lookup_inputs[batch].to_instance_slices(self.instance_idx),
            &mut self.non_lookup_inputs[batch][(self.instance_idx * n_non_lookup_inputs)
                                                   ..(self.instance_idx * n_non_lookup_inputs
                                                       + n_non_lookup_inputs)],
            &T::ATTACHMENT_ADDRS,
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
    use std::collections::HashMap;

    use conllx::Token;

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
        let vectorizer = test_vectorizer();
        let collector = test_collector(&vectorizer);
        let (labels, lookup_inputs, non_lookup_inputs) = collector.into_data();
        assert_eq!(labels.len(), 0);
        assert_eq!(lookup_inputs.len(), 0);
        assert_eq!(non_lookup_inputs.len(), 0);
    }

    #[test]
    fn collect_two() {
        let sent = vec![Token::new("een"), Token::new("test")];
        let mut state = ParserState::new(&sent);

        let vectorizer = test_vectorizer();
        let mut collector = test_collector(&vectorizer);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::LeftArc("FOO".into()), &state)
            .unwrap();
        let (labels, lookup_inputs, non_lookup_inputs) = collector.into_data();

        // There should be one batch.
        assert_eq!(labels.len(), 1);
        assert_eq!(lookup_inputs.len(), 1);
        assert_eq!(non_lookup_inputs.len(), 1);

        // Check batch shapes.
        assert_eq!(labels[0].dims(), &[2]);
        assert_eq!(lookup_inputs[0][features::Layer::Token].dims(), &[2, 2]);
        assert_eq!(non_lookup_inputs[0].dims(), &[2, 2]);
        assert_eq!(lookup_inputs[0][features::Layer::DepRel].dims(), &[2, 0]);

        // Check batch contents.
        assert_eq!(&*labels[0], &[1, 2]);
        assert_eq!(
            lookup_inputs[0][features::Layer::Token].as_ref(),
            &[1, 2, 2, 3]
        );
        assert_eq!(&*non_lookup_inputs[0], &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn collect_three() {
        let sent = vec![
            Token::new("een"),
            Token::new("collector"),
            Token::new("test"),
        ];
        let mut state = ParserState::new(&sent);

        let vectorizer = test_vectorizer();
        let mut collector = test_collector(&vectorizer);
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
        let (labels, lookup_inputs, non_lookup_inputs) = collector.into_data();

        // There should be two batches.
        assert_eq!(labels.len(), 2);
        assert_eq!(lookup_inputs.len(), 2);
        assert_eq!(non_lookup_inputs.len(), 2);

        // Check batch shapes.
        assert_eq!(labels[0].dims(), &[2]);
        assert_eq!(lookup_inputs[0][features::Layer::Token].dims(), &[2, 2]);
        assert_eq!(non_lookup_inputs[0].dims(), &[2, 2]);
        assert_eq!(labels[1].dims(), &[1]);
        assert_eq!(lookup_inputs[1][features::Layer::Token].dims(), &[1, 2]);
        assert_eq!(non_lookup_inputs[1].dims(), &[1, 2]);

        // Check batch contents.
        assert_eq!(&*labels[0], &[1, 1]);
        assert_eq!(
            lookup_inputs[0][features::Layer::Token].as_ref(),
            &[1, 2, 2, 3]
        );
        assert_eq!(&*non_lookup_inputs[0], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&*labels[1], &[2]);
        assert_eq!(lookup_inputs[1][features::Layer::Token].as_ref(), &[3, 4]);
        assert_eq!(&*non_lookup_inputs[1], &[0.0, 0.0]);
    }

    #[test]
    fn collect_one_pmi() {
        let sent = vec![
            Token::new("een"),
            Token::new("collector"),
            Token::new("test"),
        ];
        let mut state = ParserState::new(&sent);

        let pmi_vectorizer = test_pmi_vectorizer();
        let mut collector = test_collector(&pmi_vectorizer);
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
        StackProjectiveTransition::LeftArc("FOO".into()).apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        let (_labels, _lookup_inputs, non_lookup_inputs) = collector.into_data();

        // There should be two batches.
        assert_eq!(non_lookup_inputs.len(), 2);

        // Check batch shapes.
        assert_eq!(non_lookup_inputs[0].dims(), &[2, 2]);
        assert_eq!(non_lookup_inputs[1].dims(), &[2, 2]);

        // Check batch contents.
        assert_eq!(&*non_lookup_inputs[0], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&*non_lookup_inputs[1], &[0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn collect_two_pmis() {
        let sent = vec![
            Token::new("een"),
            Token::new("collector"),
            Token::new("test"),
        ];
        let mut state = ParserState::new(&sent);

        let pmi_vectorizer = test_pmi_vectorizer();
        let mut collector = test_collector(&pmi_vectorizer);
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
        StackProjectiveTransition::LeftArc("FOO".into()).apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::LeftArc("BAR".into()), &state)
            .unwrap();
        StackProjectiveTransition::LeftArc("BAR".into()).apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        let (_labels, _lookup_inputs, non_lookup_inputs) = collector.into_data();

        // There should be two batches.
        assert_eq!(non_lookup_inputs.len(), 3);

        // Check batch shapes.
        assert_eq!(non_lookup_inputs[0].dims(), &[2, 2]);
        assert_eq!(non_lookup_inputs[1].dims(), &[2, 2]);
        assert_eq!(non_lookup_inputs[2].dims(), &[2, 4]);

        // Check batch contents.
        assert_eq!(&*non_lookup_inputs[0], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&*non_lookup_inputs[1], &[0.0, 0.0, 1.0, 1.0]);
        assert_eq!(
            &*non_lookup_inputs[2],
            &[0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]
        );
    }

    fn test_vectorizer() -> InputVectorizer {
        let stack0 = AddressedValue {
            address: vec![Source::Stack(0)],
            layer: Layer::Token,
        };

        let buffer0 = AddressedValue {
            address: vec![Source::Buffer(0)],
            layer: Layer::Token,
        };

        let mut lookups = LayerLookups::new();
        let token_table: Box<Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::Token, token_table);
        let deprel_table: Box<Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::DepRel, deprel_table);

        let association_strengths = HashMap::new();

        InputVectorizer::new(
            lookups,
            AddressedValues(vec![stack0, buffer0]),
            association_strengths,
        )
    }

    fn test_pmi_vectorizer() -> InputVectorizer {
        let stack0 = AddressedValue {
            address: vec![Source::Stack(0)],
            layer: Layer::Token,
        };

        let stack1 = AddressedValue {
            address: vec![Source::Stack(1)],
            layer: Layer::Token,
        };

        let stack0_ldep0 = AddressedValue {
            address: vec![Source::Stack(0), Source::LDep(0)],
            layer: Layer::DepRel,
        };

        let stack1_rdep0 = AddressedValue {
            address: vec![Source::Stack(1), Source::RDep(0)],
            layer: Layer::DepRel,
        };

        let mut lookups = LayerLookups::new();
        let token_table: Box<Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::Token, token_table);
        let deprel_table: Box<Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::DepRel, deprel_table);

        let mut association_strengths = HashMap::new();
        association_strengths.insert(
            (
                "collector".to_string(),
                "ROOT".to_string(),
                "FOO".to_string(),
            ),
            1.0,
        );
        association_strengths.insert(
            (
                "ROOT".to_string(),
                "collector".to_string(),
                "FOO".to_string(),
            ),
            1.0,
        );
        association_strengths.insert(
            ("ROOT".to_string(), "test".to_string(), "FOO".to_string()),
            1.0,
        );
        association_strengths.insert(
            ("test".to_string(), "ROOT".to_string(), "FOO".to_string()),
            1.0,
        );
        association_strengths.insert(
            ("ROOT".to_string(), "test".to_string(), "BAR".to_string()),
            1.0,
        );
        association_strengths.insert(
            ("test".to_string(), "ROOT".to_string(), "BAR".to_string()),
            1.0,
        );

        InputVectorizer::new(
            lookups,
            AddressedValues(vec![stack0, stack1, stack0_ldep0, stack1_rdep0]),
            association_strengths,
        )
    }

    fn test_collector(vectorizer: &InputVectorizer) -> TensorCollector<StackProjectiveSystem> {
        TensorCollector::new(StackProjectiveSystem::new(), vectorizer, 2)
    }
}
