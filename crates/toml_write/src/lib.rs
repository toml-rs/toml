//! A low-level interface for writing out TOML
//!
//! # Example
//!
//! ```rust
//! use toml_write::TomlWrite as _;
//!
//! # fn main() -> std::fmt::Result {
//! let mut output = String::new();
//! output.newline()?;
//! output.open_table_header()?;
//! output.key("table")?;
//! output.close_table_header()?;
//! output.newline()?;
//!
//! output.key("key")?;
//! output.space()?;
//! output.keyval_sep()?;
//! output.space()?;
//! output.value("value")?;
//! output.newline()?;
//!
//! assert_eq!(output, r#"
//! [table]
//! key = "value"
//! "#);
//! #   Ok(())
//! # }
//! ```

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod key;
mod string;
mod value;
mod write;

#[cfg(feature = "alloc")]
pub use key::ToTomlKey;
pub use key::WriteTomlKey;
pub use string::TomlKey;
pub use string::TomlKeyBuilder;
pub use string::TomlString;
pub use string::TomlStringBuilder;
#[cfg(feature = "alloc")]
pub use value::ToTomlValue;
pub use value::WriteTomlValue;
pub use write::TomlWrite;
