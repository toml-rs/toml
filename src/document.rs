use crate::decor::InternalString;
use crate::parser;
use crate::table::{Item, Iter, Table};
use std::str::FromStr;

/// Type representing a TOML document
#[derive(Debug, Clone)]
pub struct Document {
    /// Root should always be `Item::Table`.
    pub root: Item,
    // Trailing comments and whitespaces
    pub(crate) trailing: InternalString,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            root: Item::Table(Table::with_pos(Some(0))),
            trailing: Default::default(),
        }
    }
}

impl Document {
    /// Creates an empty document
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a reference to the root table.
    pub fn as_table(&self) -> &Table {
        self.root.as_table().expect("root should always be a table")
    }

    /// Returns a mutable reference to the root table.
    pub fn as_table_mut(&mut self) -> &mut Table {
        self.root
            .as_table_mut()
            .expect("root should always be a table")
    }

    /// Returns an iterator over the root table.
    pub fn iter(&self) -> Iter<'_> {
        self.root
            .as_table()
            .expect("root should always be a table")
            .iter()
    }
}

impl FromStr for Document {
    type Err = parser::TomlError;

    /// Parses a document from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::TomlParser::parse(s)
    }
}
