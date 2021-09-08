pub use crate::datetime::*;

/// A parsed TOML datetime value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Datetime {
    /// An RFC 3339 formatted date-time with offset.
    #[serde(with = "string")]
    OffsetDateTime(OffsetDateTime),
    /// An RFC 3339 formatted date-time without offset.
    #[serde(with = "string")]
    LocalDateTime(LocalDateTime),
    /// Date portion of an RFC 3339 formatted date-time.
    #[serde(with = "string")]
    LocalDate(LocalDate),
    /// Time portion of an RFC 3339 formatted date-time.
    #[serde(with = "string")]
    LocalTime(LocalTime),
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
