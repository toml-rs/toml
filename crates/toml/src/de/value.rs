use serde::de::IntoDeserializer as _;

use crate::de::ArrayDeserializer;
use crate::de::DatetimeDeserializer;
use crate::de::DeString;
use crate::de::DeValue;
use crate::de::Error;
use crate::de::TableDeserializer;

/// Deserialization implementation for TOML [values][crate::Value].
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
/// let deserializer = toml::de::ValueDeserializer::parse(value).unwrap();
/// let config = Config::deserialize(deserializer).unwrap();
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// # }
/// # }
/// ```
pub struct ValueDeserializer<'i> {
    span: Option<std::ops::Range<usize>>,
    input: DeValue<'i>,
    validate_struct_keys: bool,
}

impl<'i> ValueDeserializer<'i> {
    /// Parse a TOML value
    pub fn parse(raw: &'i str) -> Result<Self, Error> {
        let input = DeValue::parse(raw)?;
        let span = input.span();
        let input = input.into_inner();
        Ok(Self::new(input, Some(span)))
    }

    pub(crate) fn new(input: DeValue<'i>, span: Option<std::ops::Range<usize>>) -> Self {
        Self {
            input,
            span,
            validate_struct_keys: false,
        }
    }

    pub(crate) fn with_struct_key_validation(mut self) -> Self {
        self.validate_struct_keys = true;
        self
    }
}

impl<'de> serde::Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span.clone();
        match self.input {
            DeValue::String(DeString::Owned(v)) => visitor.visit_string(v),
            DeValue::String(DeString::Borrowed(v)) => visitor.visit_str(v),
            DeValue::Integer(v) => visitor.visit_i64(v),
            DeValue::Float(v) => visitor.visit_f64(v),
            DeValue::Boolean(v) => visitor.visit_bool(v),
            DeValue::Datetime(v) => visitor.visit_map(DatetimeDeserializer::new(v)),
            DeValue::Array(v) => ArrayDeserializer::new(v, span.clone()).deserialize_any(visitor),
            DeValue::Table(v) => TableDeserializer::new(v, span.clone()).deserialize_any(visitor),
        }
        .map_err(|mut e: Self::Error| {
            if e.span().is_none() {
                e.set_span(span);
            }
            e
        })
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span.clone();
        visitor.visit_some(self).map_err(|mut e: Self::Error| {
            if e.span().is_none() {
                e.set_span(span);
            }
            e
        })
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span.clone();
        visitor
            .visit_newtype_struct(self)
            .map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
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
        if serde_spanned::__unstable::is_spanned(name, fields) {
            if let Some(span) = self.span.clone() {
                return visitor.visit_map(super::SpannedDeserializer::new(self, span));
            }
        }

        if name == toml_datetime::__unstable::NAME && fields == [toml_datetime::__unstable::FIELD] {
            let span = self.span.clone();
            if let DeValue::Datetime(d) = self.input {
                return visitor.visit_map(DatetimeDeserializer::new(d)).map_err(
                    |mut e: Self::Error| {
                        if e.span().is_none() {
                            e.set_span(span);
                        }
                        e
                    },
                );
            }
        }

        if self.validate_struct_keys {
            let span = self.span.clone();
            match &self.input {
                DeValue::Table(values) => super::validate_struct_keys(values, fields),
                _ => Ok(()),
            }
            .map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
                e
            })?;
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
        let span = self.span.clone();
        match self.input {
            DeValue::String(v) => visitor.visit_enum(v.into_deserializer()),
            DeValue::Table(v) => {
                TableDeserializer::new(v, span.clone()).deserialize_enum(name, variants, visitor)
            }
            _ => Err(Error::custom("wanted string or table", span.clone())),
        }
        .map_err(|mut e: Self::Error| {
            if e.span().is_none() {
                e.set_span(span);
            }
            e
        })
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, Error> for ValueDeserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
