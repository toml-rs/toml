use serde::de::IntoDeserializer;

use crate::easy::de::Error;

pub(crate) struct TableMapAccess {
    iter: linked_hash_map::IntoIter<crate::repr::InternalString, crate::table::TableKeyValue>,
    value: Option<crate::Item>,
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
                self.value = Some(v.value);
                seed.deserialize(k.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(v) => seed.deserialize(crate::easy::de::ItemDeserializer::new(v)),
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
