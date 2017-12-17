#![deny(missing_docs)]
#![deny(warnings)]

//! # `toml_edit`
//!
//! This crate allows you to parse and modify toml
//! documents, while preserving comments, spaces* and
//! relative order* or items.
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
//!
//! ## Limitations
//!
//! *Things it does not preserve:
//! 1. Different quotes and spaces around the same table key, e.g.
//!
//! ```text
//! [ 'a'. b]
//! [ "a"  .c]
//! [a.d]
//! ```
//!
//! will be represented as (spaces are removed, the first encountered quote type is used)
//!
//! ```text
//! ['a'.b]
//! ['a'.c]
//! ['a'.d]
//! ```
//!
//! 2. Children tables before parent table (tables are reordered, see [test]).
//! 3. Scattered array of tables (tables are reordered, see [test]).
//!
//! The reason behind the first limitation is that `Table` does not store its header,
//! allowing us to safely swap two tables
//! (we store a mapping in each table: child key -> child table).
//!
//! This last two limitations allow us to represent a toml document as a tree-like data structure,
//! which enables easier implementation of editing operations
//! and an easy to use and type-safe API.
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
pub use table::{array, table, value, Item, Iter, Table, TableLike};
pub use array_of_tables::ArrayOfTables;
pub use document::Document;
