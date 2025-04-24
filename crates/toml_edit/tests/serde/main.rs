#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display"))]

mod de_enum;
mod de_errors;
mod general;
mod ser_formatting;
mod ser_formatting_raw;
mod ser_tables_last;
mod spanned;

use serde_spanned::Spanned;
use toml_edit::de::from_str;
use toml_edit::ser::to_string;
use toml_edit::ser::to_string_pretty;
use toml_edit::Date;
use toml_edit::Datetime;
use toml_edit::Time;

use toml_types::Table as SerdeDocument;
use toml_types::Table as SerdeTable;
use toml_types::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, toml_edit::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(s.parse::<toml_edit::de::ValueDeserializer>().unwrap())
}
