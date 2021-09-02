use std::str::FromStr;

use combine::stream::position::Stream;

use crate::parser;
use crate::repr::{Decor, InternalString, Repr};

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
    pub(crate) repr: Repr,
    pub(crate) decor: Decor,
}

impl Key {
    pub(crate) fn new_unchecked(repr: Repr, key: InternalString, decor: Decor) -> Self {
        Self { key, repr, decor }
    }

    pub(crate) fn with_key(key: impl AsRef<str>) -> Self {
        key_string_repr(key.as_ref())
    }

    /// Returns the parsed key value.
    pub fn get(&self) -> &str {
        &self.key
    }

    /// Returns the key raw representation.
    pub fn repr(&self) -> &Repr {
        &self.repr
    }

    /// Returns the key raw representation.
    pub fn into_repr(self) -> Repr {
        self.repr
    }

    /// Returns the key raw representation.
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Returns the key raw representation.
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    fn try_parse(s: &str) -> Result<Key, parser::TomlError> {
        use combine::EasyParser;
        let result = parser::key_parser().easy_parse(Stream::new(s));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(parser::TomlError::from_unparsed(rest.positioner, s))
            }
            Ok(((raw, key), _)) => Ok(Key::new_unchecked(
                Repr::new_unchecked(raw),
                key,
                Decor::default(),
            )),
            Err(e) => Err(parser::TomlError::new(e, s)),
        }
    }
}

impl FromStr for Key {
    type Err = parser::TomlError;

    /// Tries to parse a key from a &str,
    /// if fails, tries as basic quoted key (surrounds with "")
    /// and then literal quoted key (surrounds with '')
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let basic = format!("\"{}\"", s);
        let literal = format!("'{}'", s);
        Key::try_parse(s)
            .or_else(|_| Key::try_parse(&basic))
            .or_else(|_| Key::try_parse(&literal))
    }
}

impl<'b> From<&'b str> for Key {
    fn from(s: &'b str) -> Self {
        Key::with_key(s)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::with_key(s)
    }
}

#[doc(hidden)]
impl From<Key> for InternalString {
    fn from(key: Key) -> InternalString {
        key.key
    }
}

// TODO: clean this mess
pub(crate) fn key_string_repr(s: &str) -> Key {
    if let Ok(k) = Key::try_parse(s) {
        if k.get() == s {
            return k;
        }
    }

    let basic = format!("\"{}\"", s);
    if let Ok(k) = Key::try_parse(&basic) {
        if k.get() == s {
            return k;
        }
    }

    let literal = format!("'{}'", s);
    if let Ok(k) = Key::try_parse(&literal) {
        if k.get() == s {
            return k;
        }
    }

    panic!("toml key parse error: {}", s);
}
