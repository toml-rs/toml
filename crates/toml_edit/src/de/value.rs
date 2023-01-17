use serde::de::IntoDeserializer;

use crate::de::DatetimeDeserializer;
use crate::de::Error;

impl<'de> serde::Deserializer<'de> for crate::Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            crate::Value::String(v) => visitor.visit_string(v.into_value()),
            crate::Value::Integer(v) => visitor.visit_i64(v.into_value()),
            crate::Value::Float(v) => visitor.visit_f64(v.into_value()),
            crate::Value::Boolean(v) => visitor.visit_bool(v.into_value()),
            crate::Value::Datetime(v) => {
                visitor.visit_map(DatetimeDeserializer::new(v.into_value()))
            }
            crate::Value::Array(v) => v.into_deserializer().deserialize_any(visitor),
            crate::Value::InlineTable(v) => v.into_deserializer().deserialize_any(visitor),
        }
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
            if let crate::Value::Datetime(d) = self {
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

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)
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
        match self {
            crate::Value::String(v) => visitor.visit_enum(v.into_value().into_deserializer()),
            crate::Value::InlineTable(v) => {
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
            _ => Err(crate::de::Error::custom(
                "wanted string or inline table",
                self.span(),
            )),
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit newtype_struct
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Value {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
