//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents into Rust structures.

use serde::Deserialize;

mod array;
mod array_of_tables;
mod inline_table;
mod item;
mod table;
mod value;

use array::*;
use array_of_tables::*;
use inline_table::*;
use item::*;
use table::*;
use value::*;

/// Errors that can occur when deserializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    pub(crate) fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            message: msg.to_string(),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}

impl From<crate::TomlError> for Error {
    fn from(e: crate::TomlError) -> Error {
        Self::custom(e)
    }
}

impl From<Error> for crate::TomlError {
    fn from(e: Error) -> crate::TomlError {
        Self::custom(e.to_string())
    }
}

impl std::error::Error for Error {}

/// Convert a value into `T`.
pub fn from_str<T>(s: &'_ str) -> Result<T, Error>
where
    T: Deserialize<'static>,
{
    let d = s.parse::<crate::Document>()?;
    from_document(d)
}

/// Convert a value into `T`.
pub fn from_slice<T>(s: &'_ [u8]) -> Result<T, Error>
where
    T: Deserialize<'static>,
{
    let s = std::str::from_utf8(s).map_err(Error::custom)?;
    from_str(s)
}

/// Convert a value into `T`.
pub fn from_document<T>(d: crate::Document) -> Result<T, Error>
where
    T: Deserialize<'static>,
{
    let mut deserializer = Deserializer::new(d);
    T::deserialize(&mut deserializer)
}

/// Deserialization implementation for TOML.
pub struct Deserializer {
    input: crate::Document,
}

impl Deserializer {
    /// Deserialization implementation for TOML.
    pub fn new(input: crate::Document) -> Self {
        Self { input }
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut input = Default::default();
        std::mem::swap(&mut input, &mut self.input.root);
        ItemDeserializer::new(input).deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}
