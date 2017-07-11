use value::{Array, DateTime, InlineTable, KeyValue, KeyValuePairs, Value};
use decor::{Decor, Formatted, InternalString, Repr};
use std::iter::FromIterator;


fn decorate_array(array: &mut Array) {
    for (i, val) in array.values.iter_mut().enumerate() {
        // [value1, value2, value3]
        if i > 0 {
            decorate(val, " ", "");
        }
    }
}

fn decorate_inline_table(table: &mut InlineTable) {
    let n = table.key_value_pairs.len();
    for (i, (_, kv)) in table.key_value_pairs.iter_mut().enumerate() {
        // { key1 = value1, key2 = value2 }
        kv.key.decor.prefix = InternalString::from(" ");
        if i == n - 1 {
            decorate(&mut kv.value, " ", " ");
        }
    }
}

pub(crate) fn decorate(value: &mut Value, prefix: &str, suffix: &str) {
    let decor = match *value {
        Value::Integer(ref mut f) => &mut f.repr.decor,
        Value::String(ref mut f) => &mut f.repr.decor,
        Value::Float(ref mut f) => &mut f.repr.decor,
        Value::DateTime(ref mut f) => &mut f.repr.decor,
        Value::Boolean(ref mut f) => &mut f.repr.decor,
        Value::Array(ref mut a) => &mut a.decor,
        Value::InlineTable(ref mut t) => &mut t.decor,
    };
    decor.prefix = InternalString::from(prefix);
    decor.suffix = InternalString::from(suffix);
}

pub(crate) fn decorated(mut value: Value, prefix: &str, suffix: &str) -> Value {
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
        Value::DateTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Boolean(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        _ => {}
    };
    val
}

pub(crate) fn to_key_value(key: &str, mut value: Value) -> KeyValue {
    decorate(&mut value, " ", "");
    KeyValue {
        key: Repr {
            decor: Decor {
                prefix: InternalString::from(""),
                suffix: InternalString::from(" "),
            },
            raw_value: key.into(),
        },
        value: value,
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(Formatted::new(
            i,
            Repr::new("".to_string(), i.to_string(), "".to_string()),
        ))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(Formatted::new(
            f,
            Repr::new("".to_string(), f.to_string(), "".to_string()),
        ))
    }
}

impl<'b> From<&'b str> for Value {
    fn from(s: &'b str) -> Self {
        Value::String(Formatted::new(
            s.to_owned(),
            Repr::new("".to_string(), format!("\"{}\"", s), "".to_string()),
        ))
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
            Repr::new("", if b { "true" } else { "false" }, ""),
        ))
    }
}

impl From<DateTime> for Value {
    fn from(d: DateTime) -> Self {
        let s = d.to_string();
        Value::DateTime(Formatted::new(
            d,
            Repr::new("".to_string(), s, "".to_string()),
        ))
    }
}

impl<V: Into<Value>> FromIterator<V> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let v = iter.into_iter().map(|a| a.into());
        let mut array = Array {
            values: v.collect(),
            ..Default::default()
        };
        decorate_array(&mut array);
        Value::Array(array)
    }
}

pub(crate) fn to_key_value_pairs<K, V, I>(iter: I) -> KeyValuePairs
where
    K: Into<String>,
    V: Into<Value>,
    I: IntoIterator<Item = (K, V)>,
{
    let v = iter.into_iter().map(|(a, b)| {
        let s = InternalString::from(a.into());
        (s.clone(), to_key_value(&s, b.into()))
    });
    KeyValuePairs::from_iter(v)
}

impl<K: Into<String>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = InlineTable {
            key_value_pairs: to_key_value_pairs(iter),
            ..Default::default()
        };
        decorate_inline_table(&mut table);
        Value::InlineTable(table)
    }
}
