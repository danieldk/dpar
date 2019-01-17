use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem;
use std::path::Path;

use dpar::features::{Lookup, LookupTable, MutableLookupTable};
use failure::Error;
use tensorflow::Tensor;

use {CborRead, CborWrite};

pub enum StoredLookupTable {
    Table(LookupTable),
    FreshTable {
        write: Box<Write>,
        table: MutableLookupTable,
    },
}

impl StoredLookupTable {
    pub fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let f = File::open(path)?;
        Ok(StoredLookupTable::Table(LookupTable::from_cbor_read(f)?))
    }

    pub fn create<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let f = File::create(path)?;
        let write = BufWriter::new(f);
        Ok(StoredLookupTable::FreshTable {
            write: Box::new(write),
            table: MutableLookupTable::new(),
        })
    }
}

impl Drop for StoredLookupTable {
    fn drop(&mut self) {
        if let StoredLookupTable::FreshTable {
            ref mut write,
            ref mut table,
        } = self
        {
            // We want to serialize an immutable table. So, get the inner MutableLookupTable,
            // convert it into a LookupTable.
            let table: LookupTable = mem::replace(table, MutableLookupTable::new()).into();

            if let Err(err) = table.to_cbor_write(write) {
                eprintln!("Error writing lookup table: {}", err);
            }
        }
    }
}

impl Lookup for StoredLookupTable {
    fn embed_matrix(&self) -> Option<&Tensor<f32>> {
        None
    }

    fn len(&self) -> usize {
        match self {
            StoredLookupTable::Table(ref table) => table.len(),
            StoredLookupTable::FreshTable { ref table, .. } => table.len(),
        }
    }

    fn lookup(&self, feature: &str) -> Option<usize> {
        match self {
            StoredLookupTable::Table(ref table) => table.lookup(feature),
            StoredLookupTable::FreshTable { ref table, .. } => table.lookup(feature),
        }
    }

    fn lookup_values<'a>(&'a self) -> Cow<'a, [String]> {
        match self {
            StoredLookupTable::Table(ref table) => table.lookup_values(),
            StoredLookupTable::FreshTable { ref table, .. } => table.lookup_values(),
        }
    }

    fn null(&self) -> usize {
        match self {
            StoredLookupTable::Table(ref table) => table.null(),
            StoredLookupTable::FreshTable { ref table, .. } => table.null(),
        }
    }

    fn unknown(&self) -> usize {
        match self {
            StoredLookupTable::Table(ref table) => table.unknown(),
            StoredLookupTable::FreshTable { ref table, .. } => table.unknown(),
        }
    }
}
