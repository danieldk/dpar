use std::hash::Hash;
use std::io::{Read, Write};

use dpar::features::LookupTable;
use dpar::system::TransitionSystem;
use dpar::systems::{
    ArcEagerSystem, ArcHybridSystem, ArcStandardSystem, StackProjectiveSystem, StackSwapSystem,
};
use dpar::Numberer;
use failure::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_cbor;
use toml;

use super::Config;

pub trait TomlRead {
    fn from_toml_read<R>(read: R) -> Result<Config, Error>
    where
        R: Read;
}

impl TomlRead for Config {
    fn from_toml_read<R>(mut read: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut data = String::new();
        read.read_to_string(&mut data)?;
        let config: Config = toml::from_str(&data)?;
        Ok(config)
    }
}

pub trait CborRead {
    type Value;

    fn from_cbor_read<R>(read: R) -> Result<Self::Value, Error>
    where
        R: Read;
}

macro_rules! cbor_read {
    ($type:ty) => {
        impl CborRead for $type {
            type Value = $type;

            fn from_cbor_read<R>(read: R) -> Result<$type, Error>
            where
                R: Read,
            {
                let system = serde_cbor::from_reader(read)?;
                Ok(system)
            }
        }
    };
}

cbor_read!(ArcEagerSystem);
cbor_read!(ArcHybridSystem);
cbor_read!(ArcStandardSystem);
cbor_read!(StackProjectiveSystem);
cbor_read!(StackSwapSystem);
cbor_read!(LookupTable);

pub trait CborWrite {
    fn to_cbor_write<W>(&self, write: &mut W) -> Result<(), Error>
    where
        W: Write;
}

impl<T> CborWrite for Numberer<T>
where
    T: Eq + Hash + Serialize + DeserializeOwned,
{
    fn to_cbor_write<W>(&self, write: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        let data = serde_cbor::to_vec(self)?;
        write.write_all(&data)?;
        Ok(())
    }
}

macro_rules! cbor_write {
    ($type:ty) => {
        impl CborWrite for $type {
            fn to_cbor_write<W>(&self, write: &mut W) -> Result<(), Error>
            where
                W: Write,
            {
                let data = serde_cbor::to_vec(self)?;
                write.write_all(&data)?;
                Ok(())
            }
        }
    };
}

cbor_write!(ArcEagerSystem);
cbor_write!(ArcHybridSystem);
cbor_write!(ArcStandardSystem);
cbor_write!(StackProjectiveSystem);
cbor_write!(StackSwapSystem);
cbor_write!(LookupTable);

pub trait SerializableTransitionSystem:
    Default + TransitionSystem + CborRead<Value = Self> + CborWrite
{
}

impl<T> SerializableTransitionSystem for T where
    T: Default + TransitionSystem + CborRead<Value = T> + CborWrite
{
}
