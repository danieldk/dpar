use std::f32;
use std::fs::File;
use std::io::{BufReader, Read};

use enum_map::EnumMap;
use tensorflow::{
    Graph, ImportGraphDefOptions, Operation, Session, SessionOptions, StepWithGraph, Tensor,
    TensorType,
};

use super::model::tf_model_to_protobuf;
use super::Model;
use features::{InputVectorizer, Layer, LayerLookups};
use guide::{BatchGuide, Guide};
use system::{ParserState, Transition, TransitionSystem};
use Result;

/// Layer op in the parsing model
///
/// The parser can use several information layers, such as tokens, tags, and
/// dependency relations. Such layers have to be mapped to concrete Tensorflow
/// graph ops. This data structure is used to define a mapping from one
/// particular layer to a Tensorflow placeholder graph op.
pub enum LayerOp<T> {
    /// Op for layers using pre-trained embeddings
    ///
    /// For the use of pre-trained embeddings, we need to ops:
    ///
    /// * The placeholder op for the layer vector.
    /// * The placeholder op for the embedding matrix.
    Embedding { op: T, embed_op: T },

    /// Op for layers using a lookup table.
    ///
    /// The given op is the placeholder op for the layer vector.
    Table { op: T },
}

impl<S> LayerOp<S>
where
    S: AsRef<str>,
{
    /// Convert a graph op identifier to a graph op.
    fn to_graph_op(&self, graph: &Graph) -> Result<LayerOp<Operation>> {
        match self {
            &LayerOp::Embedding {
                ref op,
                ref embed_op,
            } => Ok(LayerOp::Embedding {
                op: graph.operation_by_name_required(op.as_ref())?,
                embed_op: graph.operation_by_name_required(embed_op.as_ref())?,
            }),
            &LayerOp::Table { ref op } => Ok(LayerOp::Table {
                op: graph.operation_by_name_required(op.as_ref())?,
            }),
        }
    }
}

/// A bundling of ops for different layers.
pub struct LayerOps<S>(EnumMap<Layer, Option<LayerOp<S>>>);

impl<S> LayerOps<S>
where
    S: AsRef<str>,
{
    /// Convert a graph op identifiers for all layers to a graph ops.
    fn to_graph_ops(&self, graph: &Graph) -> Result<LayerOps<Operation>> {
        let mut graph_ops = EnumMap::new();

        for (layer, op_name) in &self.0 {
            let op_name = ok_or_continue!(op_name.as_ref());
            graph_ops[layer] = Some(op_name.to_graph_op(graph)?);
        }

        Ok(LayerOps(graph_ops))
    }
}

impl<S> LayerOps<S> {
    /// Construct a new `LayerOps`.
    ///
    /// By default, the op for every layer is set to `None`.
    pub fn new() -> Self {
        LayerOps(EnumMap::new())
    }

    /// Set the op for a layer.
    pub fn insert(&mut self, layer: Layer, op: LayerOp<S>) {
        self.0[layer] = Some(op);
    }

    /// Get the op for a layer.
    pub fn layer_lookup(&self, layer: Layer) -> Option<&LayerOp<S>> {
        self.0[layer].as_ref()
    }
}

/// Simple wrapper for `Tensor` that implements `Default` tensors.
struct TensorWrap(Tensor<i32>);

impl Default for TensorWrap {
    fn default() -> Self {
        TensorWrap(Tensor::new(&[]))
    }
}

/// Parser guide that uses a Tensorflow graph and model.
pub struct TensorflowGuide<T>
where
    T: TransitionSystem,
{
    session: Session,
    system: T,
    vectorizer: InputVectorizer,
    layer_ops: LayerOps<Operation>,
    logits_op: Operation,
}

impl<T> TensorflowGuide<T>
where
    T: TransitionSystem,
{
    /// Load a Tensorflow graph.
    ///
    /// This should be a frozen graph --- a graph in which each variable is
    /// converted to a constant. A graph can be frozen using Tensorflow's
    /// [freeze_graph.py](https://github.com/tensorflow/tensorflow/blob/master/tensorflow/python/tools/freeze_graph.py)
    /// script.
    pub fn load_graph<S>(
        model: &Model,
        system: T,
        vectorizer: InputVectorizer,
        op_names: &LayerOps<S>,
    ) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut f = BufReader::new(File::open(&model.filename)?);
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;

        let opts = ImportGraphDefOptions::new();
        let mut graph = Graph::new();
        graph.import_graph_def(&data, &opts)?;

        let mut session_opts = SessionOptions::new();
        session_opts.set_config(&tf_model_to_protobuf(&model)?)?;
        let session = Session::new(&session_opts, &graph)?;

        let layer_ops = op_names.to_graph_ops(&graph)?;

        // Output
        let logits_op = graph.operation_by_name_required("prediction/model/logits")?;

        Ok(TensorflowGuide {
            system,
            session,
            vectorizer,
            layer_ops,
            logits_op,
        })
    }

    /// Find the best transition given a slice of transition logits.
    fn logits_best_transition<S>(&self, state: &ParserState, logits: S) -> T::T
    where
        S: AsRef<[f32]>,
    {
        // Invariant: we should have as many predictions as transitions.
        let n_predictions = logits.as_ref().len();
        let n_transitions = self.system.transitions().len() + self.system.transitions().start_at();
        assert_eq!(
            n_predictions, n_transitions,
            "Number of transitions ({}) and predictions ({}) are inequal.",
            n_transitions, n_predictions
        );

        let mut best = self.system.transitions().value(0).unwrap();
        let mut best_score = f32::NEG_INFINITY;

        for (idx, logit) in logits.as_ref().iter().enumerate() {
            if *logit > best_score {
                let transition = self.system.transitions().value(idx).unwrap();
                if transition.is_possible(state) {
                    best = transition;
                    best_score = *logit;
                }
            }
        }

        best.clone()
    }
}

impl<T> Guide for TensorflowGuide<T>
where
    T: TransitionSystem,
{
    type T = T::T;

    fn best_transition(&mut self, state: &ParserState) -> Self::T {
        let v = self.vectorizer.realize(state);

        let mut input_tensors = EnumMap::new();
        for (layer, vector) in v.layers {
            // Fixme: make InputVector use Tensor?
            input_tensors[layer] = TensorWrap(slice_to_tensor(&vector));
        }

        let mut step = StepWithGraph::new();
        add_to_step(
            &mut step,
            &self.layer_ops,
            self.vectorizer.layer_lookups(),
            &mut input_tensors,
        );
        let logits_token = step.request_output(&self.logits_op, 0);

        self.session.run(&mut step).expect("Cannot run graph");

        let logits: Tensor<f32> = step.take_output(logits_token)
            .expect("Unable to retrieve output");

        self.logits_best_transition(state, &*logits)
    }
}

impl<T> BatchGuide for TensorflowGuide<T>
where
    T: TransitionSystem,
{
    type T = T::T;

    fn best_transitions(&mut self, states: &[&ParserState]) -> Vec<Self::T> {
        if states.is_empty() {
            return Vec::new();
        }

        // Allocate batch tensors.
        let mut input_tensors = EnumMap::new();
        for (layer, size) in self.vectorizer.layer_sizes() {
            input_tensors[layer] = TensorWrap(Tensor::new(&[states.len() as u64, size as u64]));
        }

        // Fill tensors.
        for (idx, state) in states.iter().enumerate() {
            self.vectorizer.realize_into(
                state,
                &mut batch_to_instance_slices(&mut input_tensors, idx),
            );
        }

        // Prepare step.
        let mut step = StepWithGraph::new();
        add_to_step(
            &mut step,
            &self.layer_ops,
            self.vectorizer.layer_lookups(),
            &mut input_tensors,
        );
        let logits_token = step.request_output(&self.logits_op, 0);

        self.session.run(&mut step).expect("Cannot run graph");

        let logits: Tensor<f32> = step.take_output(logits_token)
            .expect("Unable to retrieve output");

        // Get the best transition for each parser state.
        let n_labels = logits.dims()[1] as usize;
        states
            .iter()
            .enumerate()
            .map(|(idx, state)| {
                let offset = idx * n_labels;
                self.logits_best_transition(state, &logits[offset..offset + n_labels])
            })
            .collect()
    }
}

// Unfortunately, add_to_step cannot be a method of TensorflowGuide with
// the following signature:
//
// add_to_step<'a>(&'a self, step: &mut StepWithGraph<'a>, ...)
//
// Because step would hold a reference to &self, which disallows us to run
// the session, because session running requires &mut self. The following
// RFC would solve this:
//
// https://github.com/rust-lang/rfcs/issues/1215
//
// Another possibility would be to use interior mutability for the
// Tensorflow Session, but I'd like to avoid this.
fn add_to_step<'a>(
    step: &mut StepWithGraph<'a>,
    layer_ops: &LayerOps<Operation>,
    layer_lookups: &'a LayerLookups,
    input_tensors: &'a EnumMap<Layer, TensorWrap>,
) {
    for (layer, layer_op) in &layer_ops.0 {
        let layer_op = ok_or_continue!(layer_op.as_ref());

        match layer_op {
            &LayerOp::Embedding {
                ref op,
                ref embed_op,
            } => {
                // Fill the layer vector placeholder.
                step.add_input(op, 0, &input_tensors[layer].0);

                // Fill the embedding placeholder. If we have an op for
                // the embedding of a layer, there should always be a
                // corresponding embedding matrix.
                let embed_matrix = layer_lookups
                    .layer_lookup(layer)
                    .unwrap()
                    .embed_matrix()
                    .unwrap();
                step.add_input(embed_op, 0, embed_matrix);
            }
            &LayerOp::Table { ref op } => {
                // Fill the layer vector placeholder.
                step.add_input(op, 0, &input_tensors[layer].0);
            }
        }
    }
}

fn slice_to_tensor<T>(slice: &[T]) -> Tensor<T>
where
    T: Copy + TensorType,
{
    let mut tensor = Tensor::new(&[slice.len() as u64]);
    tensor.copy_from_slice(slice);
    tensor
}

/// Extract for each layer the slice corresponding to the `idx`-th
/// instance from the batch.
fn batch_to_instance_slices<'a>(
    batch_tensors: &'a mut EnumMap<Layer, TensorWrap>,
    idx: usize,
) -> EnumMap<Layer, &'a mut [i32]> {
    let mut slices = EnumMap::new();

    for (layer, tensor) in batch_tensors {
        let layer_size = tensor.0.dims()[1] as usize;
        let offset = idx * layer_size;
        slices[layer] = &mut tensor.0[offset..offset + layer_size];
    }

    slices
}
