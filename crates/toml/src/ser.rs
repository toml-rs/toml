//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures
//! into TOML documents (as strings). Note that some top-level functions here
//! are also provided at the top of the crate.

/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
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
/// let toml = toml::to_string(&config).unwrap();
/// println!("{}", toml)
/// ```
#[cfg(feature = "display")]
pub fn to_string<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    let mut output = String::new();
    let serializer = Serializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}

/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `Serializer::pretty` for more details.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
///
/// For greater customization, instead serialize to a
/// [`toml_edit::Document`](https://docs.rs/toml_edit/latest/toml_edit/struct.Document.html).
#[cfg(feature = "display")]
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    let mut output = String::new();
    let serializer = Serializer::pretty(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}

/// Errors that can occur when serializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) inner: crate::edit::ser::Error,
}

impl Error {
    pub(crate) fn new(inner: impl std::fmt::Display) -> Self {
        Self {
            inner: crate::edit::ser::Error::Custom(inner.to_string()),
        }
    }

    #[cfg(feature = "display")]
    pub(crate) fn wrap(inner: crate::edit::ser::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedType(t),
        }
    }

    pub(crate) fn unsupported_none() -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedNone,
        }
    }

    pub(crate) fn key_not_string() -> Self {
        Self {
            inner: crate::edit::ser::Error::KeyNotString,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

/// Serialization for TOML documents.
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
#[non_exhaustive]
#[cfg(feature = "display")]
pub struct Serializer<W> {
    dst: W,
    settings: crate::fmt::DocumentFormatter,
}

#[cfg(feature = "display")]
impl<W: std::fmt::Write> Serializer<W> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: W) -> Self {
        Self {
            dst,
            settings: Default::default(),
        }
    }

    /// Apply a default "pretty" policy to the document
    ///
    /// For greater customization, instead serialize to a
    /// [`toml_edit::Document`](https://docs.rs/toml_edit/latest/toml_edit/struct.Document.html).
    pub fn pretty(dst: W) -> Self {
        let mut ser = Serializer::new(dst);
        ser.settings.multiline_array = true;
        ser
    }
}

#[cfg(feature = "display")]
impl<W: std::fmt::Write> serde::ser::Serializer for Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeDocumentArray<W>;
    type SerializeTuple = SerializeDocumentArray<W>;
    type SerializeTupleStruct = SerializeDocumentArray<W>;
    type SerializeTupleVariant = SerializeDocumentArray<W>;
    type SerializeMap = SerializeDocumentTable<W>;
    type SerializeStruct = SerializeDocumentTable<W>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bool(v),
        )
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i8(v),
        )
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i16(v),
        )
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i32(v),
        )
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i64(v),
        )
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u8(v),
        )
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u16(v),
        )
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u32(v),
        )
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u64(v),
        )
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f32(v),
        )
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f64(v),
        )
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_char(v),
        )
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_str(v),
        )
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bytes(v),
        )
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_none(),
        )
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_some(v),
        )
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit(),
        )
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit_struct(name),
        )
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit_variant(
                name,
                variant_index,
                variant,
            ),
        )
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_struct(name, v),
        )
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_variant(
                name,
                variant_index,
                variant,
                value,
            ),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_seq(len)
            .map_err(Error::wrap)?;
        let ser = SerializeDocumentArray::new(self, ser);
        Ok(ser)
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
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_map(len)
            .map_err(Error::wrap)?;
        let ser = SerializeDocumentTable::new(self, ser);
        Ok(ser)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_type(Some(name)))
    }
}

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
#[non_exhaustive]
#[cfg(feature = "display")]
pub struct ValueSerializer<W> {
    dst: W,
}

#[cfg(feature = "display")]
impl<W> ValueSerializer<W> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: W) -> Self {
        Self { dst }
    }
}

#[cfg(feature = "display")]
impl<W: std::fmt::Write> serde::ser::Serializer for ValueSerializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeValueArray<W>;
    type SerializeTuple = SerializeValueArray<W>;
    type SerializeTupleStruct = SerializeValueArray<W>;
    type SerializeTupleVariant = SerializeValueArray<W>;
    type SerializeMap = SerializeValueTable<W>;
    type SerializeStruct = SerializeValueTable<W>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_bool(v),
        )
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_i8(v),
        )
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_i16(v),
        )
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_i32(v),
        )
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_i64(v),
        )
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_u8(v),
        )
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_u16(v),
        )
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_u32(v),
        )
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_u64(v),
        )
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_f32(v),
        )
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_f64(v),
        )
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_char(v),
        )
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_str(v),
        )
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_bytes(v),
        )
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_none(),
        )
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_some(v),
        )
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_unit(),
        )
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_unit_struct(name),
        )
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_unit_variant(
                name,
                variant_index,
                variant,
            ),
        )
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_struct(name, v),
        )
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_variant(
                name,
                variant_index,
                variant,
                value,
            ),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_seq(len)
            .map_err(Error::wrap)?;
        let ser = SerializeValueArray::new(self, ser);
        Ok(ser)
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
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_map(len)
            .map_err(Error::wrap)?;
        let ser = SerializeValueTable::new(self, ser);
        Ok(ser)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_type(Some(name)))
    }
}

#[cfg(feature = "display")]
use internal::*;

#[cfg(feature = "display")]
mod internal {
    use super::*;

    use crate::fmt::DocumentFormatter;

    type InnerSerializeDocumentSeq =
        <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

    #[doc(hidden)]
    pub struct SerializeDocumentArray<W> {
        inner: InnerSerializeDocumentSeq,
        dst: W,
        settings: DocumentFormatter,
    }

    impl<W> SerializeDocumentArray<W> {
        pub(crate) fn new(ser: Serializer<W>, inner: InnerSerializeDocumentSeq) -> Self {
            Self {
                inner,
                dst: ser.dst,
                settings: ser.settings,
            }
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeSeq for SerializeDocumentArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTuple for SerializeDocumentArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTupleVariant for SerializeDocumentArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTupleStruct for SerializeDocumentArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    type InnerSerializeDocumentTable =
        <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

    #[doc(hidden)]
    pub struct SerializeDocumentTable<W> {
        inner: InnerSerializeDocumentTable,
        dst: W,
        settings: DocumentFormatter,
    }

    impl<W> SerializeDocumentTable<W> {
        pub(crate) fn new(ser: Serializer<W>, inner: InnerSerializeDocumentTable) -> Self {
            Self {
                inner,
                dst: ser.dst,
                settings: ser.settings,
            }
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeMap for SerializeDocumentTable<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_key(input).map_err(Error::wrap)
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_value(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeStruct for SerializeDocumentTable<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(key, value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }

    pub(crate) fn write_document(
        mut dst: impl std::fmt::Write,
        mut settings: DocumentFormatter,
        value: Result<toml_edit::Value, crate::edit::ser::Error>,
    ) -> Result<(), Error> {
        let value = value.map_err(Error::wrap)?;
        let mut table = match toml_edit::Item::Value(value).into_table() {
            Ok(i) => i,
            Err(_) => {
                return Err(Error::unsupported_type(None));
            }
        };

        use toml_edit::visit_mut::VisitMut as _;
        settings.visit_table_mut(&mut table);

        let doc: toml_edit::Document = table.into();
        write!(dst, "{}", doc).unwrap();

        Ok(())
    }

    type InnerSerializeValueSeq =
        <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;

    #[doc(hidden)]
    pub struct SerializeValueArray<W> {
        inner: InnerSerializeValueSeq,
        dst: W,
    }

    impl<W> SerializeValueArray<W> {
        pub(crate) fn new(ser: ValueSerializer<W>, inner: InnerSerializeValueSeq) -> Self {
            Self {
                inner,
                dst: ser.dst,
            }
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeSeq for SerializeValueArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTuple for SerializeValueArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTupleVariant for SerializeValueArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeTupleStruct for SerializeValueArray<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    type InnerSerializeValueTable =
        <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

    #[doc(hidden)]
    pub struct SerializeValueTable<W> {
        inner: InnerSerializeValueTable,
        dst: W,
    }

    impl<W> SerializeValueTable<W> {
        pub(crate) fn new(ser: ValueSerializer<W>, inner: InnerSerializeValueTable) -> Self {
            Self {
                inner,
                dst: ser.dst,
            }
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeMap for SerializeValueTable<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_key(input).map_err(Error::wrap)
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_value(value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    impl<W: std::fmt::Write> serde::ser::SerializeStruct for SerializeValueTable<W> {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(key, value).map_err(Error::wrap)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }

    pub(crate) fn write_value(
        mut dst: impl std::fmt::Write,
        value: Result<toml_edit::Value, crate::edit::ser::Error>,
    ) -> Result<(), Error> {
        let value = value.map_err(Error::wrap)?;

        write!(dst, "{}", value).unwrap();

        Ok(())
    }
}
