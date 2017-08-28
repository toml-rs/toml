// #![deny(missing_docs)]
// #![deny(warnings)]

//! # `toml_edit`
//!
//! This crate allows you to parse and modify toml
//! documents, while *mostly* preserving comments, spaces and
//! relative order or items.
//!
//! It is primarily tailored to the needs of [cargo-edit](https://github.com/killercup/cargo-edit/).
//!
//! # Example
//!
//! ```rust
//! extern crate toml_edit;
//!
//! use toml_edit::Document;
//!
//! fn main() {
//!     let toml = r#"
//!       "hello" = 'toml!' # comment
//!       ['a'.b]
//!     "#;
//!     let doc = toml.parse::<Document>();
//!     assert!(doc.is_ok());
//!     assert_eq!(doc.unwrap().to_string(), toml);
//! }
//! ```
#[macro_use]
extern crate combine;
extern crate chrono;
extern crate linked_hash_map;


pub(crate) mod formatted;
mod value;
mod display;
mod parser;
mod decor;
mod key;
mod array_of_tables;
mod table;
mod document;

pub use value::{Array, InlineTable, Value};
pub use key::Key;
pub use parser::TomlError;
pub use table::{Iter, IterMut, Table, TableChild, TableChildMut, TableEntry};
pub use array_of_tables::ArrayOfTables;
pub use document::Document;
