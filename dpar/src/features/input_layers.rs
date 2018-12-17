use std::borrow::Cow;
use std::fmt;
use std::io::BufRead;
use std::result;

use enum_map::EnumMap;
use failure::Error;

use features::addr;
use features::lookup::BoxedLookup;
use features::parse_addr::parse_addressed_values;
use features::Lookup;
use system::ParserState;

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
    pub layers: EnumMap<Layer, Vec<i32>>,
}

impl InputVector {
    /// Construct a new input vector.
    pub fn new() -> Self {
        InputVector {
            layers: EnumMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum Layer {
    Token,
    Tag,
    DepRel,
    Feature,
    Char,
    Var
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s = match *self {
            Layer::Token => "tokens",
            Layer::Tag => "tags",
            Layer::DepRel => "deprels",
            Layer::Feature => "features",
            Layer::Char => "chars",
            Layer::Var => "various"
        };

        f.write_str(s)
    }
}

// I am not sure whether I like the use of Borrow here, but is there another
// convenient way to convert from both addr::Layer and &addr::Layer?
impl<'a> From<&'a addr::Layer> for Layer {
    fn from(layer: &addr::Layer) -> Self {
        match layer {
            &addr::Layer::Token => Layer::Token,
            &addr::Layer::Tag => Layer::Tag,
            &addr::Layer::DepRel => Layer::DepRel,
            &addr::Layer::Feature(_) => Layer::Feature,
            &addr::Layer::Char(_, _) => Layer::Char,
            &addr::Layer::Var => Layer::Var,
        }
    }
}

/// Lookups for layers.
///
/// This data structure bundles lookups for the different layers (tokens,
/// part-of-speech, etc).
pub struct LayerLookups(EnumMap<Layer, BoxedLookup>);

impl LayerLookups {
    pub fn new() -> Self {
        LayerLookups(EnumMap::new())
    }

    pub fn insert<L>(&mut self, layer: Layer, lookup: L)
    where
        L: Into<Box<Lookup>>,
    {
        self.0[layer] = BoxedLookup::new(lookup);
    }

    /// Get the lookup for a layer.
    pub fn layer_lookup(&self, layer: Layer) -> Option<&Lookup> {
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
    pub fn new(layer_lookups: LayerLookups, input_addrs: AddressedValues) -> Self {
        InputVectorizer {
            layer_lookups: layer_lookups,
            input_layer_addrs: input_addrs,
        }
    }

    pub fn layer_addrs(&self) -> &AddressedValues {
        &self.input_layer_addrs
    }

    /// Get the layer lookups.
    pub fn layer_lookups(&self) -> &LayerLookups {
        &self.layer_lookups
    }

    pub fn layer_sizes(&self) -> EnumMap<Layer, usize> {
        let mut sizes = EnumMap::new();

        for layer in &self.input_layer_addrs.0 {
            match layer.layer {
                addr::Layer::Char(prefix_len, suffix_len) => {
                    sizes[Layer::Char] += prefix_len + suffix_len
                }
                ref layer => sizes[layer.into()] += 1,
            }
        }

        sizes
    }

    /// Vectorize a parser state.
    pub fn realize(&self, state: &ParserState) -> InputVector {
        let mut layers = EnumMap::new();

        for (layer, &size) in &self.layer_sizes() {
            layers[layer] = vec![0; size];
        }

        self.realize_into(state, &mut layers);

        InputVector { layers }
    }

    /// Vectorize a parser state into the given slices.
    pub fn realize_into<S>(&self, state: &ParserState, slices: &mut EnumMap<Layer, S>)
    where
        S: AsMut<[i32]>,
    {
        let mut layer_offsets: EnumMap<Layer, usize> = EnumMap::new();

        for layer in &self.input_layer_addrs.0 {
            let val = layer.get(state);
            let mut offset = &mut layer_offsets[(&layer.layer).into()];

            match layer.layer {
                addr::Layer::Char(prefix_len, suffix_len) => match val {
                    Some(chars) => {
                        for ch in chars.as_ref().chars() {
                            slices[Layer::Char].as_mut()[*offset] = lookup_char(
                                self.layer_lookups
                                    .layer_lookup(Layer::Char)
                                    .expect("Missing layer lookup for: Char"),
                                ch,
                            );

                            *offset += 1;
                        }
                    }
                    None => {
                        let null_char = self
                            .layer_lookups
                            .layer_lookup(Layer::Char)
                            .expect("Missing layer lookup for: Char")
                            .null() as i32;
                        for _ in 0..(prefix_len + suffix_len) {
                            slices[Layer::Char].as_mut()[*offset] = null_char;
                            *offset += 1;
                        }
                    }
                },
                ref layer => {
                    slices[layer.into()].as_mut()[*offset] = lookup_value(
                        self.layer_lookups
                            .layer_lookup(layer.into())
                            .expect(&format!("Missing layer lookup for: {:?}", layer)),
                        val,
                    );
                    *offset += 1;
                }
            }
        }
    }
}

fn lookup_char(lookup: &Lookup, feature: char) -> i32 {
    if feature == '\0' {
        return lookup.null() as i32;
    }

    let feature = feature.to_string();
    if let Some(idx) = lookup.lookup(&feature) {
        idx as i32
    } else {
        lookup.unknown() as i32
    }
}

fn lookup_value(lookup: &Lookup, feature: Option<Cow<str>>) -> i32 {
    match feature {
        Some(f) => lookup.lookup(f.as_ref()).unwrap_or(lookup.unknown()) as i32,
        None => lookup.null() as i32,
    }
}
