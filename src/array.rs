use std::iter::FromIterator;
use std::mem;

use crate::repr::{Decor, InternalString};
use crate::{Item, Value};

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

/// An iterator type over `Array`'s values.
pub type ArrayIter<'a> = Box<dyn Iterator<Item = &'a Value> + 'a>;

impl Array {
    /// Returns the length of the underlying Vec.
    /// To get the actual number of items use `a.iter().count()`.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return true iff `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over all values.
    pub fn iter(&self) -> ArrayIter<'_> {
        Box::new(self.values.iter().filter_map(Item::as_value))
    }

    /// Appends a new value to the end of the array, applying default formatting to it.
    pub fn push<V: Into<Value>>(&mut self, v: V) {
        self.value_op(v.into(), true, |items, value| {
            items.push(Item::Value(value))
        })
    }

    /// Appends a new, already formatted value to the end of the array.
    pub fn push_formatted(&mut self, v: Value) {
        self.value_op(v, false, |items, value| items.push(Item::Value(value)))
    }

    /// Inserts an element at the given position within the array, applying default formatting to
    /// it and shifting all values after it to the right.
    ///
    /// Panics if `index > len`.
    pub fn insert<V: Into<Value>>(&mut self, index: usize, v: V) {
        self.value_op(v.into(), true, |items, value| {
            items.insert(index, Item::Value(value))
        })
    }

    /// Inserts an already formatted value at the given position within the array, shifting all
    /// values after it to the right.
    ///
    /// Panics if `index > len`.
    pub fn insert_formatted(&mut self, index: usize, v: Value) {
        self.value_op(v, false, |items, value| {
            items.insert(index, Item::Value(value))
        })
    }

    /// Replaces the element at the given position within the array, preserving existing formatting.
    ///
    /// Panics if `index >= len`.
    pub fn replace<V: Into<Value>>(&mut self, index: usize, v: V) -> Value {
        // Read the existing value's decor and preserve it.
        let existing_decor = self
            .get(index)
            .unwrap_or_else(|| panic!("index {} out of bounds (len = {})", index, self.len()))
            .decor();
        let mut value = v.into();
        *value.decor_mut() = existing_decor.clone();
        self.replace_formatted(index, value)
    }

    /// Replaces the element at the given position within the array with an already formatted value.
    ///
    /// Panics if `index >= len`.
    pub fn replace_formatted(&mut self, index: usize, v: Value) -> Value {
        self.value_op(v, false, |items, value| {
            match mem::replace(&mut items[index], Item::Value(value)) {
                Item::Value(old_value) => old_value,
                x => panic!("non-value item {:?} in an array", x),
            }
        })
    }

    /// Returns a reference to the value at the given index, or `None` if the index is out of
    /// bounds.
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index).and_then(Item::as_value)
    }

    /// Removes the value at the given index.
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

    /// Auto formats the array.
    pub fn fmt(&mut self) {
        decorate_array(self);
    }

    fn value_op<T>(
        &mut self,
        v: Value,
        decorate: bool,
        op: impl FnOnce(&mut Vec<Item>, Value) -> T,
    ) -> T {
        let mut value = v;
        if !self.is_empty() && decorate {
            value.decorate(" ", "");
        } else if decorate {
            value.decorate("", "");
        }
        op(&mut self.values, value)
    }
}

impl<V: Into<Value>> FromIterator<V> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let v = iter.into_iter().map(|a| Item::Value(a.into()));
        let mut array = Array {
            values: v.collect(),
            ..Default::default()
        };
        array.fmt();
        array
    }
}

pub fn decorate_array(array: &mut Array) {
    for (i, val) in array
        .values
        .iter_mut()
        .filter_map(Item::as_value_mut)
        .enumerate()
    {
        // [value1, value2, value3]
        if i > 0 {
            val.decorate(" ", "");
        } else {
            val.decorate("", "");
        }
    }
}
