use linked_hash_map::LinkedHashMap;
use value::{sort_key_value_pairs, InlineTable, KeyValuePairs, Value};
use decor::{Decor, InternalString};
use key::Key;
use array_of_tables::ArrayOfTables;
use formatted::to_key_value;

// TODO: add method to convert a table into inline table
// TODO: impl Index
// TODO: documentation

/// Type representing a TOML non-inline table
#[derive(Clone, Debug, Default)]
pub struct Table {
    pub(crate) key_value_pairs: KeyValuePairs,
    pub(crate) containers: LinkedHashMap<InternalString, (InternalString, Container)>,
    pub(crate) decor: Decor,
    pub(crate) implicit: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum Container {
    Table(Table),
    Array(ArrayOfTables),
}

/// An immutable reference to a child
#[derive(Clone, Debug)]
pub enum TableChild<'a> {
    /// A reference to a child value
    Value(&'a Value),
    /// A reference to a child table
    Table(&'a Table),
    /// A reference to a child array of tables
    Array(&'a ArrayOfTables),
}

pub type Iter<'a> = Box<Iterator<Item = (&'a str, TableChild<'a>)> + 'a>;

/// A mutable reference to a child
#[derive(Debug)]
pub enum TableChildMut<'a> {
    /// A mutable reference to a child value
    Value(&'a mut Value),
    /// A mutable reference to a child table
    Table(&'a mut Table),
    /// A mutable reference to a child array of tables
    Array(&'a mut ArrayOfTables),
}

pub type IterMut<'a> = Box<Iterator<Item = (&'a str, TableChildMut<'a>)> + 'a>;

/// Return type of table.entry("key")
#[derive(Debug)]
pub enum TableEntry<'a> {
    /// A mutable reference to a child value
    Value(&'a mut Value),
    /// A mutable reference to a child table
    Table(&'a mut Table),
    /// A mutable reference to a child array of tables
    Array(&'a mut ArrayOfTables),
    /// A mutable reference to the table itself
    Vacant(&'a mut Table),
}

impl Container {
    fn as_table_please(&mut self) -> &mut Table {
        match *self {
            Container::Table(ref mut t) => t,
            _ => unreachable!("table please"),
        }
    }
    fn as_array_please(&mut self) -> &mut ArrayOfTables {
        match *self {
            Container::Array(ref mut a) => a,
            _ => unreachable!("array please"),
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Self::with_decor(Decor::new("\n", ""))
    }

    pub(crate) fn with_decor(decor: Decor) -> Self {
        Self {
            decor: decor,
            ..Default::default()
        }
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.contains_value(key) || self.contains_container(key)
    }
    pub(crate) fn contains_value(&self, key: &str) -> bool {
        self.key_value_pairs.contains_key(key)
    }
    fn contains_container(&self, key: &str) -> bool {
        self.containers.contains_key(key)
    }
    pub(crate) fn contains_table(&self, key: &str) -> bool {
        match self.containers.get(key) {
            Some(&(_, Container::Table(..))) => true,
            _ => false,
        }
    }
    /// Iterator over key/value pairs, arrays of tables and subtables.
    pub fn iter(&self) -> Iter {
        Box::new(
            self.key_value_pairs
                .iter()
                .map(|(k, kv)| (&k[..], TableChild::Value(&kv.value)))
                .chain(self.containers.iter().map(|(k, c)| {
                    (
                        &k[..],
                        match c.1 {
                            Container::Table(ref t) => TableChild::Table(t),
                            Container::Array(ref a) => TableChild::Array(a),
                        },
                    )
                })),
        )
    }


    /// Mutable iterator over key/value pairs, arrays of tables and subtables.
    pub fn iter_mut(&mut self) -> IterMut {
        Box::new(
            self.key_value_pairs
                .iter_mut()
                .map(|(k, kv)| (&k[..], TableChildMut::Value(&mut kv.value)))
                .chain(self.containers.iter_mut().map(|(k, p)| {
                    (
                        &k[..],
                        match p.1 {
                            Container::Table(ref mut t) => TableChildMut::Table(t),
                            Container::Array(ref mut a) => TableChildMut::Array(a),
                        },
                    )
                })),
        )
    }

    /// Moves all elements from `other` into `Self`, leaving `other` empty.
    ///
    /// **Note**: this method will remove comments from overwritten key/value pairs.
    pub fn append(&mut self, other: &mut InlineTable) {
        while let Some((k, kv)) = other.key_value_pairs.pop_front() {
            self.key_value_pairs.insert(k, kv);
        }
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.remove_container(key) || self.remove_value(key).is_some()
    }

    pub fn remove_value(&mut self, key: &str) -> Option<Value> {
        let val = self.key_value_pairs.remove(key).map(|kv| kv.value);
        if val.is_some() {
            self.set_implicit();
        }
        val
    }

    fn remove_container(&mut self, key: &str) -> bool {
        self.containers.remove(key).is_some()
    }

    /// Sorts Key/Value Pairs of the table,
    /// doesn't affect subtables or subarrays.
    pub fn sort_values(&mut self) {
        sort_key_value_pairs(&mut self.key_value_pairs);
    }

    pub fn len(&self) -> usize {
        self.key_value_pairs.len() + self.containers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get<'a>(&'a self, key: &str) -> Option<TableChild<'a>> {
        if let Some(c) = self.containers.get(key) {
            match c.1 {
                Container::Table(ref t) => Some(TableChild::Table(t)),
                Container::Array(ref a) => Some(TableChild::Array(a)),
            }
        } else if let Some(kv) = self.key_value_pairs.get(key) {
            Some(TableChild::Value(&kv.value))
        } else {
            None
        }
    }

    pub fn entry<'a>(&'a mut self, key: &str) -> TableEntry<'a> {
        if !self.contains_key(key) {
            TableEntry::Vacant(self)
        } else if let Some(c) = self.containers.get_mut(key) {
            match c.1 {
                Container::Table(ref mut t) => TableEntry::Table(t),
                Container::Array(ref mut a) => TableEntry::Array(a),
            }
        } else {
            let kv = self.key_value_pairs
                .get_mut(key)
                .expect("non-lexical lifetimes");
            TableEntry::Value(&mut kv.value)
        }
    }

    pub fn append_value<V: Into<Value>>(&mut self, key: &Key, value: V) -> TableChildMut {
        match self.entry(key.get()) {
            TableEntry::Vacant(me) => {
                TableChildMut::Value(me.append_value_assume_vacant(key, value.into()))
            }
            TableEntry::Array(v) => TableChildMut::Array(v),
            TableEntry::Table(v) => TableChildMut::Table(v),
            TableEntry::Value(v) => TableChildMut::Value(v),
        }
    }

    pub fn append_table(&mut self, key: &Key, table: Table) -> TableChildMut {
        match self.entry(key.get()) {
            TableEntry::Vacant(me) => {
                TableChildMut::Table(me.append_table_assume_vacant(key, table))
            }
            TableEntry::Array(v) => TableChildMut::Array(v),
            TableEntry::Table(v) => TableChildMut::Table(v),
            TableEntry::Value(v) => TableChildMut::Value(v),
        }
    }

    pub fn append_array(&mut self, key: &Key, array: ArrayOfTables) -> TableChildMut {
        match self.entry(key.get()) {
            TableEntry::Vacant(me) => {
                TableChildMut::Array(me.append_array_assume_vacant(key, array))
            }
            TableEntry::Array(v) => TableChildMut::Array(v),
            TableEntry::Table(v) => TableChildMut::Table(v),
            TableEntry::Value(v) => TableChildMut::Value(v),
        }
    }

    /// # Examples
    ///
    /// ```notrust
    /// [target."x86_64/windows.json".dependencies]
    /// ```
    ///
    /// In the document above, tables `target` and `target."x86_64/windows.json"` are implicit.
    ///
    /// ```
    /// # extern crate toml_edit;
    /// # use toml_edit::Document;
    /// #
    /// # fn main() {
    /// let mut doc = "[a]\n[a.b]\n".parse::<Document>().expect("valid toml");
    ///
    /// assert!(doc.root.entry("a").as_table_mut().unwrap().set_implicit());
    /// assert_eq!(doc.to_string(), "[a.b]\n");
    /// # }
    /// ```
    pub fn set_implicit(&mut self) -> bool {
        if self.key_value_pairs.is_empty() && !self.implicit {
            self.implicit = true;
            true
        } else {
            false
        }
    }

    fn append_value_assume_vacant(&mut self, key: &Key, value: Value) -> &mut Value {
        debug_assert!(!self.contains_key(key.get()));
        let kv = to_key_value(key.raw(), value);
        if self.implicit {
            self.implicit = false;
        }
        &mut self.key_value_pairs
            .entry(key.get().into())
            .or_insert(kv)
            .value
    }

    pub(crate) fn append_array_assume_vacant(
        &mut self,
        key: &Key,
        array: ArrayOfTables,
    ) -> &mut ArrayOfTables {
        debug_assert!(!self.contains_key(key.get()));
        let pair = (key.raw().into(), Container::Array(array));
        let result = self.containers
            .entry(key.get().into())
            .or_insert_with(|| pair);
        result.1.as_array_please()
    }

    pub(crate) fn append_table_assume_vacant(&mut self, key: &Key, table: Table) -> &mut Table {
        debug_assert!(!self.contains_key(key.get()));
        let pair = (key.raw().to_owned(), Container::Table(table));
        let result = self.containers
            .entry(key.get().into())
            .or_insert_with(|| pair);
        result.1.as_table_please()
    }
}

// TODO: This should be generated by macro or derive
/// Downcasting
impl<'a> TableEntry<'a> {
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match *self {
            TableEntry::Table(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut ArrayOfTables> {
        match *self {
            TableEntry::Array(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match *self {
            TableEntry::Value(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_vacant_mut(&mut self) -> Option<&mut Table> {
        match *self {
            TableEntry::Vacant(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_table(&self) -> Option<&Table> {
        match *self {
            TableEntry::Table(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&ArrayOfTables> {
        match *self {
            TableEntry::Array(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn as_value(&self) -> Option<&Value> {
        match *self {
            TableEntry::Value(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn as_vacant(&self) -> Option<&Table> {
        match *self {
            TableEntry::Vacant(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }
    pub fn is_vacant(&self) -> bool {
        self.as_vacant().is_some()
    }
}

/// Downcasting
impl<'a> TableChild<'a> {
    pub fn as_table(&self) -> Option<&'a Table> {
        match *self {
            TableChild::Table(me) => Some(me),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&'a ArrayOfTables> {
        match *self {
            TableChild::Array(me) => Some(me),
            _ => None,
        }
    }
    pub fn as_value(&self) -> Option<&'a Value> {
        match *self {
            TableChild::Value(me) => Some(me),
            _ => None,
        }
    }
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    pub fn get(&self, key: &str) -> Option<TableChild<'a>> {
        match *self {
            TableChild::Value(v) if v.is_inline_table() => {
                let t = v.as_inline_table().unwrap();
                t.get(key).map(|v| TableChild::Value(v))
            }
            TableChild::Table(t) => t.get(key),
            _ => None,
        }
    }
}

/// Downcasting
impl<'a> TableChildMut<'a> {
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match *self {
            TableChildMut::Table(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut ArrayOfTables> {
        match *self {
            TableChildMut::Array(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match *self {
            TableChildMut::Value(ref mut me) => Some(me),
            _ => None,
        }
    }
    pub fn as_table(&self) -> Option<&Table> {
        match *self {
            TableChildMut::Table(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&ArrayOfTables> {
        match *self {
            TableChildMut::Array(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn as_value(&self) -> Option<&Value> {
        match *self {
            TableChildMut::Value(ref me) => Some(me),
            _ => None,
        }
    }
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }
}

impl<'a> TableChildMut<'a> {
    pub fn into_entry(self) -> TableEntry<'a> {
        match self {
            TableChildMut::Table(t) => TableEntry::Table(t),
            TableChildMut::Array(t) => TableEntry::Array(t),
            TableChildMut::Value(t) => TableEntry::Value(t),
        }
    }
}

impl<'e> TableEntry<'e> {
    pub fn get(self, s: &str) -> TableEntry<'e> {
        match self {
            TableEntry::Table(table) => table.entry(s),
            TableEntry::Value(&mut Value::InlineTable(ref mut t)) if t.contains_key(s) => {
                t.get_mut(s).map(TableEntry::Value).unwrap()
            }
            a => a,
        }
    }

    pub fn get_or_insert(self, s: &str) -> TableEntry<'e> {
        let key: Key = s.parse().expect("valid key");
        match self {
            TableEntry::Value(&mut Value::InlineTable(ref mut t)) => TableEntry::Value(
                t.try_insert(&key, Value::InlineTable(InlineTable::default())),
            ),
            TableEntry::Vacant(table) | TableEntry::Table(table) => {
                table.append_table(&key, Table::new()).into_entry()
            }
            _ => panic!("can't access key {} in {:?}", s, self),
        }
    }
}
