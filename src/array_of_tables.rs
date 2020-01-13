use crate::table::{Item, Table};

/// Type representing a TOML array of tables
#[derive(Clone, Debug, Default)]
pub struct ArrayOfTables {
    // always Vec<Item::Table>
    pub(crate) values: Vec<Item>,
}

/// An iterator type over `ArrayOfTables`'s values.
type ArrayOfTablesIter<'a> = Box<dyn Iterator<Item = &'a Table> + 'a>;

impl ArrayOfTables {
    /// Creates an empty array of tables.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns an iterator over tables.
    pub fn iter(&self) -> ArrayOfTablesIter<'_> {
        Box::new(self.values.iter().filter_map(Item::as_table))
    }

    /// Returns an optional reference to the table.
    pub fn get(&self, index: usize) -> Option<&Table> {
        self.values.get(index).and_then(Item::as_table)
    }

    /// Returns an optional mutable reference to the table.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.values.get_mut(index).and_then(Item::as_table_mut)
    }

    /// Appends a table to the array.
    pub fn append(&mut self, table: Table) -> &mut Table {
        self.values.push(Item::Table(table));
        let i = self.len() - 1;
        self.get_mut(i).unwrap()
    }

    /// Removes a table with the given index.
    pub fn remove(&mut self, index: usize) {
        self.values.remove(index);
    }

    /// Removes all the tables.
    pub fn clear(&mut self) {
        self.values.clear()
    }

    /// Returns the length of the underlying Vec.
    /// To get the actual number of items use `a.iter().count()`.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true iff `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
