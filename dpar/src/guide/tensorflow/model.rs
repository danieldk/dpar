use protobuf::core::Message;
use tf_proto::ConfigProto;

use Result;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
    /// The filename of the frozen Tensorflow graph.
    pub filename: String,

    /// Thread pool size for parallel processing within a computation
    /// graph op.
    pub intra_op_parallelism_threads: usize,

    /// Thread pool size for parallel processing of independent computation
    /// graph ops.
    pub inter_op_parallelism_threads: usize,
}

pub fn tf_model_to_protobuf(model: &Model) -> Result<Vec<u8>> {
    let mut config_proto = ConfigProto::new();
    config_proto.intra_op_parallelism_threads = model.intra_op_parallelism_threads as i32;
    config_proto.inter_op_parallelism_threads = model.inter_op_parallelism_threads as i32;

    let mut bytes = Vec::new();
    config_proto.write_to_vec(&mut bytes)?;

    Ok(bytes)
}
