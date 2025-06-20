use toml_write::TomlWrite as _;

use super::value::ValueSerializer;
use super::Buffer;
use super::Error;
use super::Table;

#[doc(hidden)]
pub struct SerializeDocumentTupleVariant<'d> {
    buf: &'d mut Buffer,
    table: Table,
    seen_value: bool,
}

impl<'d> SerializeDocumentTupleVariant<'d> {
    pub(crate) fn tuple(
        buf: &'d mut Buffer,
        mut table: Table,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        let dst = table.body_mut();
        dst.key(variant)?;
        dst.space()?;
        dst.keyval_sep()?;
        dst.space()?;
        dst.open_array()?;
        Ok(Self {
            buf,
            table,
            seen_value: false,
        })
    }
}

impl<'d> serde::ser::SerializeTupleVariant for SerializeDocumentTupleVariant<'d> {
    type Ok = &'d mut Buffer;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let dst = self.table.body_mut();

        if self.seen_value {
            dst.val_sep()?;
            dst.space()?;
        }
        self.seen_value = true;
        value.serialize(ValueSerializer::new(dst))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let dst = self.table.body_mut();
        dst.close_array()?;
        dst.newline()?;
        self.buf.push(self.table);
        Ok(self.buf)
    }
}
