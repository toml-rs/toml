use toml_write::TomlWrite as _;

use super::Error;
use super::SerializeMap;
use super::SerializeStructVariant;
use super::SerializeTupleVariant;
use super::SerializeValueArray;

pub(crate) struct ValueSerializer<'d> {
    dst: &'d mut String,
}

impl<'d> ValueSerializer<'d> {
    /// Creates a new serializer generate a TOML document.
    pub(crate) fn new(dst: &'d mut String) -> Self {
        Self { dst }
    }
}

impl<'d> serde::ser::Serializer for ValueSerializer<'d> {
    type Ok = &'d mut String;
    type Error = Error;
    type SerializeSeq = SerializeValueArray<'d>;
    type SerializeTuple = SerializeValueArray<'d>;
    type SerializeTupleStruct = SerializeValueArray<'d>;
    type SerializeTupleVariant = SerializeTupleVariant<'d>;
    type SerializeMap = SerializeMap<'d>;
    type SerializeStruct = SerializeMap<'d>;
    type SerializeStructVariant = SerializeStructVariant<'d>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_f32(self, mut v: f32) -> Result<Self::Ok, Self::Error> {
        // Discard sign of NaN when serialized using Serde.
        //
        // In all likelihood the sign of NaNs is not meaningful in the user's
        // program. Ending up with `-nan` in the TOML document would usually be
        // surprising and undesirable, when the sign of the NaN was not
        // intentionally controlled by the caller, or may even be
        // nondeterministic if it comes from arithmetic operations or a cast.
        if v.is_nan() {
            v = v.copysign(1.0);
        }
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_f64(self, mut v: f64) -> Result<Self::Ok, Self::Error> {
        // Discard sign of NaN when serialized using Serde.
        //
        // In all likelihood the sign of NaNs is not meaningful in the user's
        // program. Ending up with `-nan` in the TOML document would usually be
        // surprising and undesirable, when the sign of the NaN was not
        // intentionally controlled by the caller, or may even be
        // nondeterministic if it comes from arithmetic operations or a cast.
        if v.is_nan() {
            v = v.copysign(1.0);
        }
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.dst.value(v)?;
        Ok(self.dst)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::Serialize;
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_none())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_type(Some("unit")))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_type(Some(name)))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.dst.open_inline_table()?;
        self.dst.space()?;
        self.dst.key(variant)?;
        self.dst.space()?;
        self.dst.keyval_sep()?;
        self.dst.space()?;
        value.serialize(ValueSerializer::new(self.dst))?;
        self.dst.space()?;
        self.dst.close_inline_table()?;
        Ok(self.dst)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        SerializeValueArray::seq(self.dst)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        SerializeTupleVariant::tuple(self.dst, variant, len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        SerializeMap::map(self.dst)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        SerializeMap::struct_(name, self.dst)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        SerializeStructVariant::struct_(self.dst, variant, len)
    }
}
