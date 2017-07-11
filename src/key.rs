use decor::{InternalString};
use parser;
use nom;


/// Key as part of a Key/Value Pair or a table header.
///
/// # Examples
///
/// ```notrust
/// [dependencies."nom"]
/// version = "5.0"
/// 'literal key' = "nonsense"
/// "basic string key" = 42
/// ```
///
/// There are 3 types of keys:
///
/// 1. Bare keys (`version` and `dependencies`)
///
/// 2. Basic quoted keys (`"basic string key"` and `"nom"`)
///
/// 3. Literal quoted keys (`'literal key'`)
///
/// For details see [toml spec](https://github.com/toml-lang/toml/#keyvalue-pair).
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct Key {
    key: InternalString,
    raw: InternalString,
}

/// Parses and unwraps the key from a &str
///
/// ```
/// # #[macro_use]
/// # extern crate toml_edit;
/// # use toml_edit::Key;
/// #
/// # fn main() {
/// let version = parse_key!("version");
/// assert_eq!(version.get(), "version");
/// # }
/// ```
#[macro_export]
macro_rules! parse_key {
    ($e:expr) => (
        {
            let key = Key::parse($e);
            assert!(key.is_some());
            key.unwrap()
        }
    );
}


impl Key {
    /// Parses the key from a &str
    pub fn parse(s: &str) -> Option<Self> {
        let parsed = parser::key(parser::Span::new(s));
        match parsed {
            nom::IResult::Done(i, (key, raw)) => {
                if i.fragment.is_empty() {
                    Some(Key { key: key, raw: raw.fragment.into() })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get(&self) -> &str {
        &self.key
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

#[doc(hidden)]
impl Into<InternalString> for Key {
    fn into(self) -> InternalString {
        self.key
    }
}
