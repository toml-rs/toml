//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents into Rust structures.

use serde::de::DeserializeOwned;

mod array;
mod datetime;
mod error;
mod key;
mod spanned;
mod table;
mod table_enum;
mod value;

use array::ArrayDeserializer;
use datetime::DatetimeDeserializer;
use key::KeyDeserializer;
use spanned::SpannedDeserializer;
use table_enum::TableEnumDeserializer;

pub use error::Error;
pub use value::ValueDeserializer;

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
/// let config: Config = toml_edit::de::from_str(r#"
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
    T: DeserializeOwned,
{
    let de = Deserializer::parse(s)?;
    T::deserialize(de)
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
    T: DeserializeOwned,
{
    let s = std::str::from_utf8(s).map_err(|e| Error::custom(e, None))?;
    from_str(s)
}

/// Convert a [`DocumentMut`][crate::DocumentMut] into `T`.
pub fn from_document<T>(d: impl Into<Deserializer>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let deserializer = d.into();
    T::deserialize(deserializer)
}

/// Deserialization for TOML [documents][crate::DocumentMut].
pub struct Deserializer<S = String> {
    root: crate::Item,
    raw: Option<S>,
}

#[cfg(feature = "parse")]
impl<S: AsRef<str>> Deserializer<S> {
    /// Parse a TOML document
    pub fn parse(raw: S) -> Result<Self, Error> {
        crate::Document::parse(raw)
            .map(Self::from)
            .map_err(Into::into)
    }
}

impl From<crate::DocumentMut> for Deserializer {
    fn from(doc: crate::DocumentMut) -> Self {
        let crate::DocumentMut { root, .. } = doc;
        Self { root, raw: None }
    }
}

impl<S> From<crate::Document<S>> for Deserializer<S> {
    fn from(doc: crate::Document<S>) -> Self {
        let crate::Document { root, raw, .. } = doc;
        let raw = Some(raw);
        Self { root, raw }
    }
}

#[cfg(feature = "parse")]
impl std::str::FromStr for Deserializer {
    type Err = Error;

    /// Parses a document from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc: crate::Document<_> = s.parse().map_err(Error::from)?;
        Ok(Deserializer::from(doc))
    }
}

// Note: this is wrapped by `toml::de::Deserializer` and any trait methods
// implemented here need to be wrapped there
impl<'de, S: Into<String>> serde::Deserializer<'de> for Deserializer<S> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let raw = self.raw;
        self.root
            .into_deserializer()
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
        self.root
            .into_deserializer()
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
        self.root
            .into_deserializer()
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
        self.root
            .into_deserializer()
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
        self.root
            .into_deserializer()
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

impl serde::de::IntoDeserializer<'_, Error> for Deserializer {
    type Deserializer = Deserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl serde::de::IntoDeserializer<'_, Error> for crate::DocumentMut {
    type Deserializer = Deserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::from(self)
    }
}

impl serde::de::IntoDeserializer<'_, Error> for crate::Document<String> {
    type Deserializer = Deserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::from(self)
    }
}

pub(crate) fn validate_struct_keys(
    table: &crate::table::KeyValuePairs,
    fields: &'static [&'static str],
) -> Result<(), Error> {
    let extra_fields = table
        .keys()
        .filter_map(|key| {
            if !fields.contains(&key.get()) {
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
                    .map(|k| k.get())
                    .collect::<Vec<_>>()
                    .join(", "),
                fields.join(", "),
            ),
            extra_fields[0].span(),
        ))
    }
}
