use serde::de::IntoDeserializer as _;

use crate::de::DatetimeDeserializer;
use crate::de::Error;

/// Deserialization implementation for TOML [items][crate::Item].
///
/// Can be created either directly from TOML strings, using [`std::str::FromStr`],
/// or from parsed [items][crate::Item] using [`serde::de::IntoDeserializer::into_deserializer`].
///
/// # Example
///
/// ```
/// # #[cfg(feature = "parse")] {
/// # #[cfg(feature = "display")] {
/// use serde::{Deserialize, de::IntoDeserializer};
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: String,
/// }
///
/// let mut table = toml_edit::Table::new();
/// table.insert("title", "TOML Example".into());
/// table.insert("owner", "Lisa".into());
///
/// let deserializer = toml_edit::Item::Table(table).into_deserializer();
/// let config = Config::deserialize(deserializer).unwrap();
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner, "Lisa");
/// # }
/// # }
/// ```
pub struct ItemDeserializer {
    input: crate::Item,
    validate_struct_keys: bool,
}

impl ItemDeserializer {
    pub(crate) fn new(input: crate::Item) -> Self {
        Self {
            input,
            validate_struct_keys: false,
        }
    }

    pub(crate) fn with_struct_key_validation(mut self) -> Self {
        self.validate_struct_keys = true;
        self
    }
}

// Note: this is wrapped by `toml::de::ItemDeserializer` and any trait methods
// implemented here need to be wrapped there
impl<'de> serde::Deserializer<'de> for ItemDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.input.span();
        match self.input {
            crate::Item::None => visitor.visit_none(),
            crate::Item::Value(crate::Value::String(v)) => visitor.visit_string(v.into_value()),
            crate::Item::Value(crate::Value::Integer(v)) => visitor.visit_i64(v.into_value()),
            crate::Item::Value(crate::Value::Float(v)) => visitor.visit_f64(v.into_value()),
            crate::Item::Value(crate::Value::Boolean(v)) => visitor.visit_bool(v.into_value()),
            crate::Item::Value(crate::Value::Datetime(v)) => {
                visitor.visit_map(DatetimeDeserializer::new(v.into_value()))
            }
            crate::Item::Value(crate::Value::Array(v)) => {
                v.into_deserializer().deserialize_any(visitor)
            }
            crate::Item::Value(crate::Value::InlineTable(v)) => {
                v.into_deserializer().deserialize_any(visitor)
            }
            crate::Item::Table(v) => v.into_deserializer().deserialize_any(visitor),
            crate::Item::ArrayOfTables(v) => v.into_deserializer().deserialize_any(visitor),
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
        let span = self.input.span();
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
        let span = self.input.span();
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
            if let Some(span) = self.input.span() {
                return visitor.visit_map(super::SpannedDeserializer::new(self, span));
            }
        }

        if name == toml_datetime::__unstable::NAME && fields == [toml_datetime::__unstable::FIELD] {
            let span = self.input.span();
            if let crate::Item::Value(crate::Value::Datetime(d)) = self.input {
                return visitor
                    .visit_map(DatetimeDeserializer::new(d.into_value()))
                    .map_err(|mut e: Self::Error| {
                        if e.span().is_none() {
                            e.set_span(span);
                        }
                        e
                    });
            }
        }

        if self.validate_struct_keys {
            let span = self.input.span();
            match &self.input {
                crate::Item::Table(values) => super::validate_struct_keys(&values.items, fields),
                crate::Item::Value(crate::Value::InlineTable(values)) => {
                    super::validate_struct_keys(&values.items, fields)
                }
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
        let span = self.input.span();
        match self.input {
            crate::Item::Value(crate::Value::String(v)) => {
                visitor.visit_enum(v.into_value().into_deserializer())
            }
            crate::Item::Value(crate::Value::InlineTable(v)) => {
                if v.is_empty() {
                    Err(Error::custom(
                        "wanted exactly 1 element, found 0 elements",
                        v.span(),
                    ))
                } else if v.len() != 1 {
                    Err(Error::custom(
                        "wanted exactly 1 element, more than 1 element",
                        v.span(),
                    ))
                } else {
                    v.into_deserializer()
                        .deserialize_enum(name, variants, visitor)
                }
            }
            crate::Item::Table(v) => v
                .into_deserializer()
                .deserialize_enum(name, variants, visitor),
            e => Err(Error::custom("wanted string or table", e.span())),
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

impl serde::de::IntoDeserializer<'_, Error> for ItemDeserializer {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl serde::de::IntoDeserializer<'_, Error> for crate::Item {
    type Deserializer = ItemDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        ItemDeserializer::new(self)
    }
}
