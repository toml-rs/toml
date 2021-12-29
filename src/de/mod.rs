//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents into Rust structures.

use itertools::Itertools;
use serde::de::DeserializeOwned;

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
    inner: Box<ErrorInner>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ErrorInner {
    message: String,
    reverse_key: Vec<crate::InternalString>,
}

impl Error {
    pub(crate) fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            inner: Box::new(ErrorInner {
                message: msg.to_string(),
                reverse_key: Default::default(),
            }),
        }
    }

    pub(crate) fn parent_key(&mut self, key: crate::InternalString) {
        self.inner.reverse_key.push(key);
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::custom(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.message.fmt(f)?;

        if !self.inner.reverse_key.is_empty() {
            write!(f, " for key `")?;
            for (i, k) in self.inner.reverse_key.iter().rev().enumerate() {
                if i > 0 {
                    write!(f, ".")?;
                }
                write!(f, "{}", k)?;
            }
            write!(f, "`")?;
        }

        Ok(())
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
    T: DeserializeOwned,
{
    let d = s.parse::<crate::Document>()?;
    from_document(d)
}

/// Convert a value into `T`.
pub fn from_slice<T>(s: &'_ [u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let s = std::str::from_utf8(s).map_err(Error::custom)?;
    from_str(s)
}

/// Convert a document into `T`.
pub fn from_document<T>(d: crate::Document) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let deserializer = Deserializer::new(d);
    T::deserialize(deserializer)
}

/// Convert an item into `T`.
pub fn from_item<T>(d: crate::Item) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(d)
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
