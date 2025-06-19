use core::fmt::Write as _;

use toml_write::TomlWrite as _;

use super::value::KeySerializer;
use super::value::MapValueSerializer;
use super::value::SerializeTable;
use super::Error;
use crate::alloc_prelude::*;

#[doc(hidden)]
pub struct SerializeDocumentTable<'d> {
    dst: &'d mut String,
    key: Option<String>,
}

impl<'d> SerializeDocumentTable<'d> {
    pub(crate) fn map(dst: &'d mut String) -> Result<Self, Error> {
        Ok(Self { dst, key: None })
    }

    fn end(self) -> Result<&'d mut String, Error> {
        Ok(self.dst)
    }
}

impl<'d> serde::ser::SerializeMap for SerializeDocumentTable<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_key<T>(&mut self, input: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let mut encoded_key = String::new();
        input.serialize(KeySerializer {
            dst: &mut encoded_key,
        })?;
        self.key = Some(encoded_key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let encoded_key = self
            .key
            .take()
            .expect("always called after `serialize_key`");
        let mut encoded_value = String::new();
        let mut is_none = false;
        let value_serializer = MapValueSerializer::new(&mut encoded_value, &mut is_none);
        let res = value.serialize(value_serializer);
        match res {
            Ok(_) => {
                write!(self.dst, "{encoded_key}")?;
                self.dst.space()?;
                self.dst.keyval_sep()?;
                self.dst.space()?;
                write!(self.dst, "{encoded_value}")?;
                self.dst.newline()?;
            }
            Err(e) => {
                if !(e == Error::unsupported_none() && is_none) {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<'d> serde::ser::SerializeStruct for SerializeDocumentTable<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let mut encoded_value = String::new();
        let mut is_none = false;
        let value_serializer = MapValueSerializer::new(&mut encoded_value, &mut is_none);
        let res = value.serialize(value_serializer);
        match res {
            Ok(_) => {
                self.dst.key(key)?;
                self.dst.space()?;
                self.dst.keyval_sep()?;
                self.dst.space()?;
                write!(self.dst, "{encoded_value}")?;
                self.dst.newline()?;
            }
            Err(e) => {
                if !(e == Error::unsupported_none() && is_none) {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

pub struct SerializeDocumentStructVariant<'d> {
    inner: SerializeTable<'d>,
}

impl<'d> SerializeDocumentStructVariant<'d> {
    pub(crate) fn struct_(
        dst: &'d mut String,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        dst.key(variant)?;
        dst.space()?;
        dst.keyval_sep()?;
        dst.space()?;
        Ok(Self {
            inner: SerializeTable::map(dst)?,
        })
    }
}

impl<'d> serde::ser::SerializeStructVariant for SerializeDocumentStructVariant<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeStruct::serialize_field(&mut self.inner, key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let dst = self.inner.end()?;
        dst.newline()?;
        Ok(dst)
    }
}
