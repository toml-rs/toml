use crate::de::Error;

pub(crate) struct ArrayDeserializer {
    input: Vec<crate::Item>,
}

impl ArrayDeserializer {
    pub(crate) fn new(input: Vec<crate::Item>) -> Self {
        Self { input }
    }
}

impl<'de> serde::Deserializer<'de> for ArrayDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ArraySeqAccess::new(self.input))
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}

impl<'de> serde::Deserializer<'de> for crate::Array {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ArraySeqAccess::with_array(self))
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Array {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

impl<'de> serde::Deserializer<'de> for crate::ArrayOfTables {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ArraySeqAccess::with_array_of_tables(self))
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier struct
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::ArrayOfTables {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub(crate) struct ArraySeqAccess {
    iter: std::vec::IntoIter<crate::Item>,
}

impl ArraySeqAccess {
    pub(crate) fn new(input: Vec<crate::Item>) -> Self {
        Self {
            iter: input.into_iter(),
        }
    }

    pub(crate) fn with_array(input: crate::Array) -> Self {
        Self::new(input.values)
    }

    pub(crate) fn with_array_of_tables(input: crate::ArrayOfTables) -> Self {
        Self::new(input.values)
    }
}

impl<'de> serde::de::SeqAccess<'de> for ArraySeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(v) => seed
                .deserialize(crate::de::ItemDeserializer::new(v))
                .map(Some),
            None => Ok(None),
        }
    }
}
