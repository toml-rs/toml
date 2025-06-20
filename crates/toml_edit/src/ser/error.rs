/// Errors that can occur when deserializing a type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// Type could not be serialized to TOML
    UnsupportedType(Option<&'static str>),
    /// Value was out of range for the given type
    OutOfRange(Option<&'static str>),
    /// `None` could not be serialized to TOML
    UnsupportedNone,
    /// Key was not convertible to `String` for serializing to TOML
    KeyNotString,
    /// A serialized date was invalid
    DateInvalid,
    /// Other serialization error
    Custom(String),
}

impl Error {
    pub(crate) fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }

    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Error::UnsupportedType(t)
    }

    pub(crate) fn out_of_range(t: Option<&'static str>) -> Self {
        Error::OutOfRange(t)
    }

    pub(crate) fn unsupported_none() -> Self {
        Error::UnsupportedNone
    }

    pub(crate) fn key_not_string() -> Self {
        Error::KeyNotString
    }

    pub(crate) fn date_invalid() -> Self {
        Error::DateInvalid
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedType(Some(t)) => write!(formatter, "unsupported {t} type"),
            Self::UnsupportedType(None) => write!(formatter, "unsupported rust type"),
            Self::OutOfRange(Some(t)) => write!(formatter, "out-of-range value for {t} type"),
            Self::OutOfRange(None) => write!(formatter, "out-of-range value"),
            Self::UnsupportedNone => "unsupported None value".fmt(formatter),
            Self::KeyNotString => "map key was not a string".fmt(formatter),
            Self::DateInvalid => "a serialized date was invalid".fmt(formatter),
            Self::Custom(s) => s.fmt(formatter),
        }
    }
}

impl From<crate::TomlError> for Error {
    fn from(e: crate::TomlError) -> Error {
        Self::custom(e)
    }
}

impl From<Error> for crate::TomlError {
    fn from(e: Error) -> crate::TomlError {
        Self::custom(e.to_string(), None)
    }
}

impl std::error::Error for Error {}
