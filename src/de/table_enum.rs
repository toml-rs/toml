use crate::de::Error;

/// Deserializes table values into enum variants.
pub(crate) struct TableEnumDeserializer {
    value: crate::Item,
}

impl TableEnumDeserializer {
    pub(crate) fn new(value: crate::Item) -> Self {
        TableEnumDeserializer { value }
    }
}

impl<'de> serde::de::VariantAccess<'de> for TableEnumDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            crate::Item::Table(values) => {
                if values.is_empty() {
                    Ok(())
                } else {
                    Err(Error::custom("expected empty table"))
                }
            }
            crate::Item::Value(crate::Value::InlineTable(values)) => {
                if values.is_empty() {
                    Ok(())
                } else {
                    Err(Error::custom("expected empty table"))
                }
            }
            e => Err(Error::custom(format!(
                "expected table, found {}",
                e.type_name()
            ))),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(super::ItemDeserializer::new(self.value))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            crate::Item::Table(values) => {
                let tuple_values = values
                    .into_iter()
                    .enumerate()
                    .map(|(index, (key, value))| match key.parse::<usize>() {
                        Ok(key_index) if key_index == index => Ok(value),
                        Ok(_) | Err(_) => Err(Error::custom(format!(
                            "expected table key `{}`, but was `{}`",
                            index, key
                        ))),
                    })
                    // Fold all values into a `Vec`, or return the first error.
                    .fold(Ok(Vec::with_capacity(len)), |result, value_result| {
                        result.and_then(move |mut tuple_values| match value_result {
                            Ok(value) => {
                                tuple_values.push(value);
                                Ok(tuple_values)
                            }
                            // `Result<de::Value, Self::Error>` to `Result<Vec<_>, Self::Error>`
                            Err(e) => Err(e),
                        })
                    })?;

                if tuple_values.len() == len {
                    serde::de::Deserializer::deserialize_seq(
                        super::ArrayDeserializer::new(tuple_values),
                        visitor,
                    )
                } else {
                    Err(Error::custom(format!("expected tuple with length {}", len)))
                }
            }
            crate::Item::Value(crate::Value::InlineTable(values)) => {
                let tuple_values = values
                    .into_iter()
                    .enumerate()
                    .map(|(index, (key, value))| match key.parse::<usize>() {
                        Ok(key_index) if key_index == index => Ok(value),
                        Ok(_) | Err(_) => Err(Error::custom(format!(
                            "expected table key `{}`, but was `{}`",
                            index, key
                        ))),
                    })
                    // Fold all values into a `Vec`, or return the first error.
                    .fold(Ok(Vec::with_capacity(len)), |result, value_result| {
                        result.and_then(move |mut tuple_values| match value_result {
                            Ok(value) => {
                                tuple_values.push(crate::Item::Value(value));
                                Ok(tuple_values)
                            }
                            // `Result<de::Value, Self::Error>` to `Result<Vec<_>, Self::Error>`
                            Err(e) => Err(e),
                        })
                    })?;

                if tuple_values.len() == len {
                    serde::de::Deserializer::deserialize_seq(
                        super::ArrayDeserializer::new(tuple_values),
                        visitor,
                    )
                } else {
                    Err(Error::custom(format!("expected tuple with length {}", len)))
                }
            }
            e => Err(Error::custom(format!(
                "expected table, found {}",
                e.type_name()
            ))),
        }
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_struct(
            super::ItemDeserializer::new(self.value).with_struct_key_validation(),
            "", // TODO: this should be the variant name
            fields,
            visitor,
        )
    }
}
