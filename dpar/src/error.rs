use std::error::Error as StdError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::io;

use hdf5;
use protobuf::ProtobufError;
use tensorflow;
use tf_embed;

error_chain! {
    foreign_links {
        HDF5(hdf5::Error);
        Io(io::Error);
        ParseInt(ParseIntError);
        Protobuf(ProtobufError);
        Embed(tf_embed::Error);
        Utf8(FromUtf8Error);
    }

     errors {
        ParseError(t: String)
        ConfigError(t: String)
        TensorflowError(t: String) {
            description("tensorflow error")
            display("tensorflow error: '{}'", t)
        }
    }
}

impl From<tensorflow::Status> for Error {
    fn from(status: tensorflow::Status) -> Self {
        ErrorKind::TensorflowError(status.description().to_owned()).into()
    }
}
