use std::iter::FromIterator;
use std::str::FromStr;

use combine::stream::position::Stream;
use combine::stream::position::Stream as PositionStream;

use crate::datetime::*;
use crate::key::Key;
use crate::parser;
use crate::parser::strings;
use crate::parser::TomlError;
use crate::repr::{Decor, Formatted, InternalString, Repr};
use crate::{Array, InlineTable};

/// Representation of a TOML Value (as part of a Key/Value Pair).
#[derive(Debug, Clone)]
pub enum Value {
    /// A string value.
    String(Formatted<String>),
    /// A 64-bit integer value.
    Integer(Formatted<i64>),
    /// A 64-bit float value.
    Float(Formatted<f64>),
    /// A boolean value.
    Boolean(Formatted<bool>),
    /// An RFC 3339 formatted date-time with offset.
    OffsetDateTime(Formatted<OffsetDateTime>),
    /// An RFC 3339 formatted date-time without offset.
    LocalDateTime(Formatted<LocalDateTime>),
    /// Date portion of an RFC 3339 formatted date-time.
    LocalDate(Formatted<LocalDate>),
    /// Time portion of an RFC 3339 formatted date-time.
    LocalTime(Formatted<LocalTime>),
    /// An inline array of values.
    Array(Array),
    /// An inline table of key/value pairs.
    InlineTable(InlineTable),
}

/// Downcasting
impl Value {
    /// Casts `self` to str.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Casts `self` to integer.
    pub fn as_integer(&self) -> Option<i64> {
        match *self {
            Value::Integer(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    /// Casts `self` to float.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// Casts `self` to boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_offset_datetime(&self) -> Option<&OffsetDateTime> {
        match *self {
            Value::OffsetDateTime(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_offset_datetime(&self) -> bool {
        self.as_offset_datetime().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_local_datetime(&self) -> Option<&LocalDateTime> {
        match *self {
            Value::LocalDateTime(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_local_datetime(&self) -> bool {
        self.as_local_datetime().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_local_date(&self) -> Option<&LocalDate> {
        match *self {
            Value::LocalDate(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_local_date(&self) -> bool {
        self.as_local_date().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_local_time(&self) -> Option<&LocalTime> {
        match *self {
            Value::LocalTime(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_local_time(&self) -> bool {
        self.as_local_time().is_some()
    }

    /// Casts `self` to array.
    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Value::Array(ref value) => Some(value),
            _ => None,
        }
    }

    /// Casts `self` to mutable array.
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match *self {
            Value::Array(ref mut value) => Some(value),
            _ => None,
        }
    }

    /// Returns true iff `self` is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Casts `self` to inline table.
    pub fn as_inline_table(&self) -> Option<&InlineTable> {
        match *self {
            Value::InlineTable(ref value) => Some(value),
            _ => None,
        }
    }

    /// Casts `self` to mutable inline table.
    pub fn as_inline_table_mut(&mut self) -> Option<&mut InlineTable> {
        match *self {
            Value::InlineTable(ref mut value) => Some(value),
            _ => None,
        }
    }

    /// Returns true iff `self` is an inline table.
    pub fn is_inline_table(&self) -> bool {
        self.as_inline_table().is_some()
    }
}

impl Value {
    /// Get the decoration of the value.
    /// # Example
    /// ```rust
    /// let v = toml_edit::Value::from(true);
    /// assert_eq!(v.decor().suffix(), "");
    ///```
    pub fn decor(&self) -> &Decor {
        match *self {
            Value::String(ref f) => &f.decor,
            Value::Integer(ref f) => &f.decor,
            Value::Float(ref f) => &f.decor,
            Value::Boolean(ref f) => &f.decor,
            Value::OffsetDateTime(ref f) => &f.decor,
            Value::LocalDateTime(ref f) => &f.decor,
            Value::LocalDate(ref f) => &f.decor,
            Value::LocalTime(ref f) => &f.decor,
            Value::Array(ref a) => &a.decor,
            Value::InlineTable(ref t) => &t.decor,
        }
    }

    /// Get the decoration of the value.
    /// # Example
    /// ```rust
    /// let v = toml_edit::Value::from(true);
    /// assert_eq!(v.decor().suffix(), "");
    ///```
    pub fn decor_mut(&mut self) -> &mut Decor {
        match self {
            Value::String(f) => &mut f.decor,
            Value::Integer(f) => &mut f.decor,
            Value::Float(f) => &mut f.decor,
            Value::Boolean(f) => &mut f.decor,
            Value::OffsetDateTime(f) => &mut f.decor,
            Value::LocalDateTime(f) => &mut f.decor,
            Value::LocalDate(f) => &mut f.decor,
            Value::LocalTime(f) => &mut f.decor,
            Value::Array(a) => &mut a.decor,
            Value::InlineTable(t) => &mut t.decor,
        }
    }

    /// Sets the prefix and the suffix for value.
    /// # Example
    /// ```rust
    /// let mut v = toml_edit::Value::from(42);
    /// assert_eq!(&v.to_string(), "42");
    /// let d = v.decorated(" ", " ");
    /// assert_eq!(&d.to_string(), " 42 ");
    /// ```
    pub fn decorated(mut self, prefix: &str, suffix: &str) -> Self {
        self.decorate(prefix, suffix);
        self
    }

    pub(crate) fn decorate(&mut self, prefix: &str, suffix: &str) {
        let decor = self.decor_mut();
        decor.prefix = InternalString::from(prefix);
        decor.suffix = InternalString::from(suffix);
    }
}

impl FromStr for Value {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use combine::EasyParser;
        let parsed = parser::value_parser().easy_parse(Stream::new(s));
        match parsed {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(Self::Err::from_unparsed(rest.positioner, s))
            }
            Ok((value, _)) => Ok(value),
            Err(e) => Err(Self::Err::new(e, s)),
        }
    }
}

impl<'b> From<&'b str> for Value {
    fn from(s: &'b str) -> Self {
        let (value, raw) = parse_string_guess_delimiters(s);
        Value::String(Formatted::new(value, Repr::new(raw), Decor::new("", "")))
    }
}

impl From<InternalString> for Value {
    fn from(s: InternalString) -> Self {
        Value::from(s.as_ref())
    }
}

macro_rules! try_parse {
    ($s:expr, $p:expr) => {{
        use combine::EasyParser;
        let result = $p.easy_parse(PositionStream::new($s));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(TomlError::from_unparsed(rest.positioner, $s))
            }
            Ok((s, _)) => Ok(s),
            Err(e) => Err(TomlError::new(e.into(), $s)),
        }
    }};
}

// TODO: clean this mess
fn parse_string_guess_delimiters(s: &str) -> (InternalString, InternalString) {
    let basic = format!("\"{}\"", s);
    let literal = format!("'{}'", s);
    let ml_basic = format!("\"\"\"{}\"\"\"", s);
    let ml_literal = format!("'''{}'''", s);
    if let Ok(r) = try_parse!(s, strings::string()) {
        (r, s.into())
    } else if let Ok(r) = try_parse!(&basic[..], strings::basic_string()) {
        (r, basic)
    } else if let Ok(r) = try_parse!(&literal[..], strings::literal_string()) {
        (r.into(), literal.clone())
    } else if let Ok(r) = try_parse!(&ml_basic[..], strings::ml_basic_string()) {
        (r, ml_literal)
    } else {
        try_parse!(&ml_literal[..], strings::ml_literal_string())
            .map(|r| (r, ml_literal))
            .unwrap_or_else(|e| panic!("toml string parse error: {}, {}", e, s))
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(Formatted::new(
            i,
            Repr::new(i.to_string()),
            Decor::new("", ""),
        ))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(Formatted::new(
            f,
            Repr::new(f.to_string()),
            Decor::new("", ""),
        ))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(Formatted::new(
            b,
            Repr::new(if b { "true" } else { "false" }),
            Decor::new("", ""),
        ))
    }
}

impl From<OffsetDateTime> for Value {
    fn from(d: OffsetDateTime) -> Self {
        let s = d.to_string();
        Value::OffsetDateTime(Formatted::new(d, Repr::new(s), Decor::new("", "")))
    }
}

impl From<LocalDateTime> for Value {
    fn from(d: LocalDateTime) -> Self {
        let s = d.to_string();
        Value::LocalDateTime(Formatted::new(d, Repr::new(s), Decor::new("", "")))
    }
}

impl From<LocalDate> for Value {
    fn from(d: LocalDate) -> Self {
        let s = d.to_string();
        Value::LocalDate(Formatted::new(d, Repr::new(s), Decor::new("", "")))
    }
}

impl From<LocalTime> for Value {
    fn from(d: LocalTime) -> Self {
        let s = d.to_string();
        Value::LocalTime(Formatted::new(d, Repr::new(s), Decor::new("", "")))
    }
}

impl From<Array> for Value {
    fn from(array: Array) -> Self {
        Value::Array(array)
    }
}

impl From<InlineTable> for Value {
    fn from(table: InlineTable) -> Self {
        Value::InlineTable(table)
    }
}

impl<V: Into<Value>> FromIterator<V> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let array: Array = iter.into_iter().collect();
        Value::Array(array)
    }
}

impl<'k, K: Into<&'k Key>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let table: InlineTable = iter.into_iter().collect();
        Value::InlineTable(table)
    }
}
