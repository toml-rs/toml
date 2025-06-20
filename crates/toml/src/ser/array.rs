use serde::Serializer as _;

use super::style::Style;
use super::write_document;
use super::{Error, Serializer};

type InnerSerializeDocumentSeq =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

#[doc(hidden)]
pub struct SerializeDocumentArray<'d> {
    inner: InnerSerializeDocumentSeq,
    dst: &'d mut String,
    settings: Style,
}

impl<'d> SerializeDocumentArray<'d> {
    pub(crate) fn seq(ser: Serializer<'d>, inner: InnerSerializeDocumentSeq) -> Self {
        Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        }
    }
}

impl serde::ser::SerializeSeq for SerializeDocumentArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_element(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

impl serde::ser::SerializeTuple for SerializeDocumentArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_element(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

impl serde::ser::SerializeTupleStruct for SerializeDocumentArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

type InnerSerializeDocumentTupleVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeTupleVariant;

#[doc(hidden)]
pub struct SerializeDocumentTupleVariant<'d> {
    inner: InnerSerializeDocumentTupleVariant,
    dst: &'d mut String,
    settings: Style,
}

impl<'d> SerializeDocumentTupleVariant<'d> {
    pub(crate) fn tuple(
        ser: Serializer<'d>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self, Error> {
        let inner = toml_edit::ser::ValueSerializer::new()
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map_err(Error::wrap)?;
        Ok(Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        })
    }
}

impl serde::ser::SerializeTupleVariant for SerializeDocumentTupleVariant<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}
