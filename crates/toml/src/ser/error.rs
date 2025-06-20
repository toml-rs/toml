use crate::alloc_prelude::*;

/// Errors that can occur when serializing a type.
#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) inner: crate::edit::ser::Error,
}

impl Error {
    pub(crate) fn new(inner: impl core::fmt::Display) -> Self {
        Self {
            inner: crate::edit::ser::Error::Custom(inner.to_string()),
        }
    }

    #[cfg(feature = "display")]
    pub(crate) fn wrap(inner: crate::edit::ser::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedType(t),
        }
    }

    pub(crate) fn unsupported_none() -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedNone,
        }
    }

    pub(crate) fn key_not_string() -> Self {
        Self {
            inner: crate::edit::ser::Error::KeyNotString,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::new(msg)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
#[cfg(not(feature = "std"))]
impl serde::de::StdError for Error {}
