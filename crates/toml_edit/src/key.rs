use std::borrow::Cow;
use std::str::FromStr;

use combine::stream::position::Stream;

use crate::encode::{to_string_repr, StringStyle};
use crate::parser;
use crate::parser::is_unquoted_char;
use crate::repr::{Decor, Repr};
use crate::InternalString;

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
    pub(crate) repr: Option<Repr>,
    pub(crate) decor: Decor,
}

impl Key {
    /// Create a new table key
    pub fn new(key: impl Into<InternalString>) -> Self {
        Self {
            key: key.into(),
            repr: None,
            decor: Default::default(),
        }
    }

    /// Parse a TOML key expression
    ///
    /// Unlike `"".parse<Key>()`, this supports dotted keys.
    pub fn parse(repr: &str) -> Result<Vec<Self>, parser::TomlError> {
        Self::try_parse_path(repr)
    }

    pub(crate) fn with_repr_unchecked(mut self, repr: Repr) -> Self {
        self.repr = Some(repr);
        self
    }

    /// While creating the `Key`, add `Decor` to it
    pub fn with_decor(mut self, decor: Decor) -> Self {
        self.decor = decor;
        self
    }

    /// Access a mutable proxy for the `Key`.
    pub fn as_mut(&mut self) -> KeyMut<'_> {
        KeyMut { key: self }
    }

    /// Returns the parsed key value.
    pub fn get(&self) -> &str {
        &self.key
    }

    pub(crate) fn get_internal(&self) -> &InternalString {
        &self.key
    }

    /// Returns the key raw representation.
    pub fn to_repr(&self) -> Cow<Repr> {
        self.repr
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(to_key_repr(&self.key)))
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Auto formats the key.
    pub fn fmt(&mut self) {
        self.repr = Some(to_key_repr(&self.key));
        self.decor.clear();
    }

    fn try_parse_simple(s: &str) -> Result<Key, parser::TomlError> {
        use combine::stream::position::{IndexPositioner, Positioner};
        use combine::EasyParser;

        let b = s.as_bytes();
        let result = parser::simple_key().easy_parse(Stream::new(b));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => Err(parser::TomlError::from_unparsed(
                (&rest.positioner
                    as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                    .position(),
                b,
            )),
            Ok(((raw, key), _)) => Ok(Key::new(key).with_repr_unchecked(Repr::new_unchecked(raw))),
            Err(e) => Err(parser::TomlError::new(e, b)),
        }
    }

    fn try_parse_path(s: &str) -> Result<Vec<Key>, parser::TomlError> {
        use combine::stream::position::{IndexPositioner, Positioner};
        use combine::EasyParser;

        let b = s.as_bytes();
        let result = parser::key_path().easy_parse(Stream::new(b));
        match result {
            Ok((_, ref rest)) if !rest.input.is_empty() => Err(parser::TomlError::from_unparsed(
                (&rest.positioner
                    as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                    .position(),
                b,
            )),
            Ok((keys, _)) => Ok(keys),
            Err(e) => Err(parser::TomlError::new(e, b)),
        }
    }
}

impl std::ops::Deref for Key {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'s> PartialEq<str> for Key {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.get(), other)
    }
}

impl<'s> PartialEq<&'s str> for Key {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.get(), *other)
    }
}

impl<'s> PartialEq<String> for Key {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.get(), other.as_str())
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, ("", ""))
    }
}

impl FromStr for Key {
    type Err = parser::TomlError;

    /// Tries to parse a key from a &str,
    /// if fails, tries as basic quoted key (surrounds with "")
    /// and then literal quoted key (surrounds with '')
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Key::try_parse_simple(s)
    }
}

fn to_key_repr(key: &str) -> Repr {
    if key.as_bytes().iter().copied().all(is_unquoted_char) && !key.is_empty() {
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

impl From<InternalString> for Key {
    fn from(s: InternalString) -> Self {
        Key::new(s)
    }
}

#[doc(hidden)]
impl From<Key> for InternalString {
    fn from(key: Key) -> InternalString {
        key.key
    }
}

/// A mutable reference to a `Key`
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct KeyMut<'k> {
    key: &'k mut Key,
}

impl<'k> KeyMut<'k> {
    /// Returns the parsed key value.
    pub fn get(&self) -> &str {
        self.key.get()
    }

    /// Returns the key raw representation.
    pub fn to_repr(&self) -> Cow<Repr> {
        self.key.to_repr()
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        self.key.decor_mut()
    }

    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        self.key.decor()
    }

    /// Auto formats the key.
    pub fn fmt(&mut self) {
        self.key.fmt()
    }
}

impl<'k> std::ops::Deref for KeyMut<'k> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'s> PartialEq<str> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.get(), other)
    }
}

impl<'s> PartialEq<&'s str> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.get(), *other)
    }
}

impl<'s> PartialEq<String> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.get(), other.as_str())
    }
}

impl<'k> std::fmt::Display for KeyMut<'k> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.key, f)
    }
}
