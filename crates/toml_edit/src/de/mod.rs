//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents into Rust structures.

use itertools::Itertools;
use serde::de::DeserializeOwned;

mod array;
mod inline_table;
mod item;
mod key;
mod spanned;
mod table;
mod table_enum;
mod value;

use array::*;
use inline_table::*;
use item::*;
use key::*;
use spanned::*;
use table::*;
use table_enum::*;

/// Errors that can occur when deserializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    inner: crate::TomlError,
}

impl Error {
    pub(crate) fn custom<T>(msg: T, span: Option<std::ops::Range<usize>>) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            inner: crate::TomlError::custom(msg.to_string(), span),
        }
    }

    /// The start/end index into the original document where the error occurred
    pub fn span(&self) -> Option<std::ops::Range<usize>> {
        self.inner.span()
    }

    pub(crate) fn set_span(&mut self, span: Option<std::ops::Range<usize>>) {
        self.inner.set_span(span);
    }

    /// Produces a (line, column) pair of the position of the error if available
    ///
    /// All indexes are 0-based.
    #[deprecated(since = "0.18.0", note = "See instead `Error::span`")]
    pub fn line_col(&self) -> Option<(usize, usize)> {
        #[allow(deprecated)]
        self.inner.line_col()
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::custom(msg, None)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<crate::TomlError> for Error {
    fn from(e: crate::TomlError) -> Error {
        Self { inner: e }
    }
}

impl From<Error> for crate::TomlError {
    fn from(e: Error) -> crate::TomlError {
        e.inner
    }
}

impl std::error::Error for Error {}

/// Convert a value into `T`.
pub fn from_str<T>(s: &'_ str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let d = crate::parser::parse_document(s)?;
    from_document(d)
}

/// Convert a value into `T`.
pub fn from_slice<T>(s: &'_ [u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let s = std::str::from_utf8(s).map_err(|e| Error::custom(e, None))?;
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

impl<'de> serde::Deserializer<'de> for Deserializer {
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

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_struct(name, fields, visitor)
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
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::Deserializer<'de> for crate::Document {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let original = self.original;
        self.root
            .deserialize_any(visitor)
            .map_err(|mut e: Self::Error| {
                e.inner.set_original(original);
                e
            })
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let original = self.original;
        self.root
            .deserialize_option(visitor)
            .map_err(|mut e: Self::Error| {
                e.inner.set_original(original);
                e
            })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if is_spanned(name, fields) {
            if let Some(span) = self.span() {
                return visitor.visit_map(SpannedDeserializer::new(self, span));
            }
        }

        self.deserialize_any(visitor)
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
        let original = self.original;
        self.root
            .deserialize_enum(name, variants, visitor)
            .map_err(|mut e: Self::Error| {
                e.inner.set_original(original);
                e
            })
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit newtype_struct
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Document {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

pub(crate) fn validate_struct_keys(
    table: &crate::table::KeyValuePairs,
    fields: &'static [&'static str],
) -> Result<(), Error> {
    let extra_fields = table
        .iter()
        .filter_map(|(key, val)| {
            if !fields.contains(&key.as_str()) {
                Some(val.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if extra_fields.is_empty() {
        Ok(())
    } else {
        Err(Error::custom(
            format!(
                "unexpected keys in table: {}, available keys: {}",
                extra_fields.iter().map(|k| k.key.get()).join(", "),
                fields.iter().join(", "),
            ),
            extra_fields[0].key.span(),
        ))
    }
}
