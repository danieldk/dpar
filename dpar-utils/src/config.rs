use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use failure::Error;
use ordered_float::NotNan;
use protobuf::core::Message;
use tf_embed;
use tf_embed::ReadWord2Vec;
use tf_proto::ConfigProto;

use dpar::features;
use dpar::features::{AddressedValues, Layer, LayerLookups};
use dpar::models::lr::ExponentialDecay;
use dpar::models::tensorflow::{LayerOp, LayerOps};

use util::associations_from_buf_read;
use StoredLookupTable;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub model: Model,
    pub parser: Parser,
    pub lookups: Lookups,
    pub train: Train,
}

impl Config {
    pub fn relativize_paths<P>(&mut self, config_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let config_path = config_path.as_ref();

        self.model.graph = relativize_path(config_path, &self.model.graph)?;
        self.model.parameters = relativize_path(config_path, &self.model.parameters)?;
        self.parser.inputs = relativize_path(config_path, &self.parser.inputs)?;
        self.parser.transitions = relativize_path(config_path, &self.parser.transitions)?;
        self.parser.associations = relativize_path(config_path, &self.parser.associations)?;

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
    pub associations: String,
    pub train_batch_size: usize,
    pub parse_batch_size: usize,
}

impl Parser {
    pub fn load_inputs(&self) -> Result<AddressedValues, Error> {
        let f = File::open(&self.inputs)?;
        Ok(AddressedValues::from_buf_read(BufReader::new(f))?)
    }

    pub fn load_associations(&self) -> Result<HashMap<(String, String, String), f32>, Error> {
        let f = File::open(&self.associations)?;
        Ok(associations_from_buf_read(f)?)
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
    pub fn construct_lookups_with<F>(&self, load_fun: F) -> Result<LayerLookups, Error>
    where
        F: Fn(&Lookup) -> Result<Box<features::Lookup>, Error>,
    {
        let mut lookups = LayerLookups::new();

        if let Some(ref lookup) = self.word {
            lookups.insert(Layer::Token, load_fun(lookup)?);
        }

        if let Some(ref lookup) = self.tag {
            lookups.insert(Layer::Tag, load_fun(lookup)?);
        }

        if let Some(ref lookup) = self.deprel {
            lookups.insert(Layer::DepRel, load_fun(lookup)?);
        }

        if let Some(ref lookup) = self.feature {
            lookups.insert(Layer::Feature, load_fun(lookup)?);
        }

        if let Some(ref lookup) = self.chars {
            lookups.insert(Layer::Char, load_fun(lookup)?);
        }

        Ok(lookups)
    }

    pub fn create_lookups(&self) -> Result<LayerLookups, Error> {
        self.construct_lookups_with(|l| self.create_layer_tables(l))
    }

    fn create_layer_tables(&self, lookup: &Lookup) -> Result<Box<features::Lookup>, Error> {
        match lookup {
            &Lookup::Embedding {
                ref filename,
                normalize,
                ..
            } => Ok(Box::new(Self::load_embeddings(filename, normalize)?)),
            &Lookup::Table { ref filename, .. } => {
                Ok(Box::new(StoredLookupTable::create(filename)?))
            }
        }
    }

    pub fn load_lookups(&self) -> Result<LayerLookups, Error> {
        self.construct_lookups_with(|l| self.load_layer_tables(l))
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

    fn load_layer_tables(&self, lookup: &Lookup) -> Result<Box<features::Lookup>, Error> {
        match lookup {
            &Lookup::Embedding {
                ref filename,
                normalize,
                ..
            } => Ok(Box::new(Self::load_embeddings(filename, normalize)?)),
            &Lookup::Table { ref filename, .. } => Ok(Box::new(StoredLookupTable::open(filename)?)),
        }
    }

    fn load_embeddings(filename: &str, normalize: bool) -> Result<tf_embed::Embeddings, Error> {
        let f = File::open(filename)?;
        let mut embeds = tf_embed::Embeddings::read_word2vec_binary(&mut BufReader::new(f))?;

        if normalize {
            embeds.normalize()
        }

        Ok(embeds)
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

fn relativize_embed_path(config_path: &Path, embed: &mut Option<Lookup>) -> Result<(), Error> {
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

fn relativize_path(config_path: &Path, filename: &str) -> Result<String, Error> {
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
        .ok_or(format_err!(
            "Cannot get the parent path for the configuration file {}",
            config_path.display()
        ))?.join(path)
        .to_str()
        .ok_or(format_err!(
            "Cannot convert parent path of configuration file to string: {}",
            config_path.display()
        ))?.to_owned())
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
    /// The filename of the Tensorflow graph.
    pub graph: String,

    /// The filename of the trained graph parameters.
    pub parameters: String,

    /// Thread pool size for parallel processing within a computation
    /// graph op.
    pub intra_op_parallelism_threads: usize,

    /// Thread pool size for parallel processing of independent computation
    /// graph ops.
    pub inter_op_parallelism_threads: usize,
}

impl Model {
    pub fn read_graph(&self) -> Result<Vec<u8>, Error> {
        let mut f = BufReader::new(File::open(&self.graph)?);
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn config_to_protobuf(&self) -> Result<Vec<u8>, Error> {
        let mut config_proto = ConfigProto::new();
        config_proto.intra_op_parallelism_threads = self.intra_op_parallelism_threads as i32;
        config_proto.inter_op_parallelism_threads = self.inter_op_parallelism_threads as i32;

        let mut bytes = Vec::new();
        config_proto.write_to_vec(&mut bytes)?;

        Ok(bytes)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Train {
    pub initial_lr: NotNan<f32>,
    pub decay_rate: NotNan<f32>,
    pub decay_steps: usize,
    pub staircase: bool,
    pub patience: usize,
}

impl Train {
    pub fn lr_schedule(&self) -> ExponentialDecay {
        ExponentialDecay::new(
            self.initial_lr.into_inner(),
            self.decay_rate.into_inner(),
            self.decay_steps,
            self.staircase,
        )
    }
}
