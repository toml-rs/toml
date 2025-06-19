use super::write_value;
use super::{Error, ValueSerializer};

type InnerSerializeValueSeq = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

#[doc(hidden)]
pub struct SerializeValueArray<'d> {
    inner: InnerSerializeValueSeq,
    dst: &'d mut String,
}

impl<'d> SerializeValueArray<'d> {
    pub(crate) fn seq(ser: ValueSerializer<'d>, inner: InnerSerializeValueSeq) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl<'d> serde::ser::SerializeSeq for SerializeValueArray<'d> {
    type Ok = &'d mut String;
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

impl<'d> serde::ser::SerializeTuple for SerializeValueArray<'d> {
    type Ok = &'d mut String;
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

impl<'d> serde::ser::SerializeTupleStruct for SerializeValueArray<'d> {
    type Ok = &'d mut String;
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

type InnerSerializeValueTupleVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeTupleVariant;

#[doc(hidden)]
pub struct SerializeValueTupleVariant<'d> {
    inner: InnerSerializeValueTupleVariant,
    dst: &'d mut String,
}

impl<'d> SerializeValueTupleVariant<'d> {
    pub(crate) fn tuple(ser: ValueSerializer<'d>, inner: InnerSerializeValueTupleVariant) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl<'d> serde::ser::SerializeTupleVariant for SerializeValueTupleVariant<'d> {
    type Ok = &'d mut String;
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
