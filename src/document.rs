use std::str::FromStr;

use crate::parser;
use crate::repr::InternalString;
use crate::table::Iter;
use crate::{Item, Table};

/// Type representing a TOML document
#[derive(Debug, Clone)]
pub struct Document {
    /// Root should always be `Item::Table`.
    pub root: Item,
    // Trailing comments and whitespaces
    pub(crate) trailing: InternalString,
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

    /// Set whitespace after last element
    pub fn set_trailing(&mut self, trailing: &str) {
        self.trailing = trailing.to_owned();
    }

    /// Whitespace after last element
    pub fn trailing(&self) -> &str {
        self.trailing.as_str()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            root: Item::Table(Table::with_pos(Some(0))),
            trailing: Default::default(),
        }
    }
}

impl FromStr for Document {
    type Err = parser::TomlError;

    /// Parses a document from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::TomlParser::parse(s)
    }
}
