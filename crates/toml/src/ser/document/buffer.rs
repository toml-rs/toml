use toml_writer::TomlWrite as _;

use crate::alloc_prelude::*;

/// TOML Document serialization buffer
#[derive(Debug, Default)]
pub struct Buffer {
    tables: Vec<Option<Table>>,
}

impl Buffer {
    /// Initialize a new serialization buffer
    pub fn new() -> Self {
        Default::default()
    }

    /// Reset the buffer for serializing another document
    pub fn clear(&mut self) {
        self.tables.clear();
    }

    pub(crate) fn root_table(&mut self) -> Table {
        self.new_table(None)
    }

    pub(crate) fn child_table(&mut self, parent: &mut Table, encoded_key: String) -> Table {
        parent.has_children = true;
        let mut encoded_key_path = parent.encoded_key.clone();
        encoded_key_path
            .get_or_insert_with(Vec::new)
            .push(encoded_key);
        self.new_table(encoded_key_path)
    }

    pub(crate) fn element_table(&mut self, parent: &mut Table, encoded_key: String) -> Table {
        let mut table = self.child_table(parent, encoded_key);
        table.array = true;
        table
    }

    pub(crate) fn new_table(&mut self, encoded_key: Option<Vec<String>>) -> Table {
        let pos = self.tables.len();
        let table = Table {
            encoded_key,
            body: String::new(),
            has_children: false,
            pos,
            array: false,
        };
        self.tables.push(None);
        table
    }

    pub(crate) fn push(&mut self, table: Table) {
        let pos = table.pos;
        self.tables[pos] = Some(table);
    }
}

impl core::fmt::Display for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tables = self
            .tables
            .iter()
            .filter_map(|t| t.as_ref())
            .filter(|t| required_table(t));
        if let Some(table) = tables.next() {
            table.fmt(f)?;
        }
        for table in tables {
            f.newline()?;
            table.fmt(f)?;
        }
        Ok(())
    }
}

fn required_table(table: &Table) -> bool {
    if table.encoded_key.is_none() {
        !table.body.is_empty()
    } else {
        table.array || !table.body.is_empty() || !table.has_children
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
    encoded_key: Option<Vec<String>>,
    body: String,
    has_children: bool,
    array: bool,
    pos: usize,
}

impl Table {
    pub(crate) fn body_mut(&mut self) -> &mut String {
        &mut self.body
    }

    pub(crate) fn has_children(&mut self, yes: bool) {
        self.has_children = yes;
    }
}

impl core::fmt::Display for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(encoded_key) = &self.encoded_key {
            if self.array {
                f.open_array_of_tables_header()?;
            } else {
                f.open_table_header()?;
            }
            let mut encoded_keys = encoded_key.iter();
            if let Some(encoded_key) = encoded_keys.next() {
                write!(f, "{encoded_key}")?;
            }
            for encoded_key in encoded_keys {
                f.key_sep()?;
                write!(f, "{encoded_key}")?;
            }
            if self.array {
                f.close_array_of_tables_header()?;
            } else {
                f.close_table_header()?;
            }
            f.newline()?;
        }

        self.body.fmt(f)?;

        Ok(())
    }
}
