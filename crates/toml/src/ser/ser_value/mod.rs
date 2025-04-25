mod array;
mod key;
mod map;
mod value;

use super::Error;
#[allow(clippy::wildcard_imports)]
use array::*;
#[allow(clippy::wildcard_imports)]
use map::*;

/// Serialization for TOML [values][crate::Value].
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Serialize)]
/// struct Database {
///     ip: String,
///     port: Vec<u16>,
///     connection_max: u32,
///     enabled: bool,
/// }
///
/// let config = Config {
///     database: Database {
///         ip: "192.168.1.1".to_string(),
///         port: vec![8001, 8002, 8003],
///         connection_max: 5000,
///         enabled: false,
///     },
/// };
///
/// let mut value = String::new();
/// serde::Serialize::serialize(
///     &config,
///     toml::ser::ValueSerializer::new(&mut value)
/// ).unwrap();
/// println!("{}", value)
/// ```
#[cfg(feature = "display")]
pub struct ValueSerializer<'d> {
    dst: &'d mut String,
}

impl<'d> ValueSerializer<'d> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: &'d mut String) -> Self {
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
        value::ValueSerializer::new(self.dst).serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_none()
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value::ValueSerializer::new(self.dst).serialize_some(v)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value::ValueSerializer::new(self.dst).serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value::ValueSerializer::new(self.dst).serialize_newtype_variant(
            name,
            variant_index,
            variant,
            value,
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_tuple_variant(
            name,
            variant_index,
            variant,
            len,
        )
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        value::ValueSerializer::new(self.dst).serialize_struct_variant(
            name,
            variant_index,
            variant,
            len,
        )
    }
}
