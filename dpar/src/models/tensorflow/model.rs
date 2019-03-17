use std::f32;
use std::path::Path;

use enum_map::EnumMap;
use failure::{err_msg, Error};
use tensorflow::{
    Graph, ImportGraphDefOptions, Operation, Session, SessionOptions, SessionRunArgs, Status,
    Tensor,
};

use features::{InputVectorizer, Layer};
use models::tensorflow::LayerTensors;
use models::ModelPerformance;
use system::{ParserState, Transition, TransitionSystem};

mod opnames {
    /// Graph initialization.
    pub static INIT: &str = "init";

    /// Save/restore.
    pub static RESTORE: &str = "save/restore_all";
    pub static SAVE: &str = "save/control_dependency";
    pub static SAVE_FILE_PATH: &str = "save/Const";

    /// Training parameters.
    pub static IS_TRAINING: &str = "model/is_training";
    pub static LR: &str = "model/lr";

    /// Embedding layer.
    pub static EMBEDS: &str = "model/embeds";

    /// Model output.
    pub static LOGITS: &str = "model/logits";

    /// Gold-standard targets.
    pub static TARGETS: &str = "model/targets";

    /// Quality measures.
    pub static ACCURACY: &str = "model/accuracy";
    pub static LOSS: &str = "model/loss";

    /// Training.
    pub static TRAIN: &str = "model/train";
}

/// Layer op in the parsing model
///
/// The parser can use several information layers, such as tokens, tags, and
/// dependency relations. Such layers have to be mapped to concrete Tensorflow
/// graph ops. This data structure is used to define a mapping from one
/// particular layer to a Tensorflow placeholder graph op, for layers that
/// are table lookups.
pub struct LayerOp<T>(pub T);

impl<S> LayerOp<S>
where
    S: AsRef<str>,
{
    /// Convert a graph op identifier to a graph op.
    fn to_graph_op(&self, graph: &Graph) -> Result<LayerOp<Operation>, Error> {
        Ok(LayerOp(
            graph
                .operation_by_name_required(self.0.as_ref())
                .map_err(status_to_error)?,
        ))
    }
}

/// A bundling of ops for the input layers.
pub struct LayerOps<S>(EnumMap<Layer, Option<LayerOp<S>>>);

impl<S> LayerOps<S>
where
    S: AsRef<str>,
{
    /// Convert a graph op identifiers for all layers to a graph ops.
    fn to_graph_ops(&self, graph: &Graph) -> Result<LayerOps<Operation>, Error> {
        let mut graph_ops = EnumMap::new();

        for (layer, op_name) in &self.0 {
            let op_name = ok_or!(op_name.as_ref(), continue);
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

/// A Tensorflow model.
///
/// This data structure can be used to load a Tensorflow parser graph.
/// The graph's parameters can be trained and validated using the `train`
/// and `validate` methods. Moreover, a trained graph can be used to
/// predict the best transition given a parser state using
/// `predict_best_transition`.
pub struct TensorflowModel<T>
where
    T: TransitionSystem,
{
    session: Session,
    system: T,
    vectorizer: InputVectorizer,
    layer_ops: LayerOps<Operation>,
    init_op: Operation,
    restore_op: Operation,
    save_op: Operation,
    save_file_path_op: Operation,
    lr_op: Operation,
    is_training_op: Operation,
    embeds_op: Operation,
    accuracy_op: Operation,
    logits_op: Operation,
    loss_op: Operation,
    targets_op: Operation,
    train_op: Operation,
}

impl<T> TensorflowModel<T>
where
    T: TransitionSystem,
{
    /// Load a Tensorflow graph.
    ///
    /// This constructor will use the graphs's initializer to initialize the
    /// graph's parameters.
    pub fn load_graph<S>(
        config_protobuf: &[u8],
        model_protobuf: &[u8],
        system: T,
        vectorizer: InputVectorizer,
        op_names: &LayerOps<S>,
    ) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let mut model = Self::load_graph_(
            config_protobuf,
            model_protobuf,
            system,
            vectorizer,
            op_names,
        )?;

        // Initialize parameters.
        let mut args = SessionRunArgs::new();
        args.add_target(&model.init_op);
        model
            .session
            .run(&mut args)
            .expect("Cannot initialize parameters");

        Ok(model)
    }

    /// Load a Tensorflow graph.
    ///
    /// This constructor will load the model parameters (such as weights) from
    /// the file specified in `parameters_path`.
    pub fn load_graph_with_weights<P, S>(
        config_protobuf: &[u8],
        model_protobuf: &[u8],
        parameters_path: P,
        system: T,
        vectorizer: InputVectorizer,
        op_names: &LayerOps<S>,
    ) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let mut model = Self::load_graph_(
            config_protobuf,
            model_protobuf,
            system,
            vectorizer,
            op_names,
        )?;

        // Restore parameters.
        let path_tensor = prepare_path(parameters_path)?.into();
        let mut args = SessionRunArgs::new();
        args.add_feed(&model.save_file_path_op, 0, &path_tensor);
        args.add_target(&model.restore_op);
        model.session.run(&mut args).map_err(status_to_error)?;

        Ok(model)
    }

    fn add_op(graph: &Graph, name: &str) -> Result<Operation, Error> {
        graph
            .operation_by_name_required(name)
            .map_err(status_to_error)
    }

    /// Load a Tensorflow graph.
    ///
    /// This function should be used by a wrapping constructor that initializes
    /// the graph parameters.
    fn load_graph_<S>(
        config_protobuf: &[u8],
        model_protobuf: &[u8],
        system: T,
        vectorizer: InputVectorizer,
        op_names: &LayerOps<S>,
    ) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let opts = ImportGraphDefOptions::new();
        let mut graph = Graph::new();
        graph
            .import_graph_def(model_protobuf, &opts)
            .map_err(status_to_error)?;

        let mut session_opts = SessionOptions::new();
        session_opts
            .set_config(config_protobuf)
            .map_err(status_to_error)?;
        let session = Session::new(&session_opts, &graph).map_err(status_to_error)?;

        let layer_ops = op_names.to_graph_ops(&graph)?;

        let init_op = Self::add_op(&graph, opnames::INIT)?;
        let restore_op = Self::add_op(&graph, opnames::RESTORE)?;
        let save_op = Self::add_op(&graph, opnames::SAVE)?;
        let save_file_path_op = Self::add_op(&graph, opnames::SAVE_FILE_PATH)?;

        let is_training_op = Self::add_op(&graph, opnames::IS_TRAINING)?;
        let lr_op = Self::add_op(&graph, opnames::LR)?;

        let embeds_op = Self::add_op(&graph, opnames::EMBEDS)?;

        let accuracy_op = Self::add_op(&graph, opnames::ACCURACY)?;
        let logits_op = Self::add_op(&graph, opnames::LOGITS)?;
        let loss_op = Self::add_op(&graph, opnames::LOSS)?;
        let targets_op = Self::add_op(&graph, opnames::TARGETS)?;

        let train_op = Self::add_op(&graph, opnames::TRAIN)?;

        Ok(TensorflowModel {
            system,
            session,
            vectorizer,
            layer_ops,
            init_op,
            restore_op,
            save_op,
            save_file_path_op,
            is_training_op,
            lr_op,
            embeds_op,
            accuracy_op,
            logits_op,
            loss_op,
            targets_op,
            train_op,
        })
    }

    /// Predict the best transitions for a batch of parser states.
    ///
    /// Both the parser states and the feature representations of the parser
    /// states should be provided. Returns the best (possible) transition for
    /// each parser state.
    pub fn predict(
        &mut self,
        states: &[&ParserState],
        embeds_tensor: &Tensor<f32>,
        input_tensors: &LayerTensors<i32>,
    ) -> Vec<T::Transition> {
        let logits = self.logits(embeds_tensor, input_tensors);

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

    /// Return the best transition for a parser state.
    ///
    /// This method finds the best transition (largest logit) that is possible given the
    /// current parser state.
    fn logits_best_transition(&self, state: &ParserState, logits: &[f32]) -> T::Transition {
        // Invariant: we should have as many predictions as transitions.
        let n_predictions = logits.len();
        let n_transitions = self.system.transitions().len();
        assert_eq!(
            n_predictions, n_transitions,
            "Number of transitions ({}) and predictions ({}) are inequal.",
            n_transitions, n_predictions
        );

        let mut best = self.system.transitions().value(1).unwrap();
        let mut best_score = f32::NEG_INFINITY;

        for (idx, logit) in logits.as_ref().iter().enumerate() {
            // The special transition 0 is used for unknown transitions
            // (e.g. in validation). Must be skipped in prediction.
            if idx == 0 {
                continue;
            }

            if *logit > best_score {
                let transition = self
                    .system
                    .transitions()
                    .value(idx)
                    .expect("Invalid transition index.");
                if transition.is_possible(state) {
                    best = transition;
                    best_score = *logit;
                }
            }
        }

        best.into_owned()
    }

    /// Compute transition logits from the feature representations of the
    /// parser states.
    ///
    /// Each input tensor has shape *[batch_size, layer_size]*. Returns a logits
    /// tensor with shape *[batch_size, n_transitions]*.
    fn logits(
        &mut self,
        embeds_tensor: &Tensor<f32>,
        input_tensors: &LayerTensors<i32>,
    ) -> Tensor<f32> {
        let mut is_training = Tensor::new(&[]);
        is_training[0] = false;

        let mut args = SessionRunArgs::new();
        args.add_feed(&self.is_training_op, 0, &is_training);
        args.add_feed(&self.embeds_op, 0, embeds_tensor);
        add_to_args(&mut args, &self.layer_ops, &input_tensors);
        let logits_token = args.request_fetch(&self.logits_op, 0);
        self.session.run(&mut args).expect("Cannot run graph");

        args.fetch(logits_token).expect("Unable to retrieve output")
    }

    /// Save the model parameters.
    ///
    /// The model parameters are stored as the given path.
    pub fn save<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        // Add leading directory component if absent.
        let path_tensor = prepare_path(path)?.into();

        // Call the save op.
        let mut args = SessionRunArgs::new();
        args.add_feed(&self.save_file_path_op, 0, &path_tensor);
        args.add_target(&self.save_op);
        self.session.run(&mut args).map_err(status_to_error)
    }

    /// Perform a training step.
    ///
    /// This method updates the model parameters using a batch of parser
    /// state feature representations (`input_tensors`), gold standard
    /// labels (`targets`) and a learning rate.
    ///
    /// The loss and accuracy on the gold standard labels are returned.
    pub fn train(
        &mut self,
        embeds: &Tensor<f32>,
        input_tensors: &LayerTensors<i32>,
        targets: &Tensor<i32>,
        learning_rate: f32,
    ) -> ModelPerformance {
        let mut is_training = Tensor::new(&[]);
        is_training[0] = true;

        let mut lr = Tensor::new(&[]);
        lr[0] = learning_rate;

        let mut args = SessionRunArgs::new();
        args.add_feed(&self.is_training_op, 0, &is_training);
        args.add_feed(&self.lr_op, 0, &lr);
        args.add_target(&self.train_op);

        self.validate_(args, embeds, input_tensors, targets)
    }

    /// Perform a validation step.
    ///
    /// This method computes the loss and accuracy for the gold standard
    /// labels (`targets`) given a batch of parser state feature representations
    /// (`input_tensors`).
    pub fn validate(
        &mut self,
        embeds: &Tensor<f32>,
        input_tensors: &LayerTensors<i32>,
        targets: &Tensor<i32>,
    ) -> ModelPerformance {
        let mut is_training = Tensor::new(&[]);
        is_training[0] = false;

        let mut args = SessionRunArgs::new();
        args.add_feed(&self.is_training_op, 0, &is_training);
        self.validate_(args, embeds, input_tensors, targets)
    }

    fn validate_<'l>(
        &'l mut self,
        mut args: SessionRunArgs<'l>,
        embeds: &'l Tensor<f32>,
        input_tensors: &'l LayerTensors<i32>,
        targets: &'l Tensor<i32>,
    ) -> ModelPerformance {
        // Add inputs.
        args.add_feed(&self.embeds_op, 0, embeds);
        add_to_args(&mut args, &self.layer_ops, input_tensors);

        // Add gold labels.
        args.add_feed(&self.targets_op, 0, targets);

        let accuracy_token = args.request_fetch(&self.accuracy_op, 0);
        let loss_token = args.request_fetch(&self.loss_op, 0);

        self.session.run(&mut args).expect("Cannot run graph");

        ModelPerformance {
            loss: args.fetch(loss_token).expect("Unable to retrieve loss")[0],
            accuracy: args
                .fetch(accuracy_token)
                .expect("Unable to retrieve accuracy")[0],
        }
    }

    /// Return the vectorizer associated with the model.
    pub fn vectorizer(&self) -> &InputVectorizer {
        &self.vectorizer
    }
}

// Unfortunately, add_to_args cannot be a method of TensorflowModel with
// the following signature:
//
// add_to_args<'l>(&'a self, step: &mut SessionRunArgs<'l>, ...)
//
// Because args would hold a reference to &self, which disallows us to run
// the session, because session running requires &mut self. The following
// RFC would solve this:
//
// https://github.com/rust-lang/rfcs/issues/1215
//
// Another possibility would be to use interior mutability for the
// Tensorflow Session, but I'd like to avoid this.
fn add_to_args<'l>(
    args: &mut SessionRunArgs<'l>,
    layer_ops: &LayerOps<Operation>,
    input_tensors: &'l LayerTensors<i32>,
) {
    for (layer, layer_op) in &layer_ops.0 {
        let layer_op = ok_or!(layer_op.as_ref(), continue);
        // Fill the layer vector placeholder.
        args.add_feed(&layer_op.0, 0, &input_tensors[layer]);
    }
}

/// Tensorflow requires a path that contains a directory component.
fn prepare_path<P>(path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path = if path.components().count() == 1 {
        Path::new("./").join(path)
    } else {
        path.to_owned()
    };

    path.to_str()
        .ok_or(err_msg("Filename contains non-unicode characters"))
        .map(ToOwned::to_owned)
}

/// Convert Tensorflow status to a string.
///
/// tensorflow::Status is not Sync, which is required by failure.
fn status_to_error(status: Status) -> Error {
    format_err!("{}", status)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use flate2::read::GzDecoder;

    use features::{AddressedValues, InputVectorizer, Layer, LayerLookups};
    use systems::StackProjectiveSystem;

    use super::{LayerOp, LayerOps, TensorflowModel};

    #[test]
    fn load_graph_test() {
        let f = File::open("testdata/parser.graph.gz").expect("Cannot open test graph.");
        let mut decoder = GzDecoder::new(BufReader::new(f));
        let mut data = Vec::new();
        decoder
            .read_to_end(&mut data)
            .expect("Cannot decompress test graph.");

        let system = StackProjectiveSystem::new();
        let vectorizer = InputVectorizer::new(LayerLookups::new(), AddressedValues(Vec::new()));

        let mut op_names = LayerOps::new();
        op_names.insert(Layer::Token, LayerOp("model/tokens"));
        op_names.insert(Layer::Tag, LayerOp("model/tags"));
        op_names.insert(Layer::DepRel, LayerOp("model/deprels"));
        op_names.insert(Layer::Feature, LayerOp("model/features"));

        TensorflowModel::load_graph(&[], &data, system, vectorizer, &op_names)
            .expect("Cannot load graph.");
    }
}
