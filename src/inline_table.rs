use std::iter::FromIterator;
use std::mem;

use crate::key::Key;
use crate::repr::{Decor, InternalString, Repr};
use crate::table::{sort_key_value_pairs, Iter, KeyValuePairs, TableKeyValue, TableLike};
use crate::{Item, Value};

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

/// An iterator type over key/value pairs of an inline table.
pub type InlineTableIter<'a> = Box<dyn Iterator<Item = (&'a str, &'a Value)> + 'a>;

impl InlineTable {
    /// Returns the number of key/value pairs.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns true iff the table is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over key/value pairs.
    pub fn iter(&self) -> InlineTableIter<'_> {
        Box::new(
            self.items
                .iter()
                .filter(|&(_, kv)| kv.value.is_value())
                .map(|(k, kv)| (&k[..], kv.value.as_value().unwrap())),
        )
    }

    /// Sorts the key/value pairs by key.
    pub fn sort(&mut self) {
        sort_key_value_pairs(&mut self.items);
    }

    /// Returns true iff the table contains given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            !kv.value.is_none()
        } else {
            false
        }
    }

    /// Merges the key/value pairs into the `other` table leaving
    /// `self` empty.
    pub fn merge_into(&mut self, other: &mut InlineTable) {
        let items = mem::replace(&mut self.items, KeyValuePairs::new());
        for (k, kv) in items {
            other.items.insert(k, kv);
        }
    }

    /// Inserts a key/value pair if the table does not contain the key.
    /// Returns a mutable reference to the corresponding value.
    pub fn get_or_insert<V: Into<Value>>(&mut self, key: &str, value: V) -> &mut Value {
        let key = Key::with_key(key);
        self.items
            .entry(key.get().to_owned())
            .or_insert(to_key_value(key.into_repr(), value.into()))
            .value
            .as_value_mut()
            .expect("non-value type in inline table")
    }

    /// Auto formats the table.
    pub fn fmt(&mut self) {
        decorate_inline_table(self);
    }

    /// Removes a key/value pair given the key.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.items
            .remove(key)
            .and_then(|kv| kv.value.as_value().cloned())
    }

    /// Return an optional reference to the value at the given the key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key).and_then(|kv| kv.value.as_value())
    }

    /// Return an optional mutable reference to the value at the given the key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.items
            .get_mut(key)
            .and_then(|kv| kv.value.as_value_mut())
    }
}

fn decorate_inline_table(table: &mut InlineTable) {
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
            value.decorate(" ", " ");
        } else {
            value.decorate(" ", "");
        }
    }
}

impl TableLike for InlineTable {
    fn iter(&self) -> Iter<'_> {
        Box::new(self.items.iter().map(|(key, kv)| (&key[..], &kv.value)))
    }
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item> {
        self.items.get(key).map(|kv| &kv.value)
    }
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item> {
        self.items.get_mut(key).map(|kv| &mut kv.value)
    }
}

impl<'k, K: Into<&'k Key>, V: Into<Value>> FromIterator<(K, V)> for InlineTable {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = InlineTable {
            items: to_key_value_pairs(iter),
            ..Default::default()
        };
        table.fmt();
        table
    }
}

fn to_key_value_pairs<'k, K, V, I>(iter: I) -> KeyValuePairs
where
    K: Into<&'k Key>,
    V: Into<Value>,
    I: IntoIterator<Item = (K, V)>,
{
    let v = iter.into_iter().map(|(a, b)| {
        let s: &Key = a.into();
        (s.get().into(), to_key_value(s.repr().to_owned(), b.into()))
    });
    v.collect()
}

fn to_key_value(key_repr: Repr, mut value: Value) -> TableKeyValue {
    value.decorate(" ", "");
    TableKeyValue {
        key_repr,
        key_decor: default_inline_key_decor(),
        value: Item::Value(value),
    }
}

pub(crate) fn default_inline_key_decor() -> Decor {
    Decor::new(" ", " ")
}
