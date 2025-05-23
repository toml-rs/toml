//! TOML lexer and parser

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

#[cfg(feature = "debug")]
pub(crate) mod debug;
mod error;
mod source;

pub mod decode;
pub mod lexer;
pub mod parser;

pub use error::ErrorSink;
pub use error::Expected;
pub use error::ParseError;
pub use source::Raw;
pub use source::Source;
pub use source::SourceIndex;
pub use source::Span;
