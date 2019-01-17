use std::borrow::Cow;
use std::cell::RefCell;

use tensorflow::Tensor;
use tf_embed::Embeddings;

use Numberer;

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
pub trait Lookup {
    /// Get the embedding matrix for the data type.
    ///
    /// Returns `None` if the lookup does not have an associated embedding
    /// matrix.
    fn embed_matrix(&self) -> Option<&Tensor<f32>>;

    // Size of the table.
    fn len(&self) -> usize;

    /// Lookup a feature index.
    fn lookup(&self, feature: &str) -> Option<usize>;

    /// Lookup features.
    fn lookup_values<'a>(&'a self) -> Cow<'a, [String]>;

    /// Null value.
    fn null(&self) -> usize;

    // Unknown value.
    fn unknown(&self) -> usize;
}

impl Lookup for Embeddings {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        Some(self.data())
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn lookup(&self, feature: &str) -> Option<usize> {
        self.indices().get(feature).cloned()
    }

    fn lookup_values<'a>(&'a self) -> Cow<'a, [String]> {
        Cow::Owned(self.words().to_owned())
    }

    fn null(&self) -> usize {
        self.indices()[NULL_TOKEN]
    }

    fn unknown(&self) -> usize {
        self.indices()[UNKNOWN_TOKEN]
    }
}

pub struct MutableLookupTable {
    numberer: RefCell<Numberer<String>>,
}

impl MutableLookupTable {
    pub fn new() -> Self {
        MutableLookupTable {
            numberer: RefCell::new(Numberer::new(1)),
        }
    }
}

impl Lookup for MutableLookupTable {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        None
    }

    fn len(&self) -> usize {
        self.numberer.borrow().len() + 1
    }

    fn lookup(&self, feature: &str) -> Option<usize> {
        let mut numberer = self.numberer.borrow_mut();
        Some(numberer.add(feature.to_owned()))
    }

    fn lookup_values<'a>(&'a self) -> Cow<'a, [String]> {
        Cow::Owned(self.numberer.borrow().values().to_owned())
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

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct LookupTable {
    numberer: Numberer<String>,
}

impl Lookup for LookupTable {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        None
    }

    fn len(&self) -> usize {
        self.numberer.len() + 1
    }

    fn lookup(&self, feature: &str) -> Option<usize> {
        self.numberer.number(feature)
    }

    fn lookup_values<'a>(&'a self) -> Cow<'a, [String]> {
        Cow::Owned(self.numberer.values().to_owned())
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

impl From<MutableLookupTable> for LookupTable {
    fn from(t: MutableLookupTable) -> Self {
        LookupTable {
            numberer: t.numberer.into_inner(),
        }
    }
}

pub struct BoxedLookup(Option<Box<Lookup>>);

impl BoxedLookup {
    /// Construct a boxed lookup from a lookup.
    pub fn new<L>(lookup: L) -> Self
    where
        L: Into<Box<Lookup>>,
    {
        BoxedLookup(Some(lookup.into()))
    }

    /// Get the lookup as a reference.
    pub fn as_ref(&self) -> Option<&Lookup> {
        self.0.as_ref().map(AsRef::as_ref)
    }
}

impl Default for BoxedLookup {
    fn default() -> Self {
        BoxedLookup(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lookup, LookupTable, MutableLookupTable};

    #[test]
    fn mutable_lookup_table_test() {
        let table = MutableLookupTable::new();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.null(), 0);
        assert_eq!(table.unknown(), 0);
        assert!(table.embed_matrix().is_none());
    }

    #[test]
    fn lookup_table_test() {
        let table = MutableLookupTable::new();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));

        let table: LookupTable = table.into();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));
        assert_eq!(table.lookup("c"), None);
        assert_eq!(table.null(), 0);
        assert_eq!(table.unknown(), 0);
        assert!(table.embed_matrix().is_none());
    }
}
