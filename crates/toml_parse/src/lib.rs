//! TOML lexer and parser

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[macro_use]
mod macros;

mod source;

pub mod lexer;

pub use source::Raw;
pub use source::Source;
pub use source::SourceIndex;
pub use source::Span;
