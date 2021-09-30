use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use combine::stream::position::Stream;

use crate::parser;

/// A parsed TOML datetime value
///
/// This structure is intended to represent the datetime primitive type that can
/// be encoded into TOML documents. This type is a parsed version that contains
/// all metadata internally.
///
/// Currently this type is intentionally conservative and only supports
/// `to_string` as an accessor. Over time though it's intended that it'll grow
/// more support!
///
/// Note that if you're using `Deserialize` to deserialize a TOML document, you
/// can use this as a placeholder for where you're expecting a datetime to be
/// specified.
///
/// Also note though that while this type implements `Serialize` and
/// `Deserialize` it's only recommended to use this type with the TOML format,
/// otherwise encoded in other formats it may look a little odd.
///
/// Depending on how the option values are used, this struct will correspond
/// with one of the following four datetimes from the [TOML v1.0.0 spec]:
///
/// | `date`    | `time`    | `offset`  | TOML type          |
/// | --------- | --------- | --------- | ------------------ |
/// | `Some(_)` | `Some(_)` | `Some(_)` | [Offset Date-Time] |
/// | `Some(_)` | `Some(_)` | `None`    | [Local Date-Time]  |
/// | `Some(_)` | `None`    | `None`    | [Local Date]       |
/// | `None`    | `Some(_)` | `None`    | [Local Time]       |
///
/// **1. Offset Date-Time**: If all the optional values are used, `Datetime`
/// corresponds to an [Offset Date-Time]. From the TOML v1.0.0 spec:
///
/// > To unambiguously represent a specific instant in time, you may use an
/// > RFC 3339 formatted date-time with offset.
/// >
/// > ```toml
/// > odt1 = 1979-05-27T07:32:00Z
/// > odt2 = 1979-05-27T00:32:00-07:00
/// > odt3 = 1979-05-27T00:32:00.999999-07:00
/// > ```
/// >
/// > For the sake of readability, you may replace the T delimiter between date
/// > and time with a space character (as permitted by RFC 3339 section 5.6).
/// >
/// > ```toml
/// > odt4 = 1979-05-27 07:32:00Z
/// > ```
///
/// **2. Local Date-Time**: If `date` and `time` are given but `offset` is
/// `None`, `Datetime` corresponds to a [Local Date-Time]. From the spec:
///
/// > If you omit the offset from an RFC 3339 formatted date-time, it will
/// > represent the given date-time without any relation to an offset or
/// > timezone. It cannot be converted to an instant in time without additional
/// > information. Conversion to an instant, if required, is implementation-
/// > specific.
/// >
/// > ```toml
/// > ldt1 = 1979-05-27T07:32:00
/// > ldt2 = 1979-05-27T00:32:00.999999
/// > ```
///
/// **3. Local Date**: If only `date` is given, `Datetime` corresponds to a
/// [Local Date]; see the docs for [`Date`].
///
/// **4. Local Time**: If only `time` is given, `Datetime` corresponds to a
/// [Local Time]; see the docs for [`Time`].
///
/// [TOML v1.0.0 spec]: https://toml.io/en/v1.0.0
/// [Offset Date-Time]: https://toml.io/en/v1.0.0#offset-date-time
/// [Local Date-Time]: https://toml.io/en/v1.0.0#local-date-time
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "dt_serde::DatetimeSerde"))]
#[cfg_attr(feature = "serde", serde(into = "dt_serde::DatetimeSerde"))]
pub struct Datetime {
    /// Optional date.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Date*.
    pub date: Option<Date>,

    /// Optional time.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Time*.
    pub time: Option<Time>,

    /// Optional offset.
    /// Required for: *Offset Date-Time*.
    pub offset: Option<Offset>,
}

#[cfg(feature = "serde")]
pub(crate) mod dt_serde {
    use std::convert::TryFrom;

    use super::Datetime;
    use crate::parser;

    // Currently serde itself doesn't have a datetime type, so we map our `Datetime`
    // to a special valid in the serde data model. Namely one with these special
    // fields/struct names.
    //
    // In general the TOML encoder/decoder will catch this and not literally emit
    // these strings but rather emit datetimes as they're intended.
    pub(crate) const NAME: &str = "$__toml_private_Datetime";
    pub(crate) const FIELD: &str = "$__toml_private_datetime";
    #[derive(
        PartialEq, Eq, PartialOrd, Ord, Clone, Debug, serde::Deserialize, serde::Serialize,
    )]
    #[serde(rename = "$__toml_private_Datetime")]
    pub(crate) struct DatetimeSerde {
        #[serde(rename = "$__toml_private_datetime")]
        field: String,
    }

    impl From<Datetime> for DatetimeSerde {
        fn from(d: Datetime) -> Self {
            Self {
                field: d.to_string(),
            }
        }
    }

    impl TryFrom<DatetimeSerde> for Datetime {
        type Error = parser::TomlError;

        fn try_from(s: DatetimeSerde) -> Result<Self, Self::Error> {
            s.field.parse()
        }
    }
}

impl From<Date> for Datetime {
    fn from(other: Date) -> Self {
        Datetime {
            date: Some(other),
            time: None,
            offset: None,
        }
    }
}

impl From<Time> for Datetime {
    fn from(other: Time) -> Self {
        Datetime {
            date: None,
            time: Some(other),
            offset: None,
        }
    }
}

impl FromStr for Datetime {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use combine::stream::position::{IndexPositioner, Positioner};
        use combine::EasyParser;

        let b = s.as_bytes();
        let result = parser::datetime::date_time().easy_parse(Stream::new(b));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => Err(parser::TomlError::from_unparsed(
                (&rest.positioner
                    as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                    .position(),
                b,
            )),
            Ok((dt, _)) => Ok(dt),
            Err(e) => Err(parser::TomlError::new(e, b)),
        }
    }
}

impl<'a> TryFrom<&'a str> for Datetime {
    type Error = parser::TomlError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for Datetime {
    type Error = parser::TomlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref date) = self.date {
            write!(f, "{}", date)?;
        }
        if let Some(ref time) = self.time {
            if self.date.is_some() {
                write!(f, "T")?;
            }
            write!(f, "{}", time)?;
        }
        if let Some(ref offset) = self.offset {
            write!(f, "{}", offset)?;
        }
        Ok(())
    }
}

/// A parsed TOML date value
///
/// May be part of a [`Datetime`]. Alone, `Date` corresponds to a [Local Date].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the date portion of an RFC 3339 formatted date-time,
/// > it will represent that entire day without any relation to an offset or
/// > timezone.
/// >
/// > ```toml
/// > ld1 = 1979-05-27
/// > ```
///
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Date {
    /// Year: four digits
    pub year: u16,
    /// Month: 1 to 12
    pub month: u8,
    /// Day: 1 to {28, 29, 30, 31} (based on month/year)
    pub day: u8,
}

impl FromStr for Date {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use combine::stream::position::{IndexPositioner, Positioner};
        use combine::EasyParser;

        let b = s.as_bytes();
        let result = parser::datetime::full_date().easy_parse(Stream::new(b));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => Err(parser::TomlError::from_unparsed(
                (&rest.positioner
                    as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                    .position(),
                b,
            )),
            Ok((dt, _)) => Ok(dt),
            Err(e) => Err(parser::TomlError::new(e, b)),
        }
    }
}

impl<'a> TryFrom<&'a str> for Date {
    type Error = parser::TomlError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for Date {
    type Error = parser::TomlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// A parsed TOML time value
///
/// May be part of a [`Datetime`]. Alone, `Time` corresponds to a [Local Time].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the time portion of an RFC 3339 formatted date-time,
/// > it will represent that time of day without any relation to a specific
/// > day or any offset or timezone.
/// >
/// > ```toml
/// > lt1 = 07:32:00
/// > lt2 = 00:32:00.999999
/// > ```
/// >
/// > Millisecond precision is required. Further precision of fractional
/// > seconds is implementation-specific. If the value contains greater
/// > precision than the implementation can support, the additional precision
/// > must be truncated, not rounded.
///
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Time {
    /// Hour: 0 to 23
    pub hour: u8,
    /// Minute: 0 to 59
    pub minute: u8,
    /// Second: 0 to {58, 59, 60} (based on leap second rules)
    pub second: u8,
    /// Nanosecond: 0 to 999_999_999
    pub nanosecond: u32,
}

impl FromStr for Time {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use combine::stream::position::{IndexPositioner, Positioner};
        use combine::EasyParser;

        let b = s.as_bytes();
        let result = parser::datetime::partial_time().easy_parse(Stream::new(b));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => Err(parser::TomlError::from_unparsed(
                (&rest.positioner
                    as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                    .position(),
                b,
            )),
            Ok((dt, _)) => Ok(dt),
            Err(e) => Err(parser::TomlError::new(e, b)),
        }
    }
}

impl<'a> TryFrom<&'a str> for Time {
    type Error = parser::TomlError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for Time {
    type Error = parser::TomlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if self.nanosecond != 0 {
            let s = format!("{:09}", self.nanosecond);
            write!(f, ".{}", s.trim_end_matches('0'))?;
        }
        Ok(())
    }
}

/// A parsed TOML time offset
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Offset {
    /// > A suffix which, when applied to a time, denotes a UTC offset of 00:00;
    /// > often spoken "Zulu" from the ICAO phonetic alphabet representation of
    /// > the letter "Z". --- [RFC 3339 section 2]
    ///
    /// [RFC 3339 section 2]: https://datatracker.ietf.org/doc/html/rfc3339#section-2
    Z,

    /// Offset between local time and UTC
    Custom {
        /// Hours: -12 to +12
        hours: i8,

        /// Minutes: 0 to 59
        minutes: u8,
    },
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Offset::Z => write!(f, "Z"),
            Offset::Custom { hours, minutes } => write!(f, "{:+03}:{:02}", hours, minutes),
        }
    }
}
