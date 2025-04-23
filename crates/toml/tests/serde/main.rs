#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display"))]

mod de_enum;
mod de_errors;
mod general;
mod ser_formatting;
mod ser_formatting_raw;
mod ser_tables_last;
mod spanned;
