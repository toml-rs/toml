use toml_write::TomlWrite as _;

use super::Error;

#[doc(hidden)]
pub struct SerializeValueArray<'d> {
    dst: &'d mut String,
    seen_value: bool,
}

impl<'d> SerializeValueArray<'d> {
    pub(crate) fn seq(dst: &'d mut String) -> Result<Self, Error> {
        dst.open_array()?;
        Ok(Self {
            dst,
            seen_value: false,
        })
    }
}

impl<'d> serde::ser::SerializeSeq for SerializeValueArray<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        if self.seen_value {
            self.dst.val_sep()?;
            self.dst.space()?;
        }
        self.seen_value = true;
        value.serialize(super::ValueSerializer::new(self.dst))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.dst.close_array()?;
        Ok(self.dst)
    }
}

impl<'d> serde::ser::SerializeTuple for SerializeValueArray<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'d> serde::ser::SerializeTupleVariant for SerializeValueArray<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'d> serde::ser::SerializeTupleStruct for SerializeValueArray<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant<'d> {
    inner: SerializeValueArray<'d>,
}

impl<'d> SerializeTupleVariant<'d> {
    pub(crate) fn tuple(
        dst: &'d mut String,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        dst.open_inline_table()?;
        dst.space()?;
        dst.key(variant)?;
        dst.space()?;
        dst.keyval_sep()?;
        dst.space()?;
        Ok(Self {
            inner: SerializeValueArray::seq(dst)?,
        })
    }
}

impl<'d> serde::ser::SerializeTupleVariant for SerializeTupleVariant<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(&mut self.inner, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let dst = serde::ser::SerializeSeq::end(self.inner)?;
        dst.space()?;
        dst.close_inline_table()?;
        Ok(dst)
    }
}
