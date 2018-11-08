use std::borrow::Cow;

use tensorflow::Tensor;
use tf_embed;

use lookup::{FreezableLookup, Lookup, LookupLen, LookupValue};

static NULL_TOKEN: &'static str = "<NULL-TOKEN>";

static UNKNOWN_TOKEN: &'static str = "<UNKNOWN-TOKEN>";

/// Trait for feature index lookup.
///
/// In dpar, the parser state is encoded as a vector of integers. The integers
/// are offsets into an embedding matrix. However, one might want to use two
/// different kinds of embeddings:
///
/// 1. Pretrained embeddings: the vector should use the offset of the datum
///    in an existing embedding matrix.
/// 2. Embeddings trained through backprop: the vector should generate an
///    offset for the datum during training.
///
/// To accommodate for these different representation, this trait is provided,
/// so that parser state vectorizers can be agnostic to where the indices come
/// from.
pub trait FeatureLookup: Lookup<str> {
    /// Get the embedding matrix for the data type.
    ///
    /// Returns `None` if the lookup does not have an associated embedding
    /// matrix.
    fn embed_matrix(&self) -> Option<&Tensor<f32>>;

    /// Null value.
    fn null(&self) -> usize;

    // Unknown value.
    fn unknown(&self) -> usize;
}

/// An embedding lookup table.
pub struct Embeddings(tf_embed::Embeddings);

impl LookupLen for Embeddings {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl LookupValue<str> for Embeddings {
    fn lookup(&self, feature: &str) -> usize {
        match self.0.indices().get(feature) {
            Some(idx) => *idx,
            None => self.unknown(),
        }
    }

    fn value(&self, id: usize) -> Option<Cow<str>> {
        self.0.words().get(id).map(|s| Cow::Borrowed(s.as_ref()))
    }
}

impl FeatureLookup for Embeddings {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        Some(self.0.data())
    }

    fn null(&self) -> usize {
        self.0.indices()[NULL_TOKEN]
    }

    fn unknown(&self) -> usize {
        self.0.indices()[UNKNOWN_TOKEN]
    }
}

impl From<tf_embed::Embeddings> for Embeddings {
    fn from(embeds: tf_embed::Embeddings) -> Self {
        Embeddings(embeds)
    }
}

impl FeatureLookup for FreezableLookup<String> {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        None
    }

    fn null(&self) -> usize {
        0
    }

    fn unknown(&self) -> usize {
        // No better possible value until we mark low-frequency tokens,
        // features, etc. as unknown. Luckily, unknowns do not really
        // happen
        0
    }
}

pub struct BoxedFeatureLookup(Option<Box<FeatureLookup>>);

impl BoxedFeatureLookup {
    /// Construct a boxed lookup from a lookup.
    pub fn new<L>(lookup: L) -> Self
    where
        L: Into<Box<FeatureLookup>>,
    {
        BoxedFeatureLookup(Some(lookup.into()))
    }

    /// Get the lookup as a reference.
    pub fn as_ref(&self) -> Option<&FeatureLookup> {
        self.0.as_ref().map(AsRef::as_ref)
    }
}

impl Default for BoxedFeatureLookup {
    fn default() -> Self {
        BoxedFeatureLookup(None)
    }
}

#[cfg(test)]
mod tests {
    use lookup::{FreezableLookup, LookupValue};

    use super::FeatureLookup;

    #[test]
    fn mutable_lookup_table_test() {
        let table = FreezableLookup::default();
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.lookup("b"), 2);
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.null(), 0);
        assert_eq!(table.unknown(), 0);
        assert!(table.embed_matrix().is_none());
    }

    #[test]
    fn lookup_table_test() {
        let table = FreezableLookup::default();
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.lookup("b"), 2);

        let table: FreezableLookup<String> = table.freeze();
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.lookup("a"), 1);
        assert_eq!(table.lookup("b"), 2);
        assert_eq!(table.lookup("c"), 0);
        assert_eq!(table.null(), 0);
        assert_eq!(table.unknown(), 0);
        assert!(table.embed_matrix().is_none());
    }
}
