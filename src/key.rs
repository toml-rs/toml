use std::str::FromStr;
use decor::InternalString;
use parser;
use combine;
use combine::Parser;


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
///
/// To parse a key use `FromStr` trait implementation: `"string".parse::<Key>()`.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct Key {
    key: InternalString,
    raw: InternalString,
}

impl FromStr for Key {
    type Err = parser::TomlError;

    /// Tries to parse a key from a &str,
    /// if fails, tries as basic quoted key (surrounds with "")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quoted = format!("\"{}\"", s);
        let parsed = parser::key()
            .parse(combine::State::new(s))
            .or_else(|_| parser::key().parse(combine::State::new(&quoted)));
        match parsed {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(Self::Err::from_unparsed(rest.positioner, s))
            }
            Ok(((raw, key), _)) => Ok(Key::new(raw, key)),
            Err(e) => Err(Self::Err::new(e, s)),
        }
    }
}

impl Key {
    pub(crate) fn new(raw: &str, key: InternalString) -> Self {
        Self {
            raw: raw.into(),
            key: key,
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
