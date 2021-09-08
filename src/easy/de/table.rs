use serde::de::IntoDeserializer;

use crate::easy::de::Error;

pub(crate) struct TableDeserializer {
    input: crate::Table,
}

impl TableDeserializer {
    pub(crate) fn new(input: crate::Table) -> Self {
        Self { input }
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut TableDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut input = crate::Table::new();
        std::mem::swap(&mut input, &mut self.input);
        visitor.visit_map(TableMapAccess::new(input))
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}
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
            Some(v) => seed.deserialize(&mut crate::easy::de::ItemDeserializer::new(v)),
            None => {
                panic!("no more values in next_value_seed, internal error in ValueDeserializer")
            }
        }
    }
}
