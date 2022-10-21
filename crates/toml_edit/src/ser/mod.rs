//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures into TOML.

mod array;
mod item;
mod key;
mod pretty;
mod table;

pub(crate) use array::*;
pub(crate) use item::*;
pub(crate) use key::*;
pub(crate) use table::*;

use crate::visit_mut::VisitMut;

/// Errors that can occur when deserializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            kind: ErrorKind::Custom(msg.to_string()),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.kind.fmt(formatter)
    }
}

impl From<crate::TomlError> for Error {
    fn from(e: crate::TomlError) -> Error {
        Self::custom(e)
    }
}

impl From<Error> for crate::TomlError {
    fn from(e: Error) -> crate::TomlError {
        Self::custom(e.to_string())
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ErrorKind {
    UnsupportedType,
    UnsupportedNone,
    KeyNotString,
    Custom(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UnsupportedType => "unsupported Rust type".fmt(formatter),
            ErrorKind::UnsupportedNone => "unsupported None value".fmt(formatter),
            ErrorKind::KeyNotString => "map key was not a string".fmt(formatter),
            ErrorKind::Custom(s) => s.fmt(formatter),
        }
    }
}

/// Serialize the given data structure as a TOML byte vector.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Error>
where
    T: serde::ser::Serialize,
{
    to_string(value).map(|e| e.into_bytes())
}

/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
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
pub fn to_string<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    to_document(value).map(|e| e.to_string())
}

/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `Serializer::pretty` for more details.
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    let mut document = to_document(value)?;
    pretty::Pretty.visit_document_mut(&mut document);
    Ok(document.to_string())
}

/// Serialize the given data structure into a TOML document.
///
/// This would allow custom formatting to be applied, mixing with format preserving edits, etc.
pub fn to_document<T: ?Sized>(value: &T) -> Result<crate::Document, Error>
where
    T: serde::ser::Serialize,
{
    let item = to_item(value)?;
    let root = item.into_table().map_err(|_| ErrorKind::UnsupportedType)?;
    Ok(root.into())
}

/// Serialize the given data structure into a TOML data structure
///
/// This would allow custom formatting to be applied, mixing with format preserving edits, etc.
pub fn to_item<T: ?Sized>(value: &T) -> Result<crate::Item, Error>
where
    T: serde::ser::Serialize,
{
    let item = value.serialize(Serializer::new())?;
    Ok(item)
}

pub use item::ItemSerializer as Serializer;
