use crate::array_of_tables::ArrayOfTables;
use crate::decor::{Decor, InternalString, Repr};
use crate::formatted::{decorated, key_repr};
use crate::key::Key;
use crate::value::{sort_key_value_pairs, Array, DateTime, InlineTable, Value};
use linked_hash_map::LinkedHashMap;

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

pub(crate) type KeyValuePairs = LinkedHashMap<InternalString, TableKeyValue>;

/// Type representing either a value, a table, an array of tables, or none.
#[derive(Debug, Clone)]
pub enum Item {
    /// Type representing none.
    None,
    /// Type representing value.
    Value(Value),
    /// Type representing table.
    Table(Table),
    /// Type representing array of tables.
    ArrayOfTables(ArrayOfTables),
}

impl Default for Item {
    fn default() -> Self {
        Item::None
    }
}

// TODO: make pub(crate)
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TableKeyValue {
    pub(crate) key: Repr,
    pub(crate) value: Item,
}

impl TableKeyValue {
    pub(crate) fn new(key: Repr, value: Item) -> Self {
        TableKeyValue { key, value }
    }
}

/// An iterator type over `Table`'s key/value pairs.
pub type Iter<'a> = Box<dyn Iterator<Item = (&'a str, &'a Item)> + 'a>;
/// A mutable iterator type over `Table`'s key/value pairs.
pub type IterMut<'a> = Box<dyn Iterator<Item = (&'a str, &'a mut Item)> + 'a>;

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

    /// Removes an item given the key.
    pub fn remove(&mut self, key: &str) -> Option<Item> {
        self.items.remove(key).map(|kv| kv.value)
    }

    /// Sorts Key/Value Pairs of the table,
    /// doesn't affect subtables or subarrays.
    pub fn sort_values(&mut self) {
        sort_key_value_pairs(&mut self.items);
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

    /// Given the `key`, return a mutable reference to the value.
    /// If there is no entry associated with the given key in the table,
    /// a `Item::None` value will be inserted.
    ///
    /// To insert to table, use `entry` to return a mutable reference
    /// and set it to the appropriate value.
    pub fn entry<'a>(&'a mut self, key: &str) -> &'a mut Item {
        let parsed_key = key.parse::<Key>().expect("invalid key");
        &mut self
            .items
            .entry(parsed_key.get().to_owned())
            .or_insert(TableKeyValue::new(key_repr(parsed_key.raw()), Item::None))
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

    /// Returns the decor associated with a given key of the table.
    pub fn decor(&self, key: &str) -> Option<&Decor> {
        self.items.get(key).map(|kv| &kv.key.decor)
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

impl Item {
    /// Sets `self` to the given item iff `self` is none and
    /// returns a mutable reference to `self`.
    pub fn or_insert(&mut self, item: Item) -> &mut Item {
        if self.is_none() {
            *self = item
        }
        self
    }
}
// TODO: This should be generated by macro or derive
/// Downcasting
impl Item {
    /// Casts `self` to value.
    pub fn as_value(&self) -> Option<&Value> {
        match *self {
            Item::Value(ref v) => Some(v),
            _ => None,
        }
    }
    /// Casts `self` to table.
    pub fn as_table(&self) -> Option<&Table> {
        match *self {
            Item::Table(ref t) => Some(t),
            _ => None,
        }
    }
    /// Casts `self` to array of tables.
    pub fn as_array_of_tables(&self) -> Option<&ArrayOfTables> {
        match *self {
            Item::ArrayOfTables(ref a) => Some(a),
            _ => None,
        }
    }
    /// Casts `self` to mutable value.
    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match *self {
            Item::Value(ref mut v) => Some(v),
            _ => None,
        }
    }
    /// Casts `self` to mutable table.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match *self {
            Item::Table(ref mut t) => Some(t),
            _ => None,
        }
    }
    /// Casts `self` to mutable array of tables.
    pub fn as_array_of_tables_mut(&mut self) -> Option<&mut ArrayOfTables> {
        match *self {
            Item::ArrayOfTables(ref mut a) => Some(a),
            _ => None,
        }
    }
    /// Returns true iff `self` is a value.
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }
    /// Returns true iff `self` is a table.
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    /// Returns true iff `self` is an array of tables.
    pub fn is_array_of_tables(&self) -> bool {
        self.as_array_of_tables().is_some()
    }
    /// Returns true iff `self` is `None`.
    pub fn is_none(&self) -> bool {
        matches!(*self, Item::None)
    }

    // Duplicate Value downcasting API

    /// Casts `self` to integer.
    pub fn as_integer(&self) -> Option<i64> {
        self.as_value().and_then(Value::as_integer)
    }

    /// Returns true iff `self` is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    /// Casts `self` to float.
    pub fn as_float(&self) -> Option<f64> {
        self.as_value().and_then(Value::as_float)
    }

    /// Returns true iff `self` is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// Casts `self` to boolean.
    pub fn as_bool(&self) -> Option<bool> {
        self.as_value().and_then(Value::as_bool)
    }

    /// Returns true iff `self` is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Casts `self` to str.
    pub fn as_str(&self) -> Option<&str> {
        self.as_value().and_then(Value::as_str)
    }

    /// Returns true iff `self` is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_date_time(&self) -> Option<&DateTime> {
        self.as_value().and_then(Value::as_date_time)
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_date_time(&self) -> bool {
        self.as_date_time().is_some()
    }

    /// Casts `self` to array.
    pub fn as_array(&self) -> Option<&Array> {
        self.as_value().and_then(Value::as_array)
    }

    /// Casts `self` to mutable array.
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        self.as_value_mut().and_then(Value::as_array_mut)
    }

    /// Returns true iff `self` is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Casts `self` to inline table.
    pub fn as_inline_table(&self) -> Option<&InlineTable> {
        self.as_value().and_then(Value::as_inline_table)
    }

    /// Casts `self` to mutable inline table.
    pub fn as_inline_table_mut(&mut self) -> Option<&mut InlineTable> {
        self.as_value_mut().and_then(Value::as_inline_table_mut)
    }

    /// Returns true iff `self` is an inline table.
    pub fn is_inline_table(&self) -> bool {
        self.as_inline_table().is_some()
    }

    /// Casts `self` to either a table or an inline table.
    pub fn as_table_like(&self) -> Option<&dyn TableLike> {
        self.as_table()
            .map(|t| t as &dyn TableLike)
            .or_else(|| self.as_inline_table().map(|t| t as &dyn TableLike))
    }

    /// Returns true iff `self` is either a table, or an inline table.
    pub fn is_table_like(&self) -> bool {
        self.as_table_like().is_some()
    }
}

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

/// Returns a formatted value.
///
/// Since formatting is part of a `Value`, the right hand side of the
/// assignment needs to be decorated with a space before the value.
/// The `value` function does just that.
///
/// # Examples
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use toml_edit::*;
/// let mut table = Table::default();
/// let mut array = Array::default();
/// array.push("hello");
/// array.push("\\, world"); // \ is only allowed in a literal string
/// table["key1"] = value("value1");
/// table["key2"] = value(42);
/// table["key3"] = value(array);
/// assert_eq!(table.to_string(),
/// r#"key1 = "value1"
/// key2 = 42
/// key3 = ["hello", '\, world']
/// "#);
/// ```
pub fn value<V: Into<Value>>(v: V) -> Item {
    Item::Value(decorated(v.into(), " ", ""))
}

/// Returns an empty table.
pub fn table() -> Item {
    Item::Table(Table::new())
}

/// Returns an empty array of tables.
pub fn array() -> Item {
    Item::ArrayOfTables(ArrayOfTables::new())
}
