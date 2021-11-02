//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents into Rust structures.

use itertools::Itertools;
use serde::Deserialize;

mod array;
mod inline_table;
mod item;
mod table;
mod table_enum;
mod value;

use array::*;
use inline_table::*;
use item::*;
use table::*;
use table_enum::*;

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
    let deserializer = Deserializer::new(d);
    T::deserialize(deserializer)
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

impl<'de, 'a> serde::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_any(visitor)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_option(visitor)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_enum(name, variants, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit newtype_struct
        ignored_any unit_struct tuple_struct tuple identifier struct
    }
}

impl<'de, 'a> serde::Deserializer<'de> for crate::Document {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.root.deserialize_any(visitor)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.root.deserialize_option(visitor)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.root.deserialize_enum(name, variants, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit newtype_struct
        ignored_any unit_struct tuple_struct tuple identifier struct
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Document {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub(crate) fn validate_struct_keys(
    table: &crate::table::KeyValuePairs,
    fields: &'static [&'static str],
) -> Result<(), Error> {
    let extra_fields = table
        .iter()
        .filter_map(|(key, _val)| {
            if !fields.contains(&key.as_str()) {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if extra_fields.is_empty() {
        Ok(())
    } else {
        Err(Error::custom(format!(
            "unexpected keys in table: {}, available keys: {}",
            extra_fields.iter().join(", "),
            fields.iter().join(", "),
        )))
    }
}
