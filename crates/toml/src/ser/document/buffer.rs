use toml_write::TomlWrite as _;

use crate::alloc_prelude::*;

/// TOML Document serialization buffer
#[derive(Debug, Default)]
pub struct Buffer {
    tables: Vec<Table>,
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

    pub(crate) fn push(&mut self, table: Table) {
        if table.key.is_none() {
            self.tables.insert(0, table);
        } else {
            self.tables.push(table);
        }
    }
}

impl core::fmt::Display for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tables = self.tables.iter().filter(|t| required_table(t));
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
    if table.key.is_none() {
        !table.body.is_empty()
    } else {
        !table.body.is_empty() || !table.has_children
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
    key: Option<Vec<String>>,
    body: String,
    has_children: bool,
}

impl Table {
    pub(crate) fn root() -> Self {
        Self {
            key: None,
            body: String::new(),
            has_children: false,
        }
    }

    pub(crate) fn body_mut(&mut self) -> &mut String {
        &mut self.body
    }

    pub(crate) fn child(&mut self, key: String) -> Self {
        self.has_children = true;
        let mut child = Self {
            key: self.key.clone(),
            body: String::new(),
            has_children: false,
        };
        child.key.get_or_insert_with(Vec::new).push(key);
        child
    }
}

impl core::fmt::Display for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(key) = &self.key {
            f.open_table_header()?;
            let mut key = key.iter();
            if let Some(key) = key.next() {
                write!(f, "{key}")?;
            }
            for key in key {
                f.key_sep()?;
                write!(f, "{key}")?;
            }
            f.close_table_header()?;
            f.newline()?;
        }

        self.body.fmt(f)?;

        Ok(())
    }
}
