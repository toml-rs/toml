#![allow(clippy::unneeded_field_pattern)]
#![allow(clippy::toplevel_ref_arg)]

#[macro_use]
mod macros;
mod array;
pub(crate) mod datetime;
mod document;
mod errors;
mod inline_table;
mod key;
pub(crate) mod numbers;
mod state;
pub(crate) mod strings;
mod table;
mod trivia;
mod value;

pub(crate) use self::document::document;
pub use self::errors::TomlError;
pub(crate) use self::key::is_unquoted_char;
pub(crate) use self::key::key as key_path;
pub(crate) use self::key::simple_key;
pub(crate) use self::value::value as value_parser;

use self::state::ParseState;
