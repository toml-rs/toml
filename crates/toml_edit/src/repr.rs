use std::borrow::Cow;

use crate::InternalString;

/// A value together with its `to_string` representation,
/// including surrounding it whitespaces and comments.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Formatted<T> {
    value: T,
    repr: Option<Repr>,
    decor: Decor,
}

impl<T> Formatted<T>
where
    T: ValueRepr,
{
    /// Default-formatted value
    pub fn new(value: T) -> Self {
        Self {
            value,
            repr: None,
            decor: Default::default(),
        }
    }

    pub(crate) fn set_repr_unchecked(&mut self, repr: Repr) {
        self.repr = Some(repr);
    }

    /// The wrapped value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The wrapped value
    pub fn into_value(self) -> T {
        self.value
    }

    /// Returns the key raw representation.
    pub fn to_repr(&self) -> Cow<Repr> {
        self.repr
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.value.to_repr()))
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Auto formats the value.
    pub fn fmt(&mut self) {
        self.repr = Some(self.value.to_repr());
    }
}

impl<T> std::fmt::Display for Formatted<T>
where
    T: ValueRepr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, ("", ""))
    }
}

pub trait ValueRepr: crate::private::Sealed {
    /// The TOML representation of the value
    fn to_repr(&self) -> Repr;
}

/// TOML-encoded value
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
pub struct Repr {
    raw_value: InternalString,
}

impl Repr {
    pub(crate) fn new_unchecked(raw: impl Into<InternalString>) -> Self {
        Repr {
            raw_value: raw.into(),
        }
    }

    /// Access the underlying value
    pub fn as_raw(&self) -> &str {
        &self.raw_value
    }
}

impl std::fmt::Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_raw().fmt(f)
    }
}

/// A prefix and suffix,
///
/// Including comments, whitespaces and newlines.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Default, Debug, Hash)]
pub struct Decor {
    prefix: Option<InternalString>,
    suffix: Option<InternalString>,
}

impl Decor {
    /// Creates a new decor from the given prefix and suffix.
    pub fn new(prefix: impl Into<InternalString>, suffix: impl Into<InternalString>) -> Self {
        Self {
            prefix: Some(prefix.into()),
            suffix: Some(suffix.into()),
        }
    }

    /// Go back to default decor
    pub fn clear(&mut self) {
        self.prefix = None;
        self.suffix = None;
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Set the prefix.
    pub fn set_prefix(&mut self, prefix: impl Into<InternalString>) {
        self.prefix = Some(prefix.into());
    }

    /// Get the suffix.
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    /// Set the suffix.
    pub fn set_suffix(&mut self, suffix: impl Into<InternalString>) {
        self.suffix = Some(suffix.into());
    }
}
