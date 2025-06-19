use serde::ser::SerializeSeq as _;
use toml_write::TomlWrite as _;

use super::value::SerializeValueArray;
use super::Error;
use crate::alloc_prelude::*;

#[doc(hidden)]
pub struct SerializeDocumentTupleVariant<'d> {
    inner: SerializeValueArray<'d>,
}

impl<'d> SerializeDocumentTupleVariant<'d> {
    pub(crate) fn tuple(
        dst: &'d mut String,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        dst.key(variant)?;
        dst.space()?;
        dst.keyval_sep()?;
        dst.space()?;
        Ok(Self {
            inner: SerializeValueArray::seq(dst)?,
        })
    }
}

impl<'d> serde::ser::SerializeTupleVariant for SerializeDocumentTupleVariant<'d> {
    type Ok = &'d mut String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let dst = self.inner.end()?;
        dst.newline()?;
        Ok(dst)
    }
}
