use std::borrow::Cow;
use std::fmt;
use std::io::BufRead;
use std::result;

use enum_map::{Enum, EnumMap};
use failure::Error;
use tensorflow::Tensor;

use crate::features::addr;
use crate::features::lookup::LookupResult;
use crate::features::parse_addr::parse_addressed_values;
use crate::features::{BoxedLookup, Lookup, LookupType};
use crate::system::ParserState;

/// Multiple addressable parts of the parser state.
///
/// `AddressedValues` represents multiple addresses into the parser state.
/// This can be used to construct feature vectors over the parser state.
pub struct AddressedValues(pub Vec<addr::AddressedValue>);

impl AddressedValues {
    /// Read addressed values specification from a text file.
    ///
    /// Such a text file consists of lines with the format
    ///
    /// ~~~text,no_run
    /// [address+] layer
    /// ~~~
    ///
    /// Multiple addresses are used to e.g. address the left/rightmost
    /// dependency of a token on the stack or buffer.
    pub fn from_buf_read<R>(mut read: R) -> Result<Self, Error>
    where
        R: BufRead,
    {
        let mut data = String::new();
        read.read_to_string(&mut data)?;
        Ok(AddressedValues(parse_addressed_values(&data)?))
    }
}

/// A feature vector.
///
/// `InputVector` instances represent feature vectors, also called
/// input layers in neural networks. The input vector is split in
/// vectors for different layers. In each layer, the feature is encoded
/// as a 32-bit identifier, which is typically the row of the layer
/// value in an embedding matrix.
pub struct InputVector {
    pub lookup_layers: EnumMap<Layer, Vec<i32>>,
    pub embed_layer: Tensor<f32>,
}

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum Layer {
    Token,
    Tag,
    DepRel,
    Feature,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        let s = match *self {
            Layer::Token => "tokens",
            Layer::Tag => "tags",
            Layer::DepRel => "deprels",
            Layer::Feature => "features",
        };

        f.write_str(s)
    }
}

// I am not sure whether I like the use of Borrow here, but is there another
// convenient way to convert from both addr::Layer and &addr::Layer?
impl From<&addr::Layer> for Layer {
    fn from(layer: &addr::Layer) -> Self {
        match *layer {
            addr::Layer::Token => Layer::Token,
            addr::Layer::Tag => Layer::Tag,
            addr::Layer::DepRel => Layer::DepRel,
            addr::Layer::Feature(_) => Layer::Feature,
        }
    }
}

/// Lookups for layers.
///
/// This data structure bundles lookups for the different layers (tokens,
/// part-of-speech, etc).
#[derive(Default)]
pub struct LayerLookups(EnumMap<Layer, BoxedLookup>);

impl LayerLookups {
    pub fn new() -> Self {
        LayerLookups(EnumMap::new())
    }

    pub fn insert<L>(&mut self, layer: Layer, lookup: L)
    where
        L: Into<Box<dyn Lookup>>,
    {
        self.0[layer] = BoxedLookup::new(lookup)
    }

    /// Get the lookup for a layer.
    pub fn layer_lookup(&self, layer: Layer) -> Option<&dyn Lookup> {
        self.0[layer].as_ref()
    }
}

/// Vectorizer for parser states.
///
/// An `InputVectorizer` vectorizes parser states.
pub struct InputVectorizer {
    layer_lookups: LayerLookups,
    input_layer_addrs: AddressedValues,
}

impl InputVectorizer {
    /// Construct an input vectorizer.
    ///
    /// The vectorizer is constructed from the layer lookups and the parser
    /// state addresses from which the feature vector should be used. The layer
    /// lookups are used to find the indices that represent the features.
    pub fn new(layer_lookups: LayerLookups, input_layer_addrs: AddressedValues) -> Self {
        InputVectorizer {
            layer_lookups,
            input_layer_addrs,
        }
    }

    pub fn embedding_layer_size(&self) -> usize {
        let mut size = 0;

        for layer in &self.input_layer_addrs.0 {
            if let Some(lookup) = self.layer_lookups.0[(&layer.layer).into()].as_ref() {
                match lookup.lookup_type() {
                    LookupType::Embedding(dims) => size += dims,
                    LookupType::Index => (),
                }
            }
        }

        size
    }

    pub fn layer_addrs(&self) -> &AddressedValues {
        &self.input_layer_addrs
    }

    /// Get the layer lookups.
    pub fn layer_lookups(&self) -> &LayerLookups {
        &self.layer_lookups
    }

    pub fn lookup_layer_sizes(&self) -> EnumMap<Layer, usize> {
        let mut sizes = EnumMap::new();

        for layer in &self.input_layer_addrs.0 {
            if let Some(lookup) = self.layer_lookups.0[(&layer.layer).into()].as_ref() {
                match lookup.lookup_type() {
                    LookupType::Embedding(_) => (),
                    LookupType::Index => sizes[(&layer.layer).into()] += 1,
                }
            }
        }

        sizes
    }

    /// Vectorize a parser state.
    pub fn realize(&self, state: &ParserState<'_>) -> InputVector {
        let mut embed_layer = Tensor::new(&[self.embedding_layer_size() as u64]);

        let mut lookup_layers = EnumMap::new();
        for (layer, &size) in &self.lookup_layer_sizes() {
            lookup_layers[layer] = vec![0; size];
        }

        self.realize_into(state, &mut embed_layer, &mut lookup_layers);

        InputVector {
            embed_layer,
            lookup_layers,
        }
    }

    /// Vectorize a parser state into the given slices.
    pub fn realize_into<S>(
        &self,
        state: &ParserState<'_>,
        embed_layer: &mut [f32],
        lookup_slices: &mut EnumMap<Layer, S>,
    ) where
        S: AsMut<[i32]>,
    {
        let mut embed_offset = 0;
        let mut layer_offsets: EnumMap<Layer, usize> = EnumMap::new();

        for layer in &self.input_layer_addrs.0 {
            let val = layer.get(state);
            let offset = &mut layer_offsets[(&layer.layer).into()];

            let layer = &layer.layer;

            match lookup_value(
                self.layer_lookups
                    .layer_lookup(layer.into())
                    .unwrap_or_else(|| panic!("Missing layer lookup for: {:?}", layer)),
                val,
            ) {
                LookupResult::Embedding(embed) => {
                    embed_layer[embed_offset..embed_offset + embed.len()]
                        .copy_from_slice(embed.as_slice().expect("Embedding is not contiguous"));
                    embed_offset += embed.len();
                }
                LookupResult::Index(idx) => {
                    lookup_slices[layer.into()].as_mut()[*offset] = idx as i32;
                    *offset += 1;
                }
            }
        }
    }
}

fn lookup_value<'a>(lookup: &'a dyn Lookup, feature: Option<Cow<'_, str>>) -> LookupResult<'a> {
    match feature {
        Some(f) => lookup
            .lookup(f.as_ref())
            .unwrap_or_else(|| lookup.unknown()),
        None => lookup.null(),
    }
}
