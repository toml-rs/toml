#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

mod de_enum;
mod de_errors;
mod de_key;
mod general;
mod ser_enum;
mod ser_key;
mod ser_tables_last;
mod ser_to_string;
mod ser_to_string_pretty;
mod spanned;

use toml::Spanned;
use toml::from_str;
use toml::to_string;
use toml::to_string_pretty;
use toml::value::Date;
use toml::value::Datetime;
use toml::value::Time;

use toml::Table as SerdeDocument;
use toml::Table as SerdeTable;
use toml::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, toml::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(toml::de::ValueDeserializer::parse(s)?)
}

fn to_string_value<T>(value: &T) -> Result<String, toml::ser::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut output = String::new();
    let serializer = toml::ser::ValueSerializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}
