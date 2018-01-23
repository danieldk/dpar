use std::io;

use conllx;
use dpar;
use serde_cbor;
use tf_embed;
use toml;

error_chain! {
    foreign_links {
        Conllx(conllx::Error);
        IO(io::Error);
        Dpar(dpar::Error);
        TOML(toml::de::Error);
        TOMLSerde(toml::ser::Error);
        CBORSerde(serde_cbor::Error);
        Embed(tf_embed::Error);
    }

    errors {
        ConfigError(m: String) {
            description("configuration error")
                display("configuration error: {}", m)
        }
    }
}
