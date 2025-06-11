/// Errors that can occur when deserializing a type.
#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    inner: crate::edit::de::Error,
}

impl Error {
    pub(crate) fn new(inner: crate::edit::de::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn add_key(&mut self, key: String) {
        self.inner.add_key(key);
    }

    /// What went wrong
    pub fn message(&self) -> &str {
        self.inner.message()
    }

    /// The start/end index into the original document where the error occurred
    pub fn span(&self) -> Option<std::ops::Range<usize>> {
        self.inner.span()
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(crate::edit::de::Error::custom(msg))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}
