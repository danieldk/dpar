use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tf_embed;
use tf_embed::ReadWord2Vec;

use dpar::features;
use dpar::features::{AddressedValues, Layer, LayerLookups};
use dpar::guide::tensorflow::{LayerOp, LayerOps, Model};

use {ErrorKind, Result, StoredLookupTable};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub model: Model,
    pub parser: Parser,
    pub lookups: Lookups,
}

impl Config {
    pub fn relativize_paths<P>(&mut self, config_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let config_path = config_path.as_ref();

        self.model.filename = relativize_path(config_path, &self.model.filename)?;
        self.parser.inputs = relativize_path(config_path, &self.parser.inputs)?;
        self.parser.transitions = relativize_path(config_path, &self.parser.transitions)?;

        relativize_embed_path(config_path, &mut self.lookups.word)?;
        relativize_embed_path(config_path, &mut self.lookups.tag)?;
        relativize_embed_path(config_path, &mut self.lookups.deprel)?;
        relativize_embed_path(config_path, &mut self.lookups.feature)?;
        relativize_embed_path(config_path, &mut self.lookups.chars)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Parser {
    pub pproj: bool,
    pub system: String,
    pub inputs: String,
    pub transitions: String,
    pub train_batch_size: usize,
    pub parse_batch_size: usize,
}

impl Parser {
    pub fn load_inputs(&self) -> Result<AddressedValues> {
        let f = File::open(&self.inputs)?;
        Ok(AddressedValues::from_buf_read(BufReader::new(f))?)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Lookups {
    pub word: Option<Lookup>,
    pub tag: Option<Lookup>,
    pub deprel: Option<Lookup>,
    pub chars: Option<Lookup>,
    pub feature: Option<Lookup>,
}

impl Lookups {
    pub fn load_lookups(&self) -> Result<LayerLookups> {
        let mut lookups = LayerLookups::new();

        if let Some(ref embed) = self.word {
            lookups.insert(Layer::Token, self.load_layer_tables(embed)?);
        }

        if let Some(ref embed) = self.tag {
            lookups.insert(Layer::Tag, self.load_layer_tables(embed)?);
        }

        if let Some(ref embed) = self.deprel {
            lookups.insert(Layer::DepRel, self.load_layer_tables(embed)?);
        }

        if let Some(ref embed) = self.feature {
            lookups.insert(Layer::Feature, self.load_layer_tables(embed)?);
        }

        if let Some(ref embed) = self.chars {
            lookups.insert(Layer::Char, self.load_layer_tables(embed)?);
        }

        Ok(lookups)
    }

    pub fn layer_ops(&self) -> LayerOps<String> {
        let mut names = LayerOps::new();

        if let Some(ref lookup) = self.word {
            names.insert(Layer::Token, self.layer_op(lookup));
        }

        if let Some(ref lookup) = self.tag {
            names.insert(Layer::Tag, self.layer_op(lookup));
        }

        if let Some(ref lookup) = self.deprel {
            names.insert(Layer::DepRel, self.layer_op(lookup));
        }

        if let Some(ref lookup) = self.feature {
            names.insert(Layer::Feature, self.layer_op(lookup));
        }

        if let Some(ref lookup) = self.chars {
            names.insert(Layer::Char, self.layer_op(lookup));
        }

        names
    }

    fn layer_op(&self, lookup: &Lookup) -> LayerOp<String> {
        match lookup {
            &Lookup::Embedding {
                ref op,
                ref embed_op,
                ..
            } => LayerOp::Embedding {
                op: op.clone(),
                embed_op: embed_op.clone(),
            },
            &Lookup::Table { ref op, .. } => LayerOp::Table { op: op.clone() },
        }
    }

    fn load_layer_tables(&self, lookup: &Lookup) -> Result<Box<features::Lookup>> {
        match lookup {
            &Lookup::Embedding {
                ref filename,
                normalize,
                ..
            } => {
                let f = File::open(filename)?;
                let mut embeds =
                    tf_embed::Embeddings::read_word2vec_binary(&mut BufReader::new(f))?;

                if normalize {
                    embeds.normalize()
                }

                Ok(Box::new(embeds))
            }
            &Lookup::Table { ref filename, .. } => {
                Ok(Box::new(StoredLookupTable::open_or_create(filename)?))
            }
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Lookup {
    Embedding {
        filename: String,
        normalize: bool,
        op: String,
        embed_op: String,
    },
    Table {
        filename: String,
        op: String,
    },
}

fn relativize_embed_path(config_path: &Path, embed: &mut Option<Lookup>) -> Result<()> {
    if let Some(embed) = embed.as_mut() {
        match embed {
            &mut Lookup::Embedding {
                ref mut filename, ..
            } => {
                *filename = relativize_path(config_path, &filename)?;
            }
            &mut Lookup::Table {
                ref mut filename, ..
            } => {
                *filename = relativize_path(config_path, &filename)?;
            }
        }
    }

    Ok(())
}

fn relativize_path(config_path: &Path, filename: &str) -> Result<String> {
    if filename.is_empty() {
        return Ok(filename.to_owned());
    }

    let path = Path::new(&filename);

    // Don't touch absolute paths.
    if path.is_absolute() {
        return Ok(filename.to_owned());
    }

    let abs_config_path = config_path.canonicalize()?;
    Ok(abs_config_path
        .parent()
        .ok_or(ErrorKind::ConfigError(String::from(
            "Cannot get the parent path of the configuration file",
        )))?.join(path)
        .to_str()
        .ok_or(ErrorKind::ConfigError(String::from(
            "Cannot convert path to string",
        )))?.to_owned())
}
