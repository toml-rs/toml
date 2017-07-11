use decor::*;
use table::{remove_table_recursive, Header, HeaderKind, Table};
use document::DocumentInner;

/// Type representing a TOML array of tables
pub struct ArrayOfTables {
    pub(crate) values: Vec<*mut Table>,
    // Example: in [["a". b]] key is "\"a\". b"
    // we need the key in order to support
    // insertion into an empty array.
    key: InternalString,
    // We need this pointer to support
    // table insertion and deletion.
    doc: *mut DocumentInner,
}

impl ArrayOfTables {
    pub(crate) fn new(key: &str, doc: *mut DocumentInner) -> Self {
        Self {
            key: key.into(),
            doc: doc,
            values: Default::default(),
        }
    }
    pub fn get(&self, index: usize) -> Option<&Table> {
        // safe, all pointer are valid
        self.values
            .get(index)
            .map(|t| unsafe { (*t).as_ref().unwrap() })
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a Table> + 'a> {
        Box::new(self.values.iter().map(|t| unsafe { &**t }))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Table> {
        // safe, all pointer are valid
        self.values
            .get_mut(index)
            .map(|t| unsafe { (*t).as_mut().unwrap() })
    }

    /// Appends a new table after the last array's child,
    /// or, if the array is empty, after the document.
    pub fn append(&mut self) -> &mut Table {
        let header = Header {
            repr: Repr::new("\n", &self.key, "\n"),
            kind: HeaderKind::Array,
        };
        let after_ptr = self.values
            .iter()
            .last()
            .map(|p| *p as *const Table)
            .unwrap_or(self.doc_mut().list.back().get().unwrap() as *const Table);
        self.append_with_header(header, after_ptr)
    }

    /// # Panics
    ///
    /// If `index >= self.len()`
    pub fn remove(&mut self, index: usize) {
        let ptr = self.values[index];
        remove_table_recursive(ptr, self.doc_mut());
        self.values.remove(index);
    }

    pub fn remove_all(&mut self) {
        let n = self.len();
        for i in (0..n).rev() {
            self.remove(i);
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn doc_mut(&mut self) -> &mut DocumentInner {
        // safe, doc pointer is always valid
        unsafe { self.doc.as_mut().unwrap() }
    }

    pub(crate) fn append_with_header(
        &mut self,
        header: Header,
        after_ptr: *const Table,
    ) -> &mut Table {
        let table = Table::new(header, self.doc);
        let ptr = self.doc_mut().insert(table, after_ptr);
        self.values.push(ptr);
        let i = self.len() - 1;
        self.get_mut(i).unwrap()
    }
}
