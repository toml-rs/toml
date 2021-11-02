use std::str::FromStr;

use crate::parser;
use crate::table::Iter;
use crate::{InternalString, Table};

/// Type representing a TOML document
#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) root: Table,
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
        &self.root
    }

    /// Returns a mutable reference to the root table.
    pub fn as_table_mut(&mut self) -> &mut Table {
        &mut self.root
    }

    /// Returns an iterator over the root table.
    pub fn iter(&self) -> Iter<'_> {
        self.root.iter()
    }

    /// Set whitespace after last element
    pub fn set_trailing(&mut self, trailing: &str) {
        self.trailing = InternalString::from(trailing);
    }

    /// Whitespace after last element
    pub fn trailing(&self) -> &str {
        self.trailing.as_str()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            root: Table::with_pos(Some(0)),
            trailing: Default::default(),
        }
    }
}

impl FromStr for Document {
    type Err = parser::TomlError;

    /// Parses a document from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::TomlParser::parse(s.as_bytes())
    }
}

impl std::ops::Deref for Document {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}
