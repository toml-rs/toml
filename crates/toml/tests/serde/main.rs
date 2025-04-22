#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display"))]

mod de_errors;
mod enum_external_deserialize;
mod formatting;
mod general;
mod pretty;
mod spanned;
mod tables_last;
