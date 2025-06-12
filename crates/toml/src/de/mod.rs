//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents
//! into Rust structures. Note that some top-level functions here are also
//! provided at the top of the crate.

#[cfg(feature = "parse")]
mod array;
#[cfg(feature = "parse")]
mod datetime;
#[cfg(feature = "parse")]
mod dearray;
#[cfg(feature = "parse")]
mod detable;
#[cfg(feature = "parse")]
mod devalue;
mod error;
#[cfg(feature = "parse")]
mod key;
#[cfg(feature = "parse")]
mod parser;
#[cfg(feature = "parse")]
mod spanned;
#[cfg(feature = "parse")]
mod table;
#[cfg(feature = "parse")]
mod table_enum;
#[cfg(feature = "parse")]
mod value;

#[cfg(feature = "parse")]
pub use dearray::DeArray;
#[cfg(feature = "parse")]
pub use detable::DeTable;
#[cfg(feature = "parse")]
pub use devalue::DeString;
#[cfg(feature = "parse")]
pub use devalue::DeValue;
#[cfg(feature = "parse")]
pub use value::ValueDeserializer;

pub use error::Error;

#[cfg(feature = "parse")]
use array::ArrayDeserializer;
#[cfg(feature = "parse")]
use datetime::DatetimeDeserializer;
#[cfg(feature = "parse")]
use key::KeyDeserializer;
#[cfg(feature = "parse")]
use spanned::SpannedDeserializer;
#[cfg(feature = "parse")]
use table::TableDeserializer;
#[cfg(feature = "parse")]
use table_enum::TableEnumDeserializer;

/// Deserializes a string into a type.
///
/// This function will attempt to interpret `s` as a TOML document and
/// deserialize `T` from the document.
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: Owner,
/// }
///
/// #[derive(Deserialize)]
/// struct Owner {
///     name: String,
/// }
///
/// let config: Config = toml::from_str(r#"
///     title = 'TOML Example'
///
///     [owner]
///     name = 'Lisa'
/// "#).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub fn from_str<T>(s: &'_ str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(Deserializer::parse(s)?)
}

/// Deserializes bytes into a type.
///
/// This function will attempt to interpret `s` as a TOML document and
/// deserialize `T` from the document.
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
#[cfg(feature = "parse")]
pub fn from_slice<T>(s: &'_ [u8]) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let s = std::str::from_utf8(s).map_err(|e| Error::custom(e.to_string(), None))?;
    from_str(s)
}

/// Deserialization for TOML [documents][crate::Table].
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
#[cfg(feature = "parse")]
pub struct Deserializer<'i> {
    span: Option<std::ops::Range<usize>>,
    root: DeTable<'i>,
    raw: Option<&'i str>,
}

#[cfg(feature = "parse")]
impl<'i> Deserializer<'i> {
    /// Parse a TOML document
    pub fn parse(raw: &'i str) -> Result<Self, Error> {
        let root = DeTable::parse(raw)?;
        let span = Some(root.span());
        let root = root.into_inner();
        Ok(Self {
            span,
            root,
            raw: Some(raw),
        })
    }

    fn into_table_de(self) -> ValueDeserializer<'i> {
        ValueDeserializer::new(DeValue::Table(self.root), self.span)
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let raw = self.raw;
        self.into_table_de()
            .deserialize_any(visitor)
            .map_err(|mut e: Self::Error| {
                e.set_raw(raw.map(|r| r.into()));
                e
            })
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let raw = self.raw;
        self.into_table_de()
            .deserialize_option(visitor)
            .map_err(|mut e: Self::Error| {
                e.set_raw(raw.map(|r| r.into()));
                e
            })
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let raw = self.raw;
        self.into_table_de()
            .deserialize_newtype_struct(name, visitor)
            .map_err(|mut e: Self::Error| {
                e.set_raw(raw.map(|r| r.into()));
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
        let raw = self.raw;
        self.into_table_de()
            .deserialize_struct(name, fields, visitor)
            .map_err(|mut e: Self::Error| {
                e.set_raw(raw.map(|r| r.into()));
                e
            })
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
        let raw = self.raw;
        self.into_table_de()
            .deserialize_enum(name, variants, visitor)
            .map_err(|mut e: Self::Error| {
                e.set_raw(raw.map(|r| r.into()));
                e
            })
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::de::IntoDeserializer<'de, Error> for Deserializer<'de> {
    type Deserializer = Deserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

#[cfg(feature = "parse")]
pub(crate) fn validate_struct_keys(
    table: &DeTable<'_>,
    fields: &'static [&'static str],
) -> Result<(), Error> {
    let extra_fields = table
        .keys()
        .filter_map(|key| {
            if !fields.contains(&key.get_ref().as_ref()) {
                Some(key.clone())
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
                extra_fields
                    .iter()
                    .map(|k| k.get_ref().as_ref())
                    .collect::<Vec<_>>()
                    .join(", "),
                fields.join(", "),
            ),
            Some(extra_fields[0].span()),
        ))
    }
}
