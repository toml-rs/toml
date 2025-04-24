#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display"))]

mod de_enum;
mod de_errors;
mod general;
mod ser_formatting;
mod ser_formatting_raw;
mod ser_tables_last;
mod spanned;

use toml::from_str;
use toml::to_string;
use toml::to_string_pretty;
use toml::value::Date;
use toml::value::Datetime;
use toml::value::Time;
use toml::Spanned;

use toml::Table as SerdeDocument;
use toml::Table as SerdeTable;
use toml::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, toml::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(toml::de::ValueDeserializer::new(s))
}
