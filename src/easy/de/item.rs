use crate::easy::de::Error;

pub(crate) struct ItemDeserializer {
    input: crate::Item,
}

impl ItemDeserializer {
    pub(crate) fn new(input: crate::Item) -> Self {
        Self { input }
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut ItemDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut input = Default::default();
        std::mem::swap(&mut input, &mut self.input);
        match input {
            crate::Item::None => visitor.visit_none(),
            crate::Item::Value(v) => {
                crate::easy::de::ValueDeserializer::new(v).deserialize_any(visitor)
            }
            crate::Item::Table(v) => visitor.visit_map(crate::easy::de::TableMapAccess::new(v)),
            crate::Item::ArrayOfTables(v) => {
                visitor.visit_seq(crate::easy::de::ArrayOfTablesSeqAccess::new(v))
            }
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}
