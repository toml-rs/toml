use std::fmt::Write;
use std::iter::FromIterator;
use std::str::FromStr;

use combine::stream::position::Stream;

use crate::datetime::*;
use crate::key::Key;
use crate::parser;
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
    /// assert_eq!(v.decor().suffix(), None);
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
    /// assert_eq!(v.decor().suffix(), None);
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
    /// assert_eq!(&v.to_string(), " 42");
    /// let d = v.decorated(" ", " ");
    /// assert_eq!(&d.to_string(), " 42 ");
    /// ```
    pub fn decorated(mut self, prefix: &str, suffix: &str) -> Self {
        self.decorate(prefix, suffix);
        self
    }

    pub(crate) fn decorate(&mut self, prefix: &str, suffix: &str) {
        let decor = self.decor_mut();
        *decor = Decor::new(prefix, suffix);
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

impl<'b> From<&'b Value> for Value {
    fn from(s: &'b Value) -> Self {
        s.clone()
    }
}

impl<'b> From<&'b str> for Value {
    fn from(s: &'b str) -> Self {
        let repr = to_string_repr(s, None, None);
        let value = s.to_owned();
        Value::String(Formatted::new(value, repr))
    }
}

impl<'b> From<&'b String> for Value {
    fn from(s: &'b String) -> Self {
        s.as_str().into()
    }
}

impl From<InternalString> for Value {
    fn from(s: InternalString) -> Self {
        Value::from(s.as_str())
    }
}

pub(crate) fn to_string_repr(
    value: &str,
    style: Option<StringStyle>,
    literal: Option<bool>,
) -> Repr {
    let (style, literal) = match (style, literal) {
        (Some(style), Some(literal)) => (style, literal),
        (_, Some(literal)) => (infer_style(value).0, literal),
        (Some(style), _) => (style, infer_style(value).1),
        (_, _) => infer_style(value),
    };

    let mut output = String::with_capacity(value.len() * 2);
    if literal {
        output.push_str(style.literal_start());
        output.push_str(value);
        output.push_str(style.literal_end());
    } else {
        output.push_str(style.standard_start());
        for ch in value.chars() {
            match ch {
                '\u{8}' => output.push_str("\\b"),
                '\u{9}' => output.push_str("\\t"),
                '\u{a}' => match style {
                    StringStyle::NewlineTripple => output.push('\n'),
                    StringStyle::OnelineSingle => output.push_str("\\n"),
                    _ => unreachable!(),
                },
                '\u{c}' => output.push_str("\\f"),
                '\u{d}' => output.push_str("\\r"),
                '\u{22}' => output.push_str("\\\""),
                '\u{5c}' => output.push_str("\\\\"),
                c if c <= '\u{1f}' || c == '\u{7f}' => {
                    write!(output, "\\u{:04X}", ch as u32).unwrap();
                }
                ch => output.push(ch),
            }
        }
        output.push_str(style.standard_end());
    }

    Repr::new_unchecked(output)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum StringStyle {
    NewlineTripple,
    OnelineTripple,
    OnelineSingle,
}

impl StringStyle {
    fn literal_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''\n",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }
    fn literal_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }

    fn standard_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"\n",
            // note: OnelineTripple can happen if do_pretty wants to do
            // '''it's one line'''
            // but literal == false
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }

    fn standard_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"",
            // note: OnelineTripple can happen if do_pretty wants to do
            // '''it's one line'''
            // but literal == false
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }
}

fn infer_style(value: &str) -> (StringStyle, bool) {
    // For doing pretty prints we store in a new String
    // because there are too many cases where pretty cannot
    // work. We need to determine:
    // - if we are a "multi-line" pretty (if there are \n)
    // - if ['''] appears if multi or ['] if single
    // - if there are any invalid control characters
    //
    // Doing it any other way would require multiple passes
    // to determine if a pretty string works or not.
    let mut out = String::with_capacity(value.len() * 2);
    let mut ty = StringStyle::OnelineSingle;
    // found consecutive single quotes
    let mut max_found_singles = 0;
    let mut found_singles = 0;
    let mut prefer_literal = false;
    let mut can_be_pretty = true;

    for ch in value.chars() {
        if can_be_pretty {
            if ch == '\'' {
                found_singles += 1;
                if found_singles >= 3 {
                    can_be_pretty = false;
                }
            } else {
                if found_singles > max_found_singles {
                    max_found_singles = found_singles;
                }
                found_singles = 0
            }
            match ch {
                '\t' => {}
                '\\' => {
                    prefer_literal = true;
                }
                '\n' => ty = StringStyle::NewlineTripple,
                // Escape codes are needed if any ascii control
                // characters are present, including \b \f \r.
                c if c <= '\u{1f}' || c == '\u{7f}' => can_be_pretty = false,
                _ => {}
            }
            out.push(ch);
        } else {
            // the string cannot be represented as pretty,
            // still check if it should be multiline
            if ch == '\n' {
                ty = StringStyle::NewlineTripple;
            }
        }
    }
    if found_singles > 0 && value.ends_with('\'') {
        // We cannot escape the ending quote so we must use """
        can_be_pretty = false;
    }
    if !prefer_literal {
        can_be_pretty = false;
    }
    if !can_be_pretty {
        debug_assert!(ty != StringStyle::OnelineTripple);
        return (ty, false);
    }
    if found_singles > max_found_singles {
        max_found_singles = found_singles;
    }
    debug_assert!(max_found_singles < 3);
    if ty == StringStyle::OnelineSingle && max_found_singles >= 1 {
        // no newlines, but must use ''' because it has ' in it
        ty = StringStyle::OnelineTripple;
    }
    (ty, true)
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(Formatted::new(i, Repr::new_unchecked(i.to_string())))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        let repr = match (f.is_sign_negative(), f.is_nan(), f == 0.0) {
            (true, true, _) => "-nan".to_owned(),
            (false, true, _) => "nan".to_owned(),
            (true, false, true) => "-0.0".to_owned(),
            (false, false, true) => "0.0".to_owned(),
            (_, false, false) => {
                if f % 1.0 == 0.0 {
                    format!("{}.0", f)
                } else {
                    format!("{}", f)
                }
            }
        };
        let repr = Repr::new_unchecked(repr);

        Value::Float(Formatted::new(f, repr))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(Formatted::new(
            b,
            Repr::new_unchecked(if b { "true" } else { "false" }),
        ))
    }
}

impl From<OffsetDateTime> for Value {
    fn from(d: OffsetDateTime) -> Self {
        let s = d.to_string();
        Value::OffsetDateTime(Formatted::new(d, Repr::new_unchecked(s)))
    }
}

impl From<LocalDateTime> for Value {
    fn from(d: LocalDateTime) -> Self {
        let s = d.to_string();
        Value::LocalDateTime(Formatted::new(d, Repr::new_unchecked(s)))
    }
}

impl From<LocalDate> for Value {
    fn from(d: LocalDate) -> Self {
        let s = d.to_string();
        Value::LocalDate(Formatted::new(d, Repr::new_unchecked(s)))
    }
}

impl From<LocalTime> for Value {
    fn from(d: LocalTime) -> Self {
        let s = d.to_string();
        Value::LocalTime(Formatted::new(d, Repr::new_unchecked(s)))
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

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let table: InlineTable = iter.into_iter().collect();
        Value::InlineTable(table)
    }
}

// `key1 = value1`
pub(crate) const DEFAULT_VALUE_DECOR: (&str, &str) = (" ", "");
// `{ key = value }`
pub(crate) const DEFAULT_TRAILING_VALUE_DECOR: (&str, &str) = (" ", " ");
// `[value1, value2]`
pub(crate) const DEFAULT_LEADING_VALUE_DECOR: (&str, &str) = ("", "");
