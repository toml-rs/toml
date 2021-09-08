use crate::easy::de::Error;

pub(crate) struct ArrayOfTablesSeqAccess {
    iter: crate::ArrayOfTablesIntoIter,
}

impl<'de> ArrayOfTablesSeqAccess {
    pub(crate) fn new(input: crate::ArrayOfTables) -> Self {
        Self {
            iter: input.into_iter(),
        }
    }
}

impl<'de> serde::de::SeqAccess<'de> for ArrayOfTablesSeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(v) => seed
                .deserialize(&mut crate::easy::de::TableDeserializer::new(v))
                .map(Some),
            None => Ok(None),
        }
    }
}
