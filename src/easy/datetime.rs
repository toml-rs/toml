pub use crate::datetime::*;

/// A parsed TOML datetime value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datetime {
    /// An RFC 3339 formatted date-time with offset.
    OffsetDateTime(OffsetDateTime),
    /// An RFC 3339 formatted date-time without offset.
    LocalDateTime(LocalDateTime),
    /// Date portion of an RFC 3339 formatted date-time.
    LocalDate(LocalDate),
    /// Time portion of an RFC 3339 formatted date-time.
    LocalTime(LocalTime),
}
