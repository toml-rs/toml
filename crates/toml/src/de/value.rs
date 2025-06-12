use crate::de::Error;

/// Deserialization TOML [value][crate::Value]
///
/// # Example
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
/// let config = Config::deserialize(toml::de::ValueDeserializer::new(
///     r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#
/// )).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub struct ValueDeserializer<'i> {
    input: &'i str,
}

#[cfg(feature = "parse")]
impl<'i> ValueDeserializer<'i> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'i str) -> Self {
        Self { input }
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
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
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
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
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
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
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
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
