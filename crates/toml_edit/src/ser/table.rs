use super::{Error, ErrorKind, KeySerializer};

#[doc(hidden)]
pub struct SerializeValueTable {
    inner: SerializeKeyValuePairs,
}

impl SerializeValueTable {
    pub(crate) fn new() -> Self {
        Self {
            inner: SerializeKeyValuePairs::new(),
        }
    }

    pub(crate) fn with_capacity(len: usize) -> Self {
        Self {
            inner: SerializeKeyValuePairs::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeMap for SerializeValueTable {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        self.inner.serialize_key(input)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        self.inner.serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner
            .end()
            .map(|items| crate::Value::InlineTable(crate::InlineTable::with_pairs(items)))
    }
}

impl serde::ser::SerializeStruct for SerializeValueTable {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner
            .end()
            .map(|items| crate::Value::InlineTable(crate::InlineTable::with_pairs(items)))
    }
}

struct SerializeKeyValuePairs {
    items: crate::table::KeyValuePairs,
    key: Option<crate::InternalString>,
}

impl SerializeKeyValuePairs {
    pub(crate) fn new() -> Self {
        Self {
            items: Default::default(),
            key: Default::default(),
        }
    }

    pub(crate) fn with_capacity(len: usize) -> Self {
        let mut s = Self::new();
        s.items.reserve(len);
        s
    }
}

impl serde::ser::SerializeMap for SerializeKeyValuePairs {
    type Ok = crate::table::KeyValuePairs;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        self.key = None;
        self.key = Some(input.serialize(KeySerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        let res = value.serialize(super::ValueSerializer {});
        match res {
            Ok(item) => {
                let key = self.key.take().unwrap();
                let kv = crate::table::TableKeyValue::new(
                    crate::Key::new(&key),
                    crate::Item::Value(item),
                );
                self.items.insert(key, kv);
            }
            Err(e) => {
                if e.kind != ErrorKind::UnsupportedNone {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.items)
    }
}

impl serde::ser::SerializeStruct for SerializeKeyValuePairs {
    type Ok = crate::table::KeyValuePairs;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        let res = value.serialize(super::ValueSerializer {});
        match res {
            Ok(item) => {
                let kv = crate::table::TableKeyValue::new(
                    crate::Key::new(key),
                    crate::Item::Value(item),
                );
                self.items.insert(crate::InternalString::from(key), kv);
            }
            Err(e) => {
                if e.kind != ErrorKind::UnsupportedNone {
                    return Err(e);
                }
            }
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.items)
    }
}
