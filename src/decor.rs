/// A value together with its `to_string` representation,
/// including surrounding it whitespaces and comments.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Formatted<T> {
    value: T,
    pub(crate) repr: Repr,
}

// String representation of a key or a value
// together with a decoration.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub(crate) struct Repr {
    pub decor: Decor,
    pub raw_value: InternalString,
}

/// A prefix and suffix,
/// including comments, whitespaces and newlines.
#[derive(Eq, PartialEq, Clone, Default, Debug, Hash)]
pub struct Decor {
    pub(crate) prefix: InternalString,
    pub(crate) suffix: InternalString,
}

pub(crate) type InternalString = String;

impl Decor {
    /// Creates a new decor from the given prefix and suffix.
    pub fn new(prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
        }
    }

    /// Get the prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the suffix.
    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}

impl Repr {
    pub fn new(
        prefix: impl Into<InternalString>,
        value: impl Into<InternalString>,
        suffix: impl Into<InternalString>,
    ) -> Self {
        Repr {
            decor: Decor::new(prefix, suffix),
            raw_value: value.into(),
        }
    }
}

impl<D: std::fmt::Display> From<&D> for Repr {
    fn from(other: &D) -> Self {
        Self::new("", other.to_string(), "")
    }
}

impl<T> Formatted<T> {
    pub fn raw(&self) -> &str {
        &self.repr.raw_value
    }

    pub fn prefix(&self) -> &str {
        &self.repr.decor.prefix
    }

    pub fn suffix(&self) -> &str {
        &self.repr.decor.suffix
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub(crate) fn new(v: T, repr: Repr) -> Self {
        Self { value: v, repr }
    }
}

impl<D: std::fmt::Display> From<D> for Formatted<D> {
    fn from(other: D) -> Self {
        let repr = Repr::from(&other);
        Self { value: other, repr }
    }
}
