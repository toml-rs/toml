//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents
//! into Rust structures. Note that some top-level functions here are also
//! provided at the top of the crate.

#[cfg(feature = "parse")]
mod detable;
#[cfg(feature = "parse")]
mod devalue;
mod error;
#[cfg(feature = "parse")]
mod value;

#[cfg(feature = "parse")]
pub use detable::DeTable;
#[cfg(feature = "parse")]
pub use devalue::DeArray;
#[cfg(feature = "parse")]
pub use devalue::DeString;
#[cfg(feature = "parse")]
pub use devalue::DeValue;

pub use error::Error;
#[cfg(feature = "parse")]
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
    T::deserialize(Deserializer::new(s))
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
    use serde::de::Error as _;
    let s = std::str::from_utf8(s).map_err(|e| Error::new(crate::edit::de::Error::custom(e)))?;
    from_str(s)
}

/// Deserialization TOML document
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
#[cfg(feature = "parse")]
pub struct Deserializer<'i> {
    input: &'i str,
}

#[cfg(feature = "parse")]
impl<'i> Deserializer<'i> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'i str) -> Self {
        Self { input }
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner.deserialize_option(visitor).map_err(Error::new)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_newtype_struct(name, visitor)
            .map_err(Error::new)
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
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_struct(name, fields, visitor)
            .map_err(Error::new)
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
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_enum(name, variants, visitor)
            .map_err(Error::new)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}
