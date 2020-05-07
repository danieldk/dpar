use std::cell::RefCell;
use std::ops::Deref;

use finalfusion::embeddings::Embeddings as FFEmbeddings;
use finalfusion::storage::StorageWrap;
use finalfusion::vocab::{Vocab, VocabWrap};
use ndarray::{Array1, CowArray, Ix1};
use serde_derive::{Deserialize, Serialize};

use crate::Numberer;

pub enum LookupResult<'a> {
    Index(usize),
    Embedding(CowArray<'a, f32, Ix1>),
}

impl<'a> LookupResult<'a> {
    pub fn index(&self) -> Option<usize> {
        use self::LookupResult::*;

        match self {
            Index(idx) => Some(*idx),
            Embedding(_) => None,
        }
    }
}

/// The type of lookup.
pub enum LookupType {
    /// Lookup that returns indices.
    Index,

    /// Lookup that returns embeddings with the given size.
    Embedding(usize),
}

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
#[allow(clippy::len_without_is_empty)]
pub trait Lookup {
    /// Size of the table.
    fn len(&self) -> usize;

    /// Lookup a feature.
    fn lookup(&self, feature: &str) -> Option<LookupResult<'_>>;

    /// Lookup type.
    fn lookup_type(&self) -> LookupType;

    /// Null value.
    fn null(&self) -> LookupResult<'_>;

    // Unknown value.
    fn unknown(&self) -> LookupResult<'_>;
}

pub struct Embeddings {
    inner: FFEmbeddings<VocabWrap, StorageWrap>,
    null: Array1<f32>,
    unknown: Array1<f32>,
}

impl Deref for Embeddings {
    type Target = FFEmbeddings<VocabWrap, StorageWrap>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<FFEmbeddings<VocabWrap, StorageWrap>> for Embeddings {
    fn from(embeddings: FFEmbeddings<VocabWrap, StorageWrap>) -> Self {
        let null = Array1::zeros(embeddings.dims());

        // Compute the average embedding.
        let mut unknown = Array1::zeros(embeddings.dims());
        for (_, embed) in &embeddings {
            unknown += &embed;
        }
        let l2norm = unknown.dot(&unknown).sqrt();
        if l2norm != 0f32 {
            unknown /= l2norm;
        }

        Embeddings {
            inner: embeddings,
            null,
            unknown,
        }
    }
}

impl Lookup for Embeddings {
    fn len(&self) -> usize {
        self.vocab().words_len()
    }

    fn lookup(&self, feature: &str) -> Option<LookupResult<'_>> {
        self.embedding(feature).map(LookupResult::Embedding)
    }

    fn lookup_type(&self) -> LookupType {
        LookupType::Embedding(self.dims())
    }

    fn null(&self) -> LookupResult<'_> {
        LookupResult::Embedding(CowArray::from(self.null.view()))
    }

    fn unknown(&self) -> LookupResult<'_> {
        LookupResult::Embedding(CowArray::from(self.unknown.view()))
    }
}

pub struct MutableLookupTable(RefCell<Numberer<String>>);

impl MutableLookupTable {
    pub fn new() -> Self {
        MutableLookupTable(RefCell::new(Numberer::new(1)))
    }
}

impl Default for MutableLookupTable {
    fn default() -> Self {
        Self::new()
    }
}

impl Lookup for MutableLookupTable {
    fn len(&self) -> usize {
        self.0.borrow().len() + 1
    }

    fn lookup(&self, feature: &str) -> Option<LookupResult<'_>> {
        let mut numberer = self.0.borrow_mut();
        Some(LookupResult::Index(numberer.add(feature.to_owned())))
    }

    fn lookup_type(&self) -> LookupType {
        LookupType::Index
    }

    fn null(&self) -> LookupResult<'_> {
        LookupResult::Index(0)
    }

    fn unknown(&self) -> LookupResult<'_> {
        LookupResult::Index(0)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct LookupTable(Numberer<String>);

impl From<MutableLookupTable> for LookupTable {
    fn from(t: MutableLookupTable) -> Self {
        LookupTable(t.0.into_inner())
    }
}

impl Lookup for LookupTable {
    fn len(&self) -> usize {
        self.0.len() + 1
    }

    fn lookup(&self, feature: &str) -> Option<LookupResult<'_>> {
        self.0.number(feature).map(LookupResult::Index)
    }

    fn lookup_type(&self) -> LookupType {
        LookupType::Index
    }

    fn null(&self) -> LookupResult<'_> {
        LookupResult::Index(0)
    }

    fn unknown(&self) -> LookupResult<'_> {
        LookupResult::Index(0)
    }
}

pub struct BoxedLookup(Option<Box<dyn Lookup>>);

impl BoxedLookup {
    /// Construct a boxed lookup from a lookup.
    pub fn new<L>(lookup: L) -> Self
    where
        L: Into<Box<dyn Lookup>>,
    {
        BoxedLookup(Some(lookup.into()))
    }

    /// Get the lookup as a reference.
    pub fn as_ref(&self) -> Option<&dyn Lookup> {
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
        assert_eq!(table.lookup("a").unwrap().index(), Some(1));
        assert_eq!(table.lookup("b").unwrap().index(), Some(2));
        assert_eq!(table.lookup("a").unwrap().index(), Some(1));
        assert_eq!(table.null().index(), Some(0));
        assert_eq!(table.unknown().index(), Some(0));
    }

    #[test]
    fn lookup_table_test() {
        let table = MutableLookupTable::new();
        assert_eq!(table.lookup("a").unwrap().index(), Some(1));
        assert_eq!(table.lookup("b").unwrap().index(), Some(2));

        let table: LookupTable = table.into();
        assert_eq!(table.lookup("a").unwrap().index(), Some(1));
        assert_eq!(table.lookup("a").unwrap().index(), Some(1));
        assert_eq!(table.lookup("b").unwrap().index(), Some(2));
        assert!(table.lookup("c").is_none());
        assert_eq!(table.null().index(), Some(0));
        assert_eq!(table.unknown().index(), Some(0));
    }
}
