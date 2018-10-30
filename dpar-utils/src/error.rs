use std::io;

use conllx;
use dpar;
use failure;
use protobuf::ProtobufError;
use serde_cbor;
use toml;

error_chain! {
    foreign_links {
        Conllx(conllx::Error);
        IO(io::Error);
        Dpar(dpar::Error);
        Protobuf(ProtobufError);
        TOML(toml::de::Error);
        TOMLSerde(toml::ser::Error);
        CBORSerde(serde_cbor::Error);
    }

    errors {
        ConfigError(m: String) {
            description("configuration error")
                display("configuration error: {}", m)
        }
        FailureError(e: failure::Error) {
            description("failure error")
            display("{}", e)
        }
    }
}

impl From<failure::Error> for Error {
    fn from(error: failure::Error) -> Self {
        ErrorKind::FailureError(error).into()
    }
}
