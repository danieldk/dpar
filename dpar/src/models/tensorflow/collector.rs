use enum_map::EnumMap;
use tensorflow::Tensor;

use crate::features::Layer;
use crate::models::tensorflow::{
    InstanceSlices, LayerTensors, ShrinkBatch, TensorWrap, TensorflowModel,
};
use crate::models::ModelPerformance;
use crate::system::ParserState;
use crate::system::TransitionSystem;
use crate::train::InstanceCollector;
use failure::Error;

/// Collect gold-standard instances into Tensorflow tensors.
///
/// This data type implements the `Collector` trait, collecting instances
/// into Tensorflow tensors. The collected instances can be used to train
/// or validate a ternsorflow model.
///
/// After all instances are collected, the `into_data` method can be used
/// to get the collected tensors. The last batch will be resized to the
/// number of instances collected into the last batch.
pub struct TrainCollector<'a, T>
where
    T: TransitionSystem,
{
    model: &'a mut TensorflowModel<T>,
    batch_size: usize,
    embeds: Tensor<f32>,
    inputs: LayerTensors<i32>,
    labels: Tensor<i32>,
    instance_idx: usize,
    is_training: bool,
    lr: f32,
    performance: ModelPerformance,
    n_instances: usize,
}

impl<'a, T> TrainCollector<'a, T>
where
    T: TransitionSystem,
{
    /// Construct a tensor collector.
    ///
    /// The tensor collector will use the given transition system, parser state
    /// vectorizer, and batch size.
    pub fn new(
        model: &'a mut TensorflowModel<T>,
        batch_size: usize,
        is_training: bool,
        lr: f32,
    ) -> Self {
        let embeds = Tensor::new(&[
            batch_size as u64,
            model.vectorizer().embedding_layer_size() as u64,
        ]);

        let layer_sizes = model.vectorizer().lookup_layer_sizes();

        let mut inputs: EnumMap<Layer, TensorWrap<i32>> = EnumMap::new();
        for (layer, tensor) in &mut inputs {
            *tensor = TensorWrap(Tensor::new(&[batch_size as u64, layer_sizes[layer] as u64]));
        }

        TrainCollector {
            model,
            batch_size,
            embeds,
            inputs,
            labels: Tensor::new(&[batch_size as u64]),
            instance_idx: 0,
            is_training,
            lr,
            performance: Default::default(),
            n_instances: 0,
        }
    }

    pub fn clear(&mut self) {
        self.instance_idx = 0;
    }

    pub fn embeds(&self) -> &Tensor<f32> {
        &self.embeds
    }

    pub fn inputs(&self) -> &LayerTensors<i32> {
        &self.inputs
    }

    pub fn is_empty(&self) -> bool {
        self.instance_idx == 0
    }

    pub fn is_full(&self) -> bool {
        self.instance_idx == self.batch_size
    }

    pub fn labels(&self) -> &Tensor<i32> {
        &self.labels
    }

    /// Number of instances in the collector.
    pub fn len(&self) -> usize {
        self.instance_idx
    }

    /// Process the last batch and return the model performance.
    pub fn finish(mut self) -> ModelPerformance {
        self.resize_last_batch();
        self.train_batch();

        let mut perf = self.performance;
        perf.loss /= self.n_instances as f32;
        perf.accuracy /= self.n_instances as f32;
        perf
    }

    /// Resize the last batch to the number of instances in that batch.
    fn resize_last_batch(&mut self) {
        if self.instance_idx == self.batch_size {
            return;
        }

        let last_size = self.instance_idx;

        self.embeds = self.embeds.shrink_batch(last_size as u64);
        self.inputs = self.inputs.shrink_batch(last_size as u64);
        self.labels = self.labels.shrink_batch(last_size as u64);
    }

    /// Get the collected tensors.
    ///
    /// This method returns the collected label and vectorized parser state
    /// tensors. Each tensor is `batch_size` in its first dimension, except
    /// the last label/layer tensors, which is sized to the number of instances
    /// of the last batch.
    pub fn into_parts(mut self) -> TensorCollectorParts {
        self.resize_last_batch();

        TensorCollectorParts {
            labels: self.labels,
            embeds: self.embeds,
            inputs: self.inputs,
        }
    }

    fn train_batch(&mut self) {
        let batch_perf = if self.is_training {
            self.model
                .train(&self.embeds, &self.inputs, &self.labels, self.lr)
        } else {
            self.model
                .validate(&self.embeds, &self.inputs, &self.labels)
        };

        self.performance.loss += batch_perf.loss * self.len() as f32;
        self.performance.accuracy += batch_perf.accuracy * self.len() as f32;
        self.n_instances += self.len();
    }
}

impl<'a, T> InstanceCollector<T> for TrainCollector<'a, T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::Transition, state: &ParserState<'_>) -> Result<(), Error> {
        if self.is_full() {
            self.train_batch();
            self.clear();
        }

        let label = self.model.system().transitions().lookup(t.clone());
        self.labels[self.instance_idx] = label as i32;

        let embed_size = self.embeds.dims()[1] as usize;
        let embed_offset = self.instance_idx * embed_size;

        self.model.vectorizer().realize_into(
            state,
            &mut self.embeds[embed_offset..embed_offset + embed_size],
            &mut self.inputs.to_instance_slices(self.instance_idx),
        );

        self.instance_idx += 1;

        Ok(())
    }
}

pub struct TensorCollectorParts {
    pub embeds: Tensor<f32>,
    pub inputs: LayerTensors<i32>,
    pub labels: Tensor<i32>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use conllx::Token;
    use flate2::read::GzDecoder;

    use crate::features::addr::{AddressedValue, Layer, Source};
    use crate::features::{
        self, AddressedValues, InputVectorizer, LayerLookups, Lookup, MutableLookupTable,
    };
    use crate::models::tensorflow::{LayerOp, LayerOps, TensorflowModel};
    use crate::system::{ParserState, Transition};
    use crate::systems::stack_projective::{StackProjectiveSystem, StackProjectiveTransition};
    use crate::train::InstanceCollector;

    use super::TrainCollector;

    #[test]
    fn collect_zero() {
        let vectorizer = test_vectorizer();
        let mut model = test_model(vectorizer);
        let collector = test_collector(&mut model);
        let parts = collector.into_parts();
        assert_eq!(parts.labels.len(), 0);
        assert_eq!(parts.embeds.len(), 0);
        assert_eq!(parts.inputs[features::Layer::Token].len(), 0);
    }

    #[test]
    fn collect_two() {
        let sent = vec![Token::new("een"), Token::new("test")];
        let mut state = ParserState::new(&sent);

        let vectorizer = test_vectorizer();
        let mut model = test_model(vectorizer);
        let mut collector = test_collector(&mut model);
        collector
            .collect(&StackProjectiveTransition::Shift, &state)
            .unwrap();
        StackProjectiveTransition::Shift.apply(&mut state);
        collector
            .collect(&StackProjectiveTransition::LeftArc("FOO".into()), &state)
            .unwrap();
        let parts = collector.into_parts();

        // Check batch shapes.
        assert_eq!(parts.labels.dims(), &[2]);
        assert_eq!(parts.inputs[features::Layer::Token].dims(), &[2, 2]);

        // Check batch contents.
        assert_eq!(&*parts.labels, &[1, 2]);
        assert_eq!(parts.inputs[features::Layer::Token].as_ref(), &[1, 2, 2, 3]);
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
        let mut model = test_model(vectorizer);
        let mut collector = test_collector(&mut model);
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
        let parts = collector.into_parts();

        // Check batch shapes.
        assert_eq!(parts.labels.dims(), &[3]);
        assert_eq!(parts.inputs[features::Layer::Token].dims(), &[3, 2]);

        // Check batch contents.
        assert_eq!(&*parts.labels, &[1, 1, 2]);
        assert_eq!(
            parts.inputs[features::Layer::Token].as_ref(),
            &[1, 2, 2, 3, 3, 4]
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
        let table: Box<dyn Lookup> = Box::new(MutableLookupTable::new());
        lookups.insert(features::Layer::Token, table);
        InputVectorizer::new(lookups, AddressedValues(vec![stack0, buffer0]))
    }

    fn test_model(vectorizer: InputVectorizer) -> TensorflowModel<StackProjectiveSystem> {
        let f = File::open("testdata/parser.graph.gz").expect("Cannot open test graph.");
        let mut decoder = GzDecoder::new(BufReader::new(f));
        let mut data = Vec::new();
        decoder
            .read_to_end(&mut data)
            .expect("Cannot decompress test graph.");

        let mut op_names = LayerOps::new();
        op_names.insert(features::Layer::Token, LayerOp("model/tokens"));
        op_names.insert(features::Layer::Tag, LayerOp("model/tags"));
        op_names.insert(features::Layer::DepRel, LayerOp("model/deprels"));
        op_names.insert(features::Layer::Feature, LayerOp("model/features"));

        let system = StackProjectiveSystem::new();

        TensorflowModel::load_graph(&[], &data, system, vectorizer, &op_names)
            .expect("Cannot load graph.")
    }

    fn test_collector<'a>(
        model: &'a mut TensorflowModel<StackProjectiveSystem>,
    ) -> TrainCollector<'a, StackProjectiveSystem> {
        TrainCollector::new(model, 3, false, 0.01)
    }
}
