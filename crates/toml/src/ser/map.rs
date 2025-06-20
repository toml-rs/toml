use serde::Serializer as _;

use super::style::Style;
use super::write_document;
use super::{Error, Serializer};

type InnerSerializeDocumentMap =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

#[doc(hidden)]
pub struct SerializeDocumentMap<'d> {
    inner: InnerSerializeDocumentMap,
    dst: &'d mut String,
    settings: Style,
}

impl<'d> SerializeDocumentMap<'d> {
    pub(crate) fn map(ser: Serializer<'d>, inner: InnerSerializeDocumentMap) -> Self {
        Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        }
    }
}

impl serde::ser::SerializeMap for SerializeDocumentMap<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, input: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_key(input).map_err(Error::wrap)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_value(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

impl serde::ser::SerializeStruct for SerializeDocumentMap<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

type InnerSerializeDocumentStructVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeStructVariant;

#[doc(hidden)]
pub struct SerializeDocumentStructVariant<'d> {
    inner: InnerSerializeDocumentStructVariant,
    dst: &'d mut String,
    settings: Style,
}

impl<'d> SerializeDocumentStructVariant<'d> {
    pub(crate) fn struct_(
        ser: Serializer<'d>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self, Error> {
        let inner = toml_edit::ser::ValueSerializer::new()
            .serialize_struct_variant(name, variant_index, variant, len)
            .map_err(Error::wrap)?;
        Ok(Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        })
    }
}

impl serde::ser::SerializeStructVariant for SerializeDocumentStructVariant<'_> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}
