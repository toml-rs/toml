pub(crate) type InternalString = String;

/// A value together with its `to_string` representation,
/// including surrounding it whitespaces and comments.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Formatted<T> {
    value: T,
    pub(crate) repr: Repr,
    pub(crate) decor: Decor,
}

impl<T> Formatted<T> {
    pub fn raw(&self) -> &str {
        &self.repr.raw_value
    }

    pub fn prefix(&self) -> Option<&str> {
        self.decor.prefix()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.decor.suffix()
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub(crate) fn new(v: T, repr: Repr) -> Self {
        Self {
            value: v,
            repr,
            decor: Default::default(),
        }
    }

    pub(crate) fn set_decor(mut self, decor: Decor) -> Self {
        self.decor = decor;
        self
    }
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

/// A prefix and suffix,
///
/// Including comments, whitespaces and newlines.
#[derive(Eq, PartialEq, Clone, Default, Debug, Hash)]
pub struct Decor {
    prefix: Option<InternalString>,
    suffix: Option<InternalString>,
}

impl Decor {
    /// Creates a new decor from the given prefix and suffix.
    pub fn new(prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self {
            prefix: Some(prefix.into()),
            suffix: Some(suffix.into()),
        }
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the suffix.
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    /// Render a value with its decor
    pub(crate) fn display<'d, D: std::fmt::Display + std::fmt::Debug>(
        &'d self,
        inner: &'d D,
        default: (&'static str, &'static str),
    ) -> DecorDisplay<'d, D> {
        DecorDisplay {
            inner,
            decor: self,
            default,
        }
    }
}

/// Render a prefix and suffix,
///
/// Including comments, whitespaces and newlines.
#[derive(Debug)]
pub(crate) struct DecorDisplay<'d, D> {
    pub(crate) inner: &'d D,
    pub(crate) decor: &'d Decor,
    pub(crate) default: (&'static str, &'static str),
}
