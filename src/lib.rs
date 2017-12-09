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
//! use toml_edit::{Document, value};
//!
//! fn main() {
//!     let toml = r#"
//! "hello" = 'toml!' # comment
//! ['a'.b]
//!     "#;
//!     let mut doc = toml.parse::<Document>().expect("invalid doc");
//!     assert_eq!(doc.to_string(), toml);
//!     // let's add a new key/value pair inside a.b: c = {d = "hello"}
//!     doc["a"]["b"]["c"]["d"] = value("hello");
//!     // autoformat inline table a.b.c: { d = "hello" }
//!     doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
//!     let expected = r#"
//! "hello" = 'toml!' # comment
//! ['a'.b]
//! c = { d = "hello" }
//!     "#;
//!     assert_eq!(doc.to_string(), expected);
//! }
//! ```
extern crate chrono;
#[macro_use]
extern crate combine;
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
mod index;

pub use value::{Array, InlineTable, Value};
pub use key::Key;
pub use parser::TomlError;
pub use table::{array, table, value, Item, Iter, Table};
pub use array_of_tables::ArrayOfTables;
pub use document::Document;
