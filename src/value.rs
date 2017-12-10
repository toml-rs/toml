use std::str::FromStr;
use std::mem;
use chrono::{self, FixedOffset};
use formatted;
use linked_hash_map::LinkedHashMap;
use decor::{Decor, Formatted, InternalString};
use key::Key;
use table::{Item, KeyValuePairs, TableKeyValue};
use parser;
use combine;
use combine::Parser;


/// Representation of a TOML Value (as part of a Key/Value Pair).
#[derive(Debug, Clone)]
pub enum Value {
    Integer(Formatted<i64>),
    String(Formatted<String>),
    Float(Formatted<f64>),
    DateTime(Formatted<DateTime>),
    Boolean(Formatted<bool>),
    Array(Array),
    InlineTable(InlineTable),
}

/// Type representing a TOML Date-Time,
/// payload of the `Value::DateTime` variant's value
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum DateTime {
    OffsetDateTime(chrono::DateTime<FixedOffset>),
    LocalDateTime(chrono::NaiveDateTime),
    LocalDate(chrono::NaiveDate),
    LocalTime(chrono::NaiveTime),
}

/// Type representing a TOML array,
/// payload of the `Value::Array` variant's value
#[derive(Debug, Default, Clone)]
pub struct Array {
    // always Vec<Item::Value>
    pub(crate) values: Vec<Item>,
    // `trailing` represents whitespaces, newlines
    // and comments in an empty array or after the trailing comma
    pub(crate) trailing: InternalString,
    pub(crate) trailing_comma: bool,
    // prefix before `[` and suffix after `]`
    pub(crate) decor: Decor,
}

/// Type representing a TOML inline table,
/// payload of the `Value::InlineTable` variant
#[derive(Debug, Default, Clone)]
pub struct InlineTable {
    pub(crate) items: KeyValuePairs,
    // `preamble` represents whitespaces in an empty table
    pub(crate) preamble: InternalString,
    // prefix before `{` and suffix after `}`
    pub(crate) decor: Decor,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub(crate) enum ValueType {
    None,
    Integer,
    String,
    Float,
    DateTime,
    Boolean,
    Array,
    InlineTable,
}

pub type ArrayIter<'a> = Box<Iterator<Item = &'a Value> + 'a>;

impl Array {
    /// Returns the length of the underlying Vec.
    /// To get the actual number of items use `a.iter().count()`.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return true iff `self.len() == 0`
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> ArrayIter {
        Box::new(self.values.iter().filter_map(|i| i.as_value()))
    }

    pub fn push<V: Into<Value>>(&mut self, v: V) -> bool {
        self.push_value(v.into(), true)
    }

    pub fn get(&mut self, index: usize) -> Option<&Value> {
        self.values.get(index).and_then(Item::as_value)
    }

    pub fn remove(&mut self, index: usize) -> Value {
        let removed = self.values.remove(index);
        if self.is_empty() {
            self.trailing_comma = false;
        }
        match removed {
            Item::Value(v) => v,
            x => panic!("non-value item {:?} in an array", x),
        }
    }

    /// Auto formats the array
    pub fn fmt(&mut self) {
        formatted::decorate_array(self);
    }

    pub(crate) fn push_value(&mut self, v: Value, decorate: bool) -> bool {
        let mut value = v;
        if !self.is_empty() && decorate {
            formatted::decorate(&mut value, " ", "");
        } else if decorate {
            formatted::decorate(&mut value, "", "");
        }
        if self.is_empty() || value.get_type() == self.value_type() {
            self.values.push(Item::Value(value));
            true
        } else {
            false
        }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        if let Some(value) = self.values.get(0).and_then(Item::as_value) {
            value.get_type()
        } else {
            ValueType::None
        }
    }
}

pub type InlineTableIter<'a> = Box<Iterator<Item = (&'a str, &'a Value)> + 'a>;

impl InlineTable {
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> InlineTableIter {
        Box::new(
            self.items
                .iter()
                .filter(|&(_, kv)| kv.value.is_value())
                .map(|(k, kv)| (&k[..], kv.value.as_value().unwrap())),
        )
    }

    pub fn sort(&mut self) {
        sort_key_value_pairs(&mut self.items);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            !kv.value.is_none()
        } else {
            false
        }
    }

    pub fn merge_into(&mut self, other: &mut InlineTable) {
        let items = mem::replace(&mut self.items, KeyValuePairs::new());
        for (k, kv) in items {
            other.items.insert(k, kv);
        }
    }

    pub fn get_or_insert<V: Into<Value>>(&mut self, key: &str, value: V) -> &mut Value {
        let parsed = key.parse::<Key>().expect("invalid key");
        self.items
            .entry(parsed.get().to_owned())
            .or_insert(formatted::to_key_value(key, value.into()))
            .value
            .as_value_mut()
            .expect("non-value type in inline table")
    }

    /// Auto formats the table
    pub fn fmt(&mut self) {
        formatted::decorate_inline_table(self);
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.items
            .remove(key)
            .and_then(|kv| kv.value.as_value().cloned())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key).and_then(|kv| kv.value.as_value())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.items
            .get_mut(key)
            .and_then(|kv| kv.value.as_value_mut())
    }
}

/// Downcasting
impl DateTime {
    pub fn as_offset_date_time(&self) -> Option<&chrono::DateTime<FixedOffset>> {
        match *self {
            DateTime::OffsetDateTime(ref dt) => Some(dt),
            _ => None,
        }
    }
    pub fn as_local_date_time(&self) -> Option<&chrono::NaiveDateTime> {
        match *self {
            DateTime::LocalDateTime(ref dt) => Some(dt),
            _ => None,
        }
    }
    pub fn as_local_date(&self) -> Option<&chrono::NaiveDate> {
        match *self {
            DateTime::LocalDate(ref d) => Some(d),
            _ => None,
        }
    }
    pub fn as_local_time(&self) -> Option<&chrono::NaiveTime> {
        match *self {
            DateTime::LocalTime(ref t) => Some(t),
            _ => None,
        }
    }
    pub fn is_offset_date_time(&self) -> bool {
        self.as_offset_date_time().is_some()
    }
    pub fn is_local_date_time(&self) -> bool {
        self.as_local_date_time().is_some()
    }
    pub fn is_local_date(&self) -> bool {
        self.as_local_date().is_some()
    }
    pub fn is_local_time(&self) -> bool {
        self.as_local_time().is_some()
    }
}

/// Downcasting
impl Value {
    pub fn as_integer(&self) -> Option<i64> {
        match *self {
            Value::Integer(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref value) => Some(value.value()),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_date_time(&self) -> Option<&DateTime> {
        match *self {
            Value::DateTime(ref value) => Some(value.value()),
            _ => None,
        }
    }

    pub fn is_date_time(&self) -> bool {
        self.as_date_time().is_some()
    }

    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Value::Array(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match *self {
            Value::Array(ref mut value) => Some(value),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_inline_table(&self) -> Option<&InlineTable> {
        match *self {
            Value::InlineTable(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn as_inline_table_mut(&mut self) -> Option<&mut InlineTable> {
        match *self {
            Value::InlineTable(ref mut value) => Some(value),
            _ => None,
        }
    }

    pub fn is_inline_table(&self) -> bool {
        self.as_inline_table().is_some()
    }

    pub(crate) fn get_type(&self) -> ValueType {
        match *self {
            Value::Integer(..) => ValueType::Integer,
            Value::String(..) => ValueType::String,
            Value::Float(..) => ValueType::Float,
            Value::DateTime(..) => ValueType::DateTime,
            Value::Boolean(..) => ValueType::Boolean,
            Value::Array(..) => ValueType::Array,
            Value::InlineTable(..) => ValueType::InlineTable,
        }
    }
}

pub(crate) fn sort_key_value_pairs(items: &mut LinkedHashMap<InternalString, TableKeyValue>) {
    let mut keys: Vec<InternalString> = items
        .iter()
        .filter_map(|i| (i.1).value.as_value().map(|_| i.0))
        .cloned()
        .collect();
    keys.sort();
    for key in keys {
        items.get_refresh(&key);
    }
}

impl FromStr for Value {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parser::value().parse(combine::State::new(s));
        match parsed {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(Self::Err::from_unparsed(rest.positioner, s))
            }
            Ok((value, _)) => Ok(value),
            Err(e) => Err(Self::Err::new(e, s)),
        }
    }
}
