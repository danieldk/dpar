use std::error::Error as StdError;
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use failure;
use pest::error::Error as PestError;
use protobuf::ProtobufError;
use tensorflow;

use features::parse_addr::Rule;

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(ParseIntError);
        Pest(PestError<Rule>);
        Protobuf(ProtobufError);
        Utf8(FromUtf8Error);
    }

     errors {
        ParseError(t: String)
        ConfigError(t: String)
        FilenameEncodingError(t: String)
        TensorflowError(t: String) {
            description("tensorflow error")
            display("tensorflow error: '{}'", t)
        }
        FailureError(e: failure::Error) {
            description("failure error")
            display("{}", e)
        }
    }
}

impl From<tensorflow::Status> for Error {
    fn from(status: tensorflow::Status) -> Self {
        ErrorKind::TensorflowError(status.description().to_owned()).into()
    }
}

impl From<failure::Error> for Error {
    fn from(error: failure::Error) -> Self {
        ErrorKind::FailureError(error).into()
    }
}
