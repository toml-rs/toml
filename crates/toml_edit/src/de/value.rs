use serde::de::IntoDeserializer as _;

use crate::de::Error;

use super::item::ItemDeserializer;

/// Deserialization implementation for TOML [values][crate::Value].
///
/// Can be created either directly from TOML strings, using [`std::str::FromStr`],
/// or from parsed [values][crate::Value] using [`serde::de::IntoDeserializer::into_deserializer`].
///
/// # Example
///
/// ```
/// # #[cfg(feature = "parse")] {
/// # #[cfg(feature = "display")] {
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
/// let value = r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#;
/// let deserializer = value.parse::<toml_edit::de::ValueDeserializer>().unwrap();
/// let config = Config::deserialize(deserializer).unwrap();
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// # }
/// # }
/// ```
pub struct ValueDeserializer {
    inner: ItemDeserializer,
}

impl ValueDeserializer {
    pub(crate) fn new(input: crate::Item) -> Self {
        Self {
            inner: ItemDeserializer::new(input),
        }
    }

    pub(crate) fn with_struct_key_validation(mut self) -> Self {
        self.inner = self.inner.with_struct_key_validation();
        self
    }
}

// Note: this is wrapped by `toml::de::ValueDeserializer` and any trait methods
// implemented here need to be wrapped there
impl<'de> serde::Deserializer<'de> for ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_any(visitor)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_option(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_newtype_struct(name, visitor)
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
        self.inner.deserialize_struct(name, fields, visitor)
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
        self.inner.deserialize_enum(name, variants, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl serde::de::IntoDeserializer<'_, Error> for ValueDeserializer {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl serde::de::IntoDeserializer<'_, Error> for crate::Value {
    type Deserializer = ValueDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer::new(crate::Item::Value(self))
    }
}

#[cfg(feature = "parse")]
impl std::str::FromStr for ValueDeserializer {
    type Err = Error;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = crate::parser::parse_value(s).map_err(Error::from)?;
        Ok(v.into_deserializer())
    }
}
