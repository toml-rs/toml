use std::str::FromStr;
use table::{Table, TableChild};
use decor::InternalString;
use parser;

/// Type representing a TOML document
#[derive(Debug, Clone, Default)]
pub struct Document {
    pub root: Table,
    // Trailing comments and whitespaces
    pub(crate) trailing: InternalString,
}


impl Document {
    /// Creates an empty document
    pub fn new() -> Self {
        Default::default()
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a str, TableChild<'a>)> + 'a> {
        self.root.iter()
    }
}

impl FromStr for Document {
    type Err = parser::TomlError;

    /// Parses a document from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::TomlParser::parse(s)
    }
}
