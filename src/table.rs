use std::iter::FromIterator;

use linked_hash_map::LinkedHashMap;

use crate::key::Key;
use crate::repr::{Decor, InternalString, Repr};
use crate::value::DEFAULT_VALUE_DECOR;
use crate::{Item, Value};

// TODO: add method to convert a table into inline table

/// Type representing a TOML non-inline table
#[derive(Clone, Debug, Default)]
pub struct Table {
    pub(crate) items: KeyValuePairs,
    // comments/spaces before and after the header
    pub(crate) decor: Decor,
    // whether to hide an empty table
    pub(crate) implicit: bool,
    // used for putting tables back in their original order when serialising.
    // Will be None when the Table wasn't parsed from a file.
    pub(crate) position: Option<usize>,
}

impl Table {
    /// Creates an empty table.
    pub fn new() -> Self {
        Self::with_decor_and_pos(Decor::new("\n", ""), None)
    }

    pub(crate) fn with_pos(position: Option<usize>) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub(crate) fn with_decor_and_pos(decor: Decor, position: Option<usize>) -> Self {
        Self {
            decor,
            position,
            ..Default::default()
        }
    }
}

impl Table {
    /// Returns an iterator over all key/value pairs, including empty.
    pub fn iter(&self) -> Iter<'_> {
        Box::new(self.items.iter().map(|(key, kv)| (&key[..], &kv.value)))
    }

    /// Returns an mutable iterator over all key/value pairs, including empty.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        Box::new(
            self.items
                .iter_mut()
                .map(|(key, kv)| (&key[..], &mut kv.value)),
        )
    }

    /// Returns the number of non-empty items in the table.
    pub fn len(&self) -> usize {
        self.items.iter().filter(|i| !(i.1).value.is_none()).count()
    }

    /// Returns the number of key/value pairs in the table.
    pub fn values_len(&self) -> usize {
        self.items.iter().filter(|i| (i.1).value.is_value()).count()
    }

    /// Returns true iff the table is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the table, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.items.clear()
    }

    /// Given the `key`, return a mutable reference to the value.
    /// If there is no entry associated with the given key in the table,
    /// a `Item::None` value will be inserted.
    ///
    /// To insert to table, use `entry` to return a mutable reference
    /// and set it to the appropriate value.
    pub fn entry<'a>(&'a mut self, key: &str) -> &'a mut Item {
        let key = Key::with_key(key);
        &mut self
            .items
            .entry(key.get().to_owned())
            .or_insert_with(|| TableKeyValue::new(key.repr().to_owned(), Item::None))
            .value
    }

    /// Given the `key`, return a mutable reference to the value.
    /// If there is no entry associated with the given key in the table,
    /// a `Item::None` value will be inserted.
    ///
    /// To insert to table, use `entry` to return a mutable reference
    /// and set it to the appropriate value.
    pub fn entry_format<'a>(&'a mut self, key: &Key) -> &'a mut Item {
        &mut self
            .items
            .entry(key.get().to_owned())
            .or_insert_with(|| TableKeyValue::new(key.repr().to_owned(), Item::None))
            .value
    }

    /// Returns an optional reference to an item given the key.
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Item> {
        self.items.get(key).map(|kv| &kv.value)
    }

    /// Returns an optional mutable reference to an item given the key.
    pub fn get_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut Item> {
        self.items.get_mut(key).map(|kv| &mut kv.value)
    }

    /// Returns true iff the table contains an item with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            !kv.value.is_none()
        } else {
            false
        }
    }

    /// Returns true iff the table contains a table with the given key.
    pub fn contains_table(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_table()
        } else {
            false
        }
    }

    /// Returns true iff the table contains a value with the given key.
    pub fn contains_value(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_value()
        } else {
            false
        }
    }

    /// Returns true iff the table contains an array of tables with the given key.
    pub fn contains_array_of_tables(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_array_of_tables()
        } else {
            false
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_formatted(&mut self, key: &Key, item: Item) -> Option<Item> {
        let kv = TableKeyValue::new(key.repr().to_owned(), item);
        self.items
            .insert(key.get().to_owned(), kv)
            .map(|kv| kv.value)
    }

    /// Removes an item given the key.
    pub fn remove(&mut self, key: &str) -> Option<Item> {
        self.items.remove(key).map(|kv| kv.value)
    }
}

impl Table {
    /// If a table has no key/value pairs and implicit, it will not be displayed.
    ///
    /// # Examples
    ///
    /// ```notrust
    /// [target."x86_64/windows.json".dependencies]
    /// ```
    ///
    /// In the document above, tables `target` and `target."x86_64/windows.json"` are implicit.
    ///
    /// ```
    /// use toml_edit::Document;
    /// let mut doc = "[a]\n[a.b]\n".parse::<Document>().expect("invalid toml");
    ///
    /// doc["a"].as_table_mut().unwrap().set_implicit(true);
    /// assert_eq!(doc.to_string(), "[a.b]\n");
    /// ```
    pub fn set_implicit(&mut self, implicit: bool) {
        self.implicit = implicit;
    }

    /// Sorts Key/Value Pairs of the table.
    ///
    /// Doesn't affect subtables or subarrays.
    pub fn sort_values(&mut self) {
        sort_key_value_pairs(&mut self.items);
    }

    /// Auto formats the table.
    pub fn fmt(&mut self) {
        decorate_table(self);
    }

    /// Returns the decor associated with a given key of the table.
    pub fn decor(&self, key: &str) -> Option<&Decor> {
        self.items.get(key).map(|kv| &kv.key_decor)
    }

    /// Sets the position of the `Table` within the `Document`.
    ///
    /// Setting the position of a table will only affect output when
    /// `Document::to_string_in_original_order` is used.
    pub fn set_position(&mut self, position: usize) {
        self.position = Some(position);
    }

    /// The position of the `Table` within the `Document`.
    ///
    /// Returns `None` if the `Table` was created manually (i.e. not via parsing)
    /// in which case its position is set automatically.
    pub fn position(&self) -> Option<usize> {
        self.position
    }
}

impl<K: Into<Key>, V: Into<Value>> Extend<(K, V)> for Table {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            let key = key.into();
            let value = Item::Value(value.into());
            let value = TableKeyValue::new(key.repr().to_owned(), value);
            self.items.insert(key.into(), value);
        }
    }
}

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for Table {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = Table::new();
        table.extend(iter);
        table
    }
}

pub(crate) type KeyValuePairs = LinkedHashMap<InternalString, TableKeyValue>;

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

fn decorate_table(table: &mut Table) {
    for (key_decor, value) in table
        .items
        .iter_mut()
        .filter(|&(_, ref kv)| kv.value.is_value())
        .map(|(_, kv)| (&mut kv.key_decor, kv.value.as_value_mut().unwrap()))
    {
        // `key1 = value1`
        *key_decor = Decor::new(DEFAULT_KEY_DECOR.0, DEFAULT_KEY_DECOR.1);
        value.decorate(DEFAULT_VALUE_DECOR.0, DEFAULT_VALUE_DECOR.1);
    }
}

// `key1 = value1`
pub(crate) const DEFAULT_KEY_DECOR: (&str, &str) = ("", " ");
pub(crate) const DEFAULT_TABLE_DECOR: (&str, &str) = ("", "");

// TODO: make pub(crate)
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TableKeyValue {
    pub(crate) key_repr: Repr,
    pub(crate) key_decor: Decor,
    pub(crate) value: Item,
}

impl TableKeyValue {
    pub(crate) fn new(key_repr: Repr, value: Item) -> Self {
        TableKeyValue {
            key_repr,
            key_decor: Default::default(),
            value,
        }
    }
}

/// An iterator type over `Table`'s key/value pairs.
pub type Iter<'a> = Box<dyn Iterator<Item = (&'a str, &'a Item)> + 'a>;
/// A mutable iterator type over `Table`'s key/value pairs.
pub type IterMut<'a> = Box<dyn Iterator<Item = (&'a str, &'a mut Item)> + 'a>;

/// This trait represents either a `Table`, or an `InlineTable`.
pub trait TableLike {
    /// Returns an iterator over key/value pairs.
    fn iter(&self) -> Iter<'_>;
    /// Returns the number of nonempty items.
    fn len(&self) -> usize {
        self.iter().filter(|&(_, v)| !v.is_none()).count()
    }
    /// Returns true iff the table is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns an optional reference to an item given the key.
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item>;
    /// Returns an optional mutable reference to an item given the key.
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item>;
}

impl TableLike for Table {
    /// Returns an iterator over all subitems, including `Item::None`.
    fn iter(&self) -> Iter<'_> {
        self.iter()
    }
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item> {
        self.get(key)
    }
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item> {
        self.get_mut(key)
    }
}
