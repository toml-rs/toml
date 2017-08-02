use std::str::FromStr;
use decor::InternalString;
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
///
/// To parse a key use `FromStr` trait implementation: `"string".parse::<Key>()`.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct Key {
    key: InternalString,
    raw: InternalString,
}

impl FromStr for Key {
    type Err = parser::Error;

    /// Parses a key from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parser::key(parser::Span::new(s));
        match parsed {
            nom::IResult::Done(i, (key, raw)) => if i.fragment.is_empty() {
                Ok(Self {
                    key: key,
                    raw: raw.fragment.into(),
                })
            } else {
                Err(Self::Err::new(parser::ErrorKind::InvalidKey, i))
            },
            nom::IResult::Error(e) => {
                let mut err = parser::to_error(&e);
                err.kind = parser::ErrorKind::InvalidKey;
                Err(err)
            }
            _ => unreachable!("key should be complete"),
        }
    }
}

impl Key {
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
