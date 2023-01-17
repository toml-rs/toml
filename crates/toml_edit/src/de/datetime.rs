use serde::de::IntoDeserializer;

use crate::de::Error;

pub(crate) struct DatetimeDeserializer {
    visited: bool,
    date: crate::Datetime,
}

impl DatetimeDeserializer {
    pub(crate) fn new(date: crate::Datetime) -> Self {
        Self {
            visited: false,
            date,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for DatetimeDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.visited {
            return Ok(None);
        }
        self.visited = true;
        seed.deserialize(DatetimeFieldDeserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.date.to_string().into_deserializer())
    }
}

pub(crate) struct DatetimeFieldDeserializer;

impl<'de> serde::de::Deserializer<'de> for DatetimeFieldDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(toml_datetime::__unstable::FIELD)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    }
}
