use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem;
use std::path::Path;

use dpar::features::FeatureLookup;
use dpar::lookup::{FreezableLookup, LookupLen, LookupValue};
use failure::Error;
use tensorflow::Tensor;

use {CborRead, CborWrite};

pub struct StoredLookupTable {
    table: FreezableLookup<String>,
    write: Option<Box<Write>>,
}

impl StoredLookupTable {
    pub fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let f = File::open(path)?;
        Ok(StoredLookupTable {
            table: FreezableLookup::from_cbor_read(f)?,
            write: None,
        })
    }

    pub fn create<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let f = File::create(path)?;
        let write = BufWriter::new(f);
        Ok(StoredLookupTable {
            table: FreezableLookup::default(),
            write: Some(Box::new(write)),
        })
    }
}

impl Drop for StoredLookupTable {
    fn drop(&mut self) {
        if let Some(ref mut write) = &mut self.write {
            // We want to serialize an immutable table. So, freeze the table
            // and serialize the frozen table.
            let mut table = mem::replace(&mut self.table, FreezableLookup::default());
            let table = table.freeze();

            if let Err(err) = table.to_cbor_write(write) {
                eprintln!("Error writing lookup table: {}", err);
            }
        }
    }
}

impl LookupLen for StoredLookupTable {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl LookupValue<str> for StoredLookupTable {
    fn lookup(&self, v: &str) -> usize {
        self.table.lookup(v)
    }

    fn value(&self, id: usize) -> Option<Cow<str>> {
        self.table.value(id)
    }
}

impl FeatureLookup for StoredLookupTable {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        self.table.embed_matrix()
    }

    fn null(&self) -> usize {
        self.table.null()
    }

    fn unknown(&self) -> usize {
        self.table.unknown()
    }
}
