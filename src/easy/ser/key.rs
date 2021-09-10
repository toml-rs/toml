use super::{Error, ErrorKind};

pub(crate) struct KeySerializer;

impl serde::ser::Serializer for KeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = serde::ser::Impossible<String, Error>;
    type SerializeTuple = serde::ser::Impossible<String, Error>;
    type SerializeTupleStruct = serde::ser::Impossible<String, Error>;
    type SerializeTupleVariant = serde::ser::Impossible<String, Error>;
    type SerializeMap = serde::ser::Impossible<String, Error>;
    type SerializeStruct = serde::ser::Impossible<String, Error>;
    type SerializeStructVariant = serde::ser::Impossible<String, Error>;

    fn serialize_bool(self, _v: bool) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_i8(self, _v: i8) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_i16(self, _v: i16) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_i32(self, _v: i32) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_i64(self, _v: i64) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_u8(self, _v: u8) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_u16(self, _v: u16) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_u32(self, _v: u32) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_u64(self, _v: u64) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_f32(self, _v: f32) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_f64(self, _v: f64) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_char(self, _v: char) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_str(self, value: &str) -> Result<String, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_none(self) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<String, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_unit(self) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ErrorKind::KeyNotString.into())
    }
}
