use super::write_document;
use super::write_value;
use super::{Error, Serializer, ValueSerializer};
use crate::fmt::DocumentFormatter;

type InnerSerializeDocumentSeq =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

#[doc(hidden)]
pub struct SerializeDocumentArray<'d> {
    inner: InnerSerializeDocumentSeq,
    dst: &'d mut String,
    settings: DocumentFormatter,
}

impl<'d> SerializeDocumentArray<'d> {
    pub(crate) fn new(ser: Serializer<'d>, inner: InnerSerializeDocumentSeq) -> Self {
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

impl serde::ser::SerializeTupleVariant for SerializeDocumentArray<'_> {
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

type InnerSerializeValueSeq = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

#[doc(hidden)]
pub struct SerializeValueArray<'d> {
    inner: InnerSerializeValueSeq,
    dst: &'d mut String,
}

impl<'d> SerializeValueArray<'d> {
    pub(crate) fn new(ser: ValueSerializer<'d>, inner: InnerSerializeValueSeq) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl serde::ser::SerializeSeq for SerializeValueArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_element(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

impl serde::ser::SerializeTuple for SerializeValueArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_element(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

impl serde::ser::SerializeTupleVariant for SerializeValueArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

impl serde::ser::SerializeTupleStruct for SerializeValueArray<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}
