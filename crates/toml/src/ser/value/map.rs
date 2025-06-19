use super::write_value;
use super::{Error, ValueSerializer};

type InnerSerializeMap = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

#[doc(hidden)]
pub struct SerializeMap<'d> {
    inner: InnerSerializeMap,
    dst: &'d mut String,
}

impl<'d> SerializeMap<'d> {
    pub(crate) fn map(ser: ValueSerializer<'d>, inner: InnerSerializeMap) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl<'d> serde::ser::SerializeMap for SerializeMap<'d> {
    type Ok = &'d mut String;
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
        write_value(self.dst, self.inner.end())
    }
}

impl<'d> serde::ser::SerializeStruct for SerializeMap<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

type InnerSerializeValueStructVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeStructVariant;

#[doc(hidden)]
pub struct SerializeValueStructVariant<'d> {
    inner: InnerSerializeValueStructVariant,
    dst: &'d mut String,
}

impl<'d> SerializeValueStructVariant<'d> {
    pub(crate) fn struct_(
        ser: ValueSerializer<'d>,
        inner: InnerSerializeValueStructVariant,
    ) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl<'d> serde::ser::SerializeStructVariant for SerializeValueStructVariant<'d> {
    type Ok = &'d mut String;
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
        write_value(self.dst, self.inner.end())
    }
}
