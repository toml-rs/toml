use std::collections::HashMap;
use std::collections::hash_map::Entry;
use value::{Value, KeyValuePairs, sort_key_value_pairs};
use decor::{InternalString, Repr};
use key::Key;
use array_of_tables::ArrayOfTables;
use intrusive_collections::LinkedListLink;
use document::{DocumentInner, ROOT_HEADER};
use formatted::{to_key_value, decorate};

// TODO: add method to extract a child table into an inline table
// TODO: add non mutable API
// TODO: impl Index
// TODO: impl Debug
// TODO: documentation

/// Type representing a TOML non-inline table
pub struct Table {
    pub(crate) header: Header,
    pub(crate) key_value_pairs: KeyValuePairs,

    // Value in the map is a  pointer to a linked list node.
    // All nodes are allocated in the typed arena and,
    // therefore, won't relocate.
    // Linked list itself (head of the list) is stored in the Document.
    //
    // Using pointer here is safe, since none of the pointers
    // will outlive the document. Also, we can't use a reference to
    // a table while having a mut ref to its child (granted by brwchk).
    //
    // Note: pointers should be wrapped in NonZero,
    // but this API is nightly-only atm.
    tables: HashMap<InternalString, *mut Table>,
    arrays: HashMap<InternalString, ArrayOfTables>,

    // Intrusive pointers to next and previous table in the document
    pub(crate) link: LinkedListLink,
    // We need this pointer to support (sub)table insertion and deletion
    doc: *mut DocumentInner,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub(crate) struct Header {
    // ```notrust
    // # comment
    // [ a.  'escaped \n'.  c ]
    // ```
    //
    // Corresponds to `Repr("#comment\n", " a.  'escaped \\n'.  c ", "\n")`
    pub repr: Repr,
    pub kind: HeaderKind,
}

// # Example
//
// ```notrust
// [dependencies.serde]
// version = "1.0"
// [[bin]]
// name = "add"
// ```
//
// Header kind of `dependencies` is `Implicit`,
// header kind of `dependencies.serde` is `Standard`,
// header kind of `bin` is `Array`.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub(crate) enum HeaderKind {
    Implicit,
    Standard,
    Array,
}

/// An immutable reference to the child
pub enum TableRef<'a> {
    /// A reference to the child value
    Value(&'a Value),
    /// A reference to the child table
    Table(&'a Table),
    /// A reference to the child array of tables
    Array(&'a ArrayOfTables),
}

/// Return type of table.entry("key")
pub enum TableEntry<'a> {
    /// A reference to the child value
    Value(&'a mut Value),
    /// A reference to the child table
    Table(&'a mut Table),
    /// A reference to the child array of tables
    Array(&'a mut ArrayOfTables),
    /// A reference to the table itself
    Vacant(&'a mut Table),
}

impl Table {
    pub(crate) fn new(header: Header, doc: *mut DocumentInner) -> Self {
        Self {
            header: header,
            doc: doc,
            key_value_pairs: Default::default(),
            tables: Default::default(),
            arrays: Default::default(),
            link: Default::default(),
        }
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.contains_value(key) || self.contains_array(key) || self.contains_table(key)
    }
    pub fn contains_value(&self, key: &str) -> bool {
        self.key_value_pairs.contains_key(key)
    }
    pub fn contains_table(&self, key: &str) -> bool {
        self.tables.contains_key(key)
    }
    pub fn contains_array(&self, key: &str) -> bool {
        self.arrays.contains_key(key)
    }

    // argh, impl trait please
    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a str, TableRef<'a>)> + 'a> {
        Box::new(
            self.key_value_pairs
                .iter()
                .map(|(k, kv)| (&k[..], TableRef::Value(&kv.value)))
                .chain(
                    self.arrays
                        .iter()
                        .map(|(k, a)| (&k[..], TableRef::Array(a))),
                )
                .chain(
                    self.tables
                        .iter()
                        .map(|(k, ptr)| (&k[..], TableRef::Table(unsafe { &**ptr }))),
                ),
        )
    }

    pub fn remove_value<'a>(&'a mut self, key: &str) -> Option<Value> {
        self.key_value_pairs.remove(key).map(|kv| kv.value)
    }

    /// Sorts Key/Value Pairs of the table,
    /// doesn't affect subtables or subarrays.
    pub fn sort_values(&mut self) {
        sort_key_value_pairs(&mut self.key_value_pairs);
    }

    pub fn len(&self) -> usize {
        self.key_value_pairs.len() + self.tables.len() + self.arrays.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn entry<'a>(&'a mut self, key: &str) -> TableEntry<'a> {
        if !self.contains_key(key) {
            TableEntry::Vacant(self)
        } else if let Some(table) = self.tables.get_mut(key) {
            TableEntry::Table(
                // safe, all child pointers are valid
                unsafe { table.as_mut().unwrap() },
            )
        } else if let Some(kv) = self.key_value_pairs.get_mut(key) {
            TableEntry::Value(&mut kv.value)
        } else {
            // argh, non-lexical lifetimes please
            TableEntry::Array(self.arrays.get_mut(key).unwrap())
        }
    }

    pub fn append_value<V: Into<Value>>(&mut self, key: Key, value: V) -> TableEntry {
        match self.entry(key.get()) {
            TableEntry::Vacant(me) => {
                TableEntry::Value(me.insert_value_assume_vacant(key, value.into()))
            }
            otherwise => otherwise,
        }
    }

    /// Tries to remove the table with the given key,
    /// returns true if this operation succeeds, false otherwise.
    pub fn remove_table<'a>(&'a mut self, key: &str) -> bool {
        match self.tables.entry(key.into()) {
            Entry::Vacant(..) => false,
            Entry::Occupied(e) => {
                // safe, doc pointer is always valid
                remove_table_recursive(*e.get(), unsafe { self.doc.as_mut().unwrap() });
                e.remove();
                true
            }
        }
    }

    /// Tries to insert a new table after the last table's child or,
    /// if the table has no children, after the table itself.
    /// If a child with the given key is present, returns a reference
    /// to this child.
    pub fn insert_table(&mut self, key: Key) -> TableEntry {
        let after_ptr = self.tables
            .values()
            .last()
            .map(|p| *p as *const Table)
            .unwrap_or(self as *const Table);
        let header = self.child_header(key.get(), HeaderKind::Standard);
        self.insert_table_with_header(key.get(), header, after_ptr)
    }

    pub fn append_table(&mut self, key: Key) -> TableEntry {
        let header = self.child_header(key.get(), HeaderKind::Standard);
        self.append_table_with_header(key.get(), header)
    }

    pub fn remove_array<'a>(&'a mut self, key: &str) -> bool {
        self.arrays
            .remove(key)
            .map(|mut a| a.remove_all())
            .is_some()
    }

    pub fn insert_array(&mut self, key: Key) -> TableEntry {
        let child_key = self.child_key(key.get());
        match self.entry(key.get()) {
            TableEntry::Vacant(me) => TableEntry::Array(me.insert_array_assume_vacant(&child_key)),
            otherwise => otherwise,
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
    /// let mut doc = Document::parse("[a]\n[a.b]").unwrap();
    /// assert!(doc.root_mut().entry("a").as_table_mut().unwrap().set_implicit());
    /// assert_eq!(doc.to_string(), "[a.b]");
    /// # }
    /// ```
    pub fn set_implicit(&mut self) -> bool {
        if self.key_value_pairs.is_empty() && self.header.kind == HeaderKind::Standard {
            self.header.kind = HeaderKind::Implicit;
            true
        } else {
            false
        }
    }

    fn insert_value_assume_vacant(&mut self, key: Key, value: Value) -> &mut Value {
        debug_assert!(!self.tables.contains_key(key.get()));
        debug_assert!(!self.arrays.contains_key(key.get()));
        let mut kv = to_key_value(key.raw(), value);
        decorate(&mut kv.value, " ", "\n");
        &mut self.key_value_pairs.entry(key.into()).or_insert(kv).value
    }

    pub(crate) fn insert_array_assume_vacant<'a>(&'a mut self, key: &str) -> &'a mut ArrayOfTables {
        let doc = self.doc;
        self.arrays
            .entry(InternalString::from(key))
            .or_insert_with(|| ArrayOfTables::new(key, doc))
    }

    pub(crate) fn move_to_end(&mut self) {
        let doc: &mut DocumentInner = unsafe { self.doc.as_mut().unwrap() };
        doc.move_to_end(self);
    }

    pub(crate) fn child_header(&self, key: &str, kind: HeaderKind) -> Header {
        let child_key = self.child_key(key);
        Header {
            repr: Repr::new("\n", &child_key, "\n"),
            kind: kind,
        }
    }

    pub(crate) fn child_key(&self, key: &str) -> InternalString {
        let prefix = &self.header.repr.raw_value;
        if prefix == ROOT_HEADER {
            key.into()
        } else {
            format!("{}.{}", prefix, key)
        }
    }

    pub(crate) fn append_table_with_header<'a>(
        &'a mut self,
        key: &str,
        header: Header,
    ) -> TableEntry<'a> {
        let doc = unsafe { self.doc.as_ref().unwrap() };
        let back = doc.list.back().get().unwrap();
        self.insert_table_with_header(key, header, back)
    }

    pub(crate) fn insert_table_with_header<'a>(
        &'a mut self,
        key: &str,
        header: Header,
        after_ptr: *const Table,
    ) -> TableEntry<'a> {
        let doc = self.doc;
        match self.entry(key) {
            TableEntry::Vacant(me) => {
                let table = Table::new(header, me.doc);
                // safe, doc pointer is always valid
                let doc: &mut DocumentInner = unsafe { doc.as_mut().unwrap() };
                // insert into the document
                let node_ptr = doc.insert(table, after_ptr);
                // insert into the table
                let result = me.tables.insert(key.into(), node_ptr);
                debug_assert!(result.is_none());
                // safe, because we're borrowing self mutably
                TableEntry::Table(unsafe { node_ptr.as_mut().unwrap() })
            }
            otherwise => otherwise,
        }
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

pub(crate) fn remove_table_recursive(ptr: *mut Table, doc: &mut DocumentInner) {
    // safe, all child pointers are valid
    let child: &mut Table = unsafe { ptr.as_mut().unwrap() };
    let keys: Vec<InternalString> = child.tables.keys().cloned().collect();
    // recursive subtables removal
    for key in keys {
        child.remove_table(&key);
    }
    let keys: Vec<InternalString> = child.arrays.keys().cloned().collect();
    // recursive subarrays removal
    for key in keys {
        child.remove_array(&key);
    }
    // remove from the list
    doc.remove(ptr);
}

