use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use failure::{format_err, Error};
use finalfusion::{
    embeddings::Embeddings as FFEmbeddings,
    io::{MmapEmbeddings, ReadEmbeddings},
    storage::StorageWrap,
    vocab::VocabWrap,
};
use ordered_float::NotNan;
use protobuf::core::Message;
use serde_derive::{Deserialize, Serialize};
use tf_proto::ConfigProto;

use dpar::features;
use dpar::features::{AddressedValues, Embeddings, Layer, LayerLookups};
use dpar::models::lr::PlateauLearningRate;
use dpar::models::tensorflow::{LayerOp, LayerOps};

use crate::StoredLookupTable;

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

        relativize_embed_path(config_path, &mut self.lookups.word)?;
        relativize_embed_path(config_path, &mut self.lookups.tag)?;
        relativize_embed_path(config_path, &mut self.lookups.deprel)?;
        relativize_embed_path(config_path, &mut self.lookups.feature)?;

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
    pub fn load_inputs(&self) -> Result<AddressedValues, Error> {
        let f = File::open(&self.inputs)?;
        Ok(AddressedValues::from_buf_read(BufReader::new(f))?)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Lookups {
    pub word: Option<Lookup>,
    pub tag: Option<Lookup>,
    pub deprel: Option<Lookup>,
    pub feature: Option<Lookup>,
}

impl Lookups {
    pub fn construct_lookups_with<F>(&self, load_fun: F) -> Result<LayerLookups, Error>
    where
        F: Fn(&Lookup) -> Result<Box<dyn features::Lookup>, Error>,
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

        Ok(lookups)
    }

    pub fn create_lookups(&self) -> Result<LayerLookups, Error> {
        self.construct_lookups_with(|l| self.create_layer_tables(l))
    }

    fn create_layer_tables(&self, lookup: &Lookup) -> Result<Box<dyn features::Lookup>, Error> {
        match *lookup {
            Lookup::Embedding {
                ref filename,
                ref alloc,
                ..
            } => Ok(Box::new(Self::load_embeddings(filename, *alloc)?)),
            Lookup::Table { ref filename, .. } => {
                Ok(Box::new(StoredLookupTable::create(filename)?))
            }
        }
    }

    pub fn load_lookups(&self) -> Result<LayerLookups, Error> {
        self.construct_lookups_with(|l| self.load_layer_tables(l))
    }

    pub fn layer_ops(&self) -> LayerOps<String> {
        let mut names = LayerOps::new();

        self.insert_layer_op(&mut names, Layer::Token, &self.word);
        self.insert_layer_op(&mut names, Layer::Tag, &self.tag);
        self.insert_layer_op(&mut names, Layer::DepRel, &self.deprel);
        self.insert_layer_op(&mut names, Layer::Feature, &self.feature);

        names
    }

    fn insert_layer_op(&self, names: &mut LayerOps<String>, layer: Layer, lookup: &Option<Lookup>) {
        let lookup = match lookup {
            Some(ref lookup) => lookup,
            None => return,
        };

        if let Lookup::Table { ref op, .. } = lookup {
            names.insert(layer, LayerOp(op.clone()));
        }
    }

    fn load_layer_tables(&self, lookup: &Lookup) -> Result<Box<dyn features::Lookup>, Error> {
        match *lookup {
            Lookup::Embedding {
                ref filename,
                ref alloc,
                ..
            } => Ok(Box::new(Self::load_embeddings(filename, *alloc)?)),
            Lookup::Table { ref filename, .. } => Ok(Box::new(StoredLookupTable::open(filename)?)),
        }
    }

    fn load_embeddings(filename: &str, alloc: EmbeddingAlloc) -> Result<Embeddings, Error> {
        let f = File::open(filename)?;
        let mut buf_read = BufReader::new(f);
        let embeds: FFEmbeddings<VocabWrap, StorageWrap> = match alloc {
            EmbeddingAlloc::Read => ReadEmbeddings::read_embeddings(&mut buf_read)?,
            EmbeddingAlloc::Mmap => MmapEmbeddings::mmap_embeddings(&mut buf_read)?,
        };

        Ok(embeds.into())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Lookup {
    Embedding {
        filename: String,
        alloc: EmbeddingAlloc,
        op: String,
        embed_op: String,
    },
    Table {
        filename: String,
        op: String,
    },
}

#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum EmbeddingAlloc {
    Mmap,
    Read,
}

fn relativize_embed_path(config_path: &Path, embed: &mut Option<Lookup>) -> Result<(), Error> {
    if let Some(embed) = embed.as_mut() {
        match *embed {
            Lookup::Embedding {
                ref mut filename, ..
            } => {
                *filename = relativize_path(config_path, &filename)?;
            }
            Lookup::Table {
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
        .ok_or_else(|| {
            format_err!(
                "Cannot get the parent path for the configuration file {}",
                config_path.display()
            )
        })?
        .join(path)
        .to_str()
        .ok_or_else(|| {
            format_err!(
                "Cannot convert parent path of configuration file to string: {}",
                config_path.display()
            )
        })?
        .to_owned())
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
    pub lr_scale: NotNan<f32>,
    pub lr_patience: usize,
    pub patience: usize,
}

impl Train {
    pub fn lr_schedule(&self) -> PlateauLearningRate {
        PlateauLearningRate::new(
            self.initial_lr.into_inner(),
            self.lr_scale.into_inner(),
            self.lr_patience,
        )
    }
}
