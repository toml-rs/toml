use super::{Error, ErrorKind, KeySerializer};
use crate::Datetime;
use std::str::FromStr;

#[doc(hidden)]
pub struct SerializeItemTable {
    is_date_time: bool,
    date_time: Option<Datetime>,
    inner: SerializeKeyValuePairs,
}

impl SerializeItemTable {
    pub(crate) fn new(is_date_time: bool) -> Self {
        Self {
            is_date_time,
            date_time: None,
            inner: SerializeKeyValuePairs::new(),
        }
    }

    pub(crate) fn with_capacity(len: usize) -> Self {
        Self {
            is_date_time: false,
            date_time: None,
            inner: SerializeKeyValuePairs::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeMap for SerializeItemTable {
    type Ok = crate::Item;
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
        self.inner.end().map(|items| {
            crate::Item::Value(crate::Value::InlineTable(crate::InlineTable::with_pairs(
                items,
            )))
        })
    }
}

impl serde::ser::SerializeStruct for SerializeItemTable {
    type Ok = crate::Item;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        if self.is_date_time {
            if key == crate::dt_serde::FIELD {
                let value = value.serialize(KeySerializer)?;
                let datetime = Datetime::from_str(&value).unwrap();
                self.date_time = Some(datetime);
                Ok(())
            } else {
                Err(Error::custom("invalid date time"))
            }
        } else {
            self.inner.serialize_field(key, value)
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.is_date_time {
            let datetime = self.date_time.unwrap();
            Ok(crate::Item::Value(crate::Value::Datetime(
                crate::Formatted::new(datetime),
            )))
        } else {
            self.inner.end().map(|items| {
                crate::Item::Value(crate::Value::InlineTable(crate::InlineTable::with_pairs(
                    items,
                )))
            })
        }
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
        let res = value.serialize(super::ItemSerializer {});
        let item = match res {
            Ok(item) => item,
            Err(e) => {
                if e.kind != ErrorKind::UnsupportedNone {
                    return Err(e);
                }
                crate::Item::None
            }
        };
        if !item.is_none() {
            let key = self.key.take().unwrap();
            let kv = crate::table::TableKeyValue::new(crate::Key::new(&key), item);
            self.items.insert(key, kv);
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
        let res = value.serialize(super::ItemSerializer {});
        let item = match res {
            Ok(item) => item,
            Err(e) => {
                if e.kind != ErrorKind::UnsupportedNone {
                    return Err(e);
                }
                crate::Item::None
            }
        };
        if !item.is_none() {
            let kv = crate::table::TableKeyValue::new(crate::Key::new(key), item);
            self.items.insert(crate::InternalString::from(key), kv);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.items)
    }
}
