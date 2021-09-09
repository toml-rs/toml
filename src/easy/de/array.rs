use crate::easy::de::Error;

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
                .deserialize(crate::easy::de::ItemDeserializer::new(v))
                .map(Some),
            None => Ok(None),
        }
    }
}
