// #![deny(missing_docs)]
#![deny(warnings)]

//! # `toml_edit`
//!
//! This crate allows you to parse and modify toml
//! documents, while preserving comments, spaces and
//! relative order or items.
//!
//! # Example
//!
//! ```rust
//! extern crate toml_edit;
//!
//! use toml_edit::Document;
//!
//! fn main() {
//!     let toml = r#"hello = 'toml!' # comment"#;
//!     let doc = Document::parse(toml);
//!     assert!(doc.is_ok());
//!     assert_eq!(doc.unwrap().to_string(), toml);
//! }
//! ```
#[macro_use]
extern crate intrusive_collections;
#[macro_use]
extern crate nom;
extern crate nom_locate;
extern crate chrono;
extern crate linked_hash_map;
extern crate typed_arena;


pub(crate) mod formatted;
mod value;
mod display;
mod parser;
mod decor;
mod key;
mod array_of_tables;
mod table;
mod document;

pub use display::*;
pub use value::{Value, Array, InlineTable};
pub use key::Key;
pub use parser::Error;
pub use table::{Table, TableEntry, TableRef};
pub use array_of_tables::ArrayOfTables;
pub use document::Document;
