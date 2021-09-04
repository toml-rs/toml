use std::str::FromStr;

use combine::stream::position::Stream;

use crate::parser;
use crate::parser::is_unquoted_char;
use crate::repr::{Decor, InternalString, Repr};
use crate::value::{to_string_repr, StringStyle};

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
    pub(crate) position: Option<usize>,
}

impl Key {
    /// Create a new table key
    pub fn new(key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        let repr = to_key_repr(key);
        Self::new_unchecked(repr, key.to_owned())
    }

    pub(crate) fn new_unchecked(repr: Repr, key: InternalString) -> Self {
        Self {
            key,
            repr,
            decor: Default::default(),
            position: Default::default(),
        }
    }

    /// While creating the `Key`, add `Decor` to it
    pub fn with_decor(mut self, decor: Decor) -> Self {
        self.decor = decor;
        self
    }

    /// While creating the `Key`, add a table position to it
    pub fn with_position(mut self, position: Option<usize>) -> Self {
        self.position = position;
        self
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
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Returns the key raw representation.
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Get the position relative to other keys in parent table
    pub fn position(&self) -> Option<usize> {
        return self.position;
    }

    /// Set the position relative to other keys in parent table
    pub fn set_position(&mut self, position: Option<usize>) {
        self.position = position;
    }

    fn try_parse(s: &str) -> Result<Key, parser::TomlError> {
        use combine::EasyParser;
        let result = parser::key_parser().easy_parse(Stream::new(s));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(parser::TomlError::from_unparsed(rest.positioner, s))
            }
            Ok(((raw, key), _)) => Ok(Key::new_unchecked(Repr::new_unchecked(raw), key)),
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

fn to_key_repr(key: &str) -> Repr {
    if key.chars().all(is_unquoted_char) && !key.is_empty() {
        Repr::new_unchecked(key)
    } else {
        to_string_repr(key, Some(StringStyle::OnelineSingle), Some(false))
    }
}

impl<'b> From<&'b str> for Key {
    fn from(s: &'b str) -> Self {
        Key::new(s)
    }
}

impl<'b> From<&'b String> for Key {
    fn from(s: &'b String) -> Self {
        Key::new(s)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::new(s)
    }
}

#[doc(hidden)]
impl From<Key> for InternalString {
    fn from(key: Key) -> InternalString {
        key.key
    }
}
