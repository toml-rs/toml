use serde::de::IntoDeserializer as _;

use crate::de::DatetimeDeserializer;
use crate::de::Error;

pub(crate) struct ItemDeserializer {
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

impl<'de> serde::Deserializer<'de> for ItemDeserializer {
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
        if super::is_spanned(name, fields) {
            if let Some(span) = self.input.span() {
                return visitor.visit_map(super::SpannedDeserializer::new(self.input, span));
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
            })?
        }

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
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for ItemDeserializer {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::Deserializer<'de> for crate::Item {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span();
        match self {
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
        let span = self.span();
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
        let span = self.span();
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
        if name == toml_datetime::__unstable::NAME && fields == [toml_datetime::__unstable::FIELD] {
            if let crate::Item::Value(crate::Value::Datetime(d)) = self {
                return visitor.visit_map(DatetimeDeserializer::new(d.into_value()));
            }
        }

        if super::is_spanned(name, fields) {
            if let Some(span) = self.span() {
                return visitor.visit_map(super::SpannedDeserializer::new(self, span));
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
        let span = self.span();
        match self {
            crate::Item::Value(crate::Value::String(v)) => {
                visitor.visit_enum(v.into_value().into_deserializer())
            }
            crate::Item::Value(crate::Value::InlineTable(v)) => {
                if v.is_empty() {
                    Err(crate::de::Error::custom(
                        "wanted exactly 1 element, found 0 elements",
                        v.span(),
                    ))
                } else if v.len() != 1 {
                    Err(crate::de::Error::custom(
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
            e => Err(crate::de::Error::custom("wanted string or table", e.span())),
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

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Item {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
