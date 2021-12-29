use serde::de::IntoDeserializer;

use crate::de::Error;

impl<'de, 'a> serde::Deserializer<'de> for crate::Table {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(crate::de::TableMapAccess::new(self))
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.is_empty() {
            Err(crate::de::Error::custom(
                "wanted exactly 1 element, found 0 elements",
            ))
        } else if self.len() != 1 {
            Err(crate::de::Error::custom(
                "wanted exactly 1 element, more than 1 element",
            ))
        } else {
            visitor.visit_enum(crate::de::TableMapAccess::new(self))
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Table {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub(crate) struct TableMapAccess {
    iter: indexmap::map::IntoIter<crate::InternalString, crate::table::TableKeyValue>,
    value: Option<(crate::InternalString, crate::Item)>,
}

impl TableMapAccess {
    pub(crate) fn new(input: crate::Table) -> Self {
        Self {
            iter: input.items.into_iter(),
            value: None,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for TableMapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                let ret = seed.deserialize(k.into_deserializer()).map(Some);
                self.value = Some((k, v.value));
                ret
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some((k, v)) => seed
                .deserialize(crate::de::ItemDeserializer::new(v))
                .map_err(|mut err| {
                    err.parent_key(k);
                    err
                }),
            None => {
                panic!("no more values in next_value_seed, internal error in ItemDeserializer")
            }
        }
    }
}

impl<'de> serde::de::EnumAccess<'de> for TableMapAccess {
    type Error = Error;
    type Variant = super::TableEnumDeserializer;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let (key, value) = match self.iter.next() {
            Some(pair) => pair,
            None => {
                return Err(Error::custom(
                    "expected table with exactly 1 entry, found empty table",
                ));
            }
        };

        seed.deserialize(key.into_deserializer())
            .map(|val| (val, super::TableEnumDeserializer::new(value.value)))
    }
}
