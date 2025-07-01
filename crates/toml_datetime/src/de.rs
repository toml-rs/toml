//! Deserialzation support for [`Datetime`][crate::Datetime]

use alloc::string::ToString;

use serde::de::value::BorrowedStrDeserializer;
use serde::de::IntoDeserializer;

/// Check if deserializing a [`Datetime`][crate::Datetime]
pub fn is_datetime(name: &'static str) -> bool {
    crate::datetime::is_datetime(name)
}

/// Deserializer / format support for emitting [`Datetime`][crate::Datetime]
pub struct DatetimeDeserializer<E> {
    date: Option<crate::Datetime>,
    _error: core::marker::PhantomData<E>,
}

impl<E> DatetimeDeserializer<E> {
    /// Create a deserializer to emit [`Datetime`][crate::Datetime]
    pub fn new(date: crate::Datetime) -> Self {
        Self {
            date: Some(date),
            _error: Default::default(),
        }
    }
}

impl<'de, E> serde::de::MapAccess<'de> for DatetimeDeserializer<E>
where
    E: serde::de::Error,
{
    type Error = E;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.date.is_some() {
            seed.deserialize(BorrowedStrDeserializer::new(crate::datetime::FIELD))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(date) = self.date.take() {
            seed.deserialize(date.to_string().into_deserializer())
        } else {
            panic!("next_value_seed called before next_key_seed")
        }
    }
}
