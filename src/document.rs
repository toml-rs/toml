use std::str::FromStr;
use table::{Item, Iter, Table};
use decor::InternalString;
use parser;

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
            root: Item::Table(Table::default()),
            trailing: Default::default(),
        }
    }
}


impl Document {
    /// Creates an empty document
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_table(&self) -> &Table {
        self.root.as_table().expect("root should always be a table")
    }

    pub fn as_table_mut(&mut self) -> &mut Table {
        self.root
            .as_table_mut()
            .expect("root should always be a table")
    }

    pub fn iter(&self) -> Iter {
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
