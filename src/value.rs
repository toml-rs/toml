use chrono::{self, FixedOffset};
use formatted;
use linked_hash_map::LinkedHashMap;
use std::slice::Iter;
use decor::*;


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
    pub(crate) values: Vec<Value>,
    // `trailing` represents whitespaces, newlines
    // and comments in an empty array or after the trailing comma
    pub(crate) trailing: InternalString,
    // prefix before `[` and suffix after `]`
    pub(crate) decor: Decor,
}

/// Type representing a TOML inline table,
/// payload of the `Value::InlineTable` variant
#[derive(Debug, Default, Clone)]
pub struct InlineTable {
    pub(crate) key_value_pairs: KeyValuePairs,
    // `preamble` represents whitespaces in an empty table
    pub(crate) preamble: InternalString,
    // prefix before `{` and suffix after `}`
    pub(crate) decor: Decor,
}

pub(crate) type KeyValuePairs = LinkedHashMap<InternalString, KeyValue>;

/// Type representing a TOML Key/Value Pair
#[derive(Debug, Clone)]
pub(crate) struct KeyValue {
    pub key: Repr,
    pub value: Value,
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


impl Array {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<Value> {
        self.values.iter()
    }

    pub fn push<V: Into<Value>>(&mut self, v: V) -> bool {
        self.push_value(v.into(), true)
    }

    pub fn get(&mut self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    pub fn remove(&mut self, index: usize) -> Value {
        self.values.remove(index)
    }

    pub(crate) fn push_value(&mut self, v: Value, decorate: bool) -> bool {
        let mut value = v.into();
        if !self.is_empty() && decorate {
            formatted::decorate(&mut value, " ", "");
        }
        if self.is_empty() || value.get_type() == self.value_type() {
            self.values.push(value);
            true
        } else {
            false
        }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        if let Some(value) = self.values.get(0) {
            value.get_type()
        } else {
            ValueType::None
        }
    }
}

impl InlineTable {
    pub fn len(&self) -> usize {
        self.key_value_pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a str, &'a Value)> + 'a> {
        Box::new(
            self.key_value_pairs
                .iter()
                .map(|(k, kv)| (&k[..], &kv.value)),
        )
    }

    pub fn sort(&mut self) {
        sort_key_value_pairs(&mut self.key_value_pairs);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.key_value_pairs.contains_key(key)
    }

    pub fn insert<V: Into<Value>>(&mut self, key: &str, value: V) -> Option<Value> {
        let kv = formatted::to_key_value(key, value.into());
        self.key_value_pairs.insert(key.into(), kv).map(|p| p.value)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.key_value_pairs.remove(key).map(|kv| kv.value)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.key_value_pairs.get(key).map(|kv| &kv.value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.key_value_pairs.get_mut(key).map(|kv| &mut kv.value)
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

    fn get_type(&self) -> ValueType {
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

pub(crate) fn sort_key_value_pairs(pairs: &mut KeyValuePairs) {
    let mut keys: Vec<InternalString> = pairs.keys().cloned().collect();
    keys.sort();
    for key in keys {
        pairs.get_refresh(&key);
    }
}
