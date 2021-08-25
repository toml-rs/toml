use crate::datetime::*;
use crate::decor::{Decor, Formatted, InternalString, Repr};
use crate::key::Key;
use crate::parser::strings;
use crate::parser::TomlError;
use crate::table::{Item, KeyValuePairs, TableKeyValue};
use crate::value::{Array, InlineTable, Value};
use combine::stream::position::Stream as PositionStream;
use std::iter::FromIterator;

pub(crate) fn decorate_array(array: &mut Array) {
    for (i, val) in array
        .values
        .iter_mut()
        .filter_map(Item::as_value_mut)
        .enumerate()
    {
        // [value1, value2, value3]
        if i > 0 {
            decorate(val, " ", "");
        } else {
            decorate(val, "", "");
        }
    }
}

pub(crate) fn decorate_inline_table(table: &mut InlineTable) {
    let n = table.len();
    for (i, (key_decor, value)) in table
        .items
        .iter_mut()
        .filter(|&(_, ref kv)| kv.value.is_value())
        .map(|(_, kv)| (&mut kv.key_decor, kv.value.as_value_mut().unwrap()))
        .enumerate()
    {
        // { key1 = value1, key2 = value2 }
        key_decor.prefix = InternalString::from(" ");
        key_decor.suffix = InternalString::from(" ");
        if i == n - 1 {
            decorate(value, " ", " ");
        } else {
            decorate(value, " ", "");
        }
    }
}

pub(crate) fn decorate(value: &mut Value, prefix: &str, suffix: &str) {
    let decor = match *value {
        Value::Integer(ref mut f) => &mut f.decor,
        Value::String(ref mut f) => &mut f.decor,
        Value::Float(ref mut f) => &mut f.decor,
        Value::OffsetDateTime(ref mut f) => &mut f.decor,
        Value::LocalDateTime(ref mut f) => &mut f.decor,
        Value::LocalDate(ref mut f) => &mut f.decor,
        Value::LocalTime(ref mut f) => &mut f.decor,
        Value::Boolean(ref mut f) => &mut f.decor,
        Value::Array(ref mut a) => &mut a.decor,
        Value::InlineTable(ref mut t) => &mut t.decor,
    };
    decor.prefix = InternalString::from(prefix);
    decor.suffix = InternalString::from(suffix);
}

/// Sets the prefix and the suffix for value.
/// # Example
/// ```rust
/// let mut v = toml_edit::Value::from(42);
/// assert_eq!(&v.to_string(), "42");
/// let d = toml_edit::decorated(v, " ", " ");
/// assert_eq!(&d.to_string(), " 42 ");
/// ```
pub fn decorated(mut value: Value, prefix: &str, suffix: &str) -> Value {
    {
        decorate(&mut value, prefix, suffix);
    }
    value
}

pub(crate) fn value(mut val: Value, raw: &str) -> Value {
    match val {
        Value::Integer(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::String(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Float(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::OffsetDateTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalDateTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalDate(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Boolean(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        _ => {}
    };
    decorate(&mut val, "", "");
    val
}

pub(crate) fn to_key_value(key: &str, mut value: Value) -> TableKeyValue {
    decorate(&mut value, " ", "");
    to_table_key_value(key, Item::Value(value))
}

pub(crate) fn to_table_key_value(key: &str, value: Item) -> TableKeyValue {
    TableKeyValue {
        key_repr: Repr::new(key),
        key_decor: default_key_decor(),
        value,
    }
}

pub(crate) fn default_key_decor() -> Decor {
    Decor::new("", " ")
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
        let v = iter.into_iter().map(|a| Item::Value(a.into()));
        let mut array = Array {
            values: v.collect(),
            ..Default::default()
        };
        decorate_array(&mut array);
        Value::Array(array)
    }
}

pub(crate) fn to_key_value_pairs<'k, K, V, I>(iter: I) -> KeyValuePairs
where
    K: Into<&'k Key>,
    V: Into<Value>,
    I: IntoIterator<Item = (K, V)>,
{
    let v = iter.into_iter().map(|(a, b)| {
        let s: &Key = a.into();
        (s.get().into(), to_key_value(s.raw(), b.into()))
    });
    v.collect()
}

impl<'k, K: Into<&'k Key>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = InlineTable {
            items: to_key_value_pairs(iter),
            ..Default::default()
        };
        decorate_inline_table(&mut table);
        Value::InlineTable(table)
    }
}
