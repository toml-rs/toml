// WARNING: we're using unsafe code here (raw pointers)
// until a better approach is found
// (not involving replacing every pointer with Rc<RefCell<T>>).
// Note, however, that the public API is safe to use.

// Current design:
// There are four main types: Value, Table, ArrayOfTables and Document(Inner).
// Document allocates all tables in the typed arena.
// Document also contains an intrusive list off all documents.
// Each table contains a link to previous and next table,
// the order is same as in the toml string representation.
// Also each table and array of tables contain a link to the document,
// in order to be able to insert and delete subtables.
// See comments in the Table struct for details about safety.
use intrusive_collections::{LinkedList, LinkedListLink, UnsafeRef};
use std::mem::size_of;
use typed_arena::Arena;
use table::{Header, HeaderKind, Table, TableRef};
use decor::{InternalString, Repr};
use std::borrow::BorrowMut;
use parser;

// clippy is confused here
#[cfg_attr(feature = "cargo-clippy", allow(unneeded_field_pattern))]
pub(crate) mod intrusive {
    use super::{LinkedListLink, Table, UnsafeRef};

    intrusive_adapter!(pub TableAdapter = UnsafeRef<Table>:
                       Table { link: LinkedListLink });
}

type TableList = LinkedList<intrusive::TableAdapter>;

pub(crate) struct DocumentInner {
    arena: Arena<Table>,
    pub(crate) list: TableList,
}

pub struct Document {
    // Note: box is needed in order to preserve
    // the same address of *mut DocumentInner
    // on document move in tables and arrays of tables.
    pub(crate) inner: Box<DocumentInner>,
    // Trailing comments and whitespaces
    pub(crate) trailing: InternalString,
}


impl Default for Document {
    fn default() -> Self {
        Document::new()
    }
}

pub(crate) const ROOT_HEADER: &'static str = "$root$";

impl Document {
    pub fn parse(input: &str) -> Result<Self, parser::Error> {
        parser::Parser::parse(input)
    }

    /// Creates an empty document
    pub fn new() -> Self {
        let list = LinkedList::new(intrusive::TableAdapter::new());
        // reserve space for 10 tables
        let arena = Arena::with_capacity(size_of::<Table>() * 10);
        let inner = DocumentInner::new(arena, list);
        let mut doc = Box::new(inner);
        let header = Header {
            repr: Repr::new("\n", ROOT_HEADER, "\n"),
            kind: HeaderKind::Implicit, // doesn't matter for root
        };
        let table = Table::new(header, doc.borrow_mut());
        // safe, we're converting &mut to *mut
        let ptr = unsafe { UnsafeRef::from_raw(doc.arena.alloc(table)) };
        doc.list.push_front(ptr);
        Self {
            inner: doc,
            trailing: "".into(),
        }
    }

    pub fn root(&self) -> &Table {
        // first element is always the root
        let ptr = self.inner.list.front().get().unwrap() as *const Table;
        // safe, since we're borrowing self
        unsafe { ptr.as_ref().unwrap() }
    }

    pub fn root_mut(&mut self) -> &mut Table {
        // first element is always the root
        let ptr = self.inner.list.front_mut().get().unwrap() as *const Table as *mut Table;
        // safe, since we're borrowing self mutably
        unsafe { ptr.as_mut().unwrap() }
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a str, TableRef<'a>)> + 'a> {
        self.root().iter()
    }
}

impl DocumentInner {
    fn new(arena: Arena<Table>, list: TableList) -> Self {
        Self {
            arena: arena,
            list: list,
        }
    }
    pub(crate) fn insert(&mut self, table: Table, after: *const Table) -> *mut Table {
        let node = self.arena.alloc(table);
        let node_ptr = node as *const Table;
        // safe, we're converting &mut to *mut
        let ptr = unsafe { UnsafeRef::from_raw(node_ptr) };
        // safe, since pointer `after` is always valid
        let mut cursor = unsafe { self.list.cursor_mut_from_ptr(after) };
        // insert into the documents list
        cursor.insert_after(ptr);
        node
    }

    pub(crate) fn move_to_end(&mut self, ptr: *const Table) {
        self.remove(ptr);
        let ptr = unsafe { UnsafeRef::from_raw(ptr) };
        self.list.back_mut().insert_after(ptr);
    }

    pub(crate) fn remove(&mut self, ptr: *const Table) {
        // safe, all pointers are valid list entries
        let mut cursor = unsafe { self.list.cursor_mut_from_ptr(ptr) };
        // remove from the list
        let res = cursor.remove();
        debug_assert!(res.is_some());
    }
}
