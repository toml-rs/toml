use table::Table;

/// Type representing a TOML array of tables
#[derive(Clone, Debug, Default)]
pub struct ArrayOfTables {
    pub(crate) values: Vec<Table>,
}

impl ArrayOfTables {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns an iterator over tables
    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a Table> + 'a> {
        Box::new(self.values.iter())
    }

    /// Returns an optional reference to the table
    pub fn get(&self, index: usize) -> Option<&Table> {
        self.values.get(index)
    }

    /// Returns an optional mutable reference to the table
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.values.get_mut(index)
    }

    pub fn append(&mut self, table: Table) -> &mut Table {
        self.values.push(table);
        let i = self.len() - 1;
        self.get_mut(i).unwrap()
    }

    pub fn remove(&mut self, index: usize) {
        self.values.remove(index);
    }

    pub fn clear(&mut self) {
        self.values.clear()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
