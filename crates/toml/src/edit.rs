#[cfg(feature = "display")]
pub(crate) mod ser {
    pub(crate) use toml_edit::ser::Error;
}

#[cfg(not(feature = "display"))]
pub(crate) mod ser {
    use crate::alloc_prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[non_exhaustive]
    pub(crate) enum Error {
        UnsupportedType(Option<&'static str>),
        UnsupportedNone,
        KeyNotString,
        Custom(String),
    }

    impl Error {
        pub(crate) fn custom<T>(msg: T) -> Self
        where
            T: core::fmt::Display,
        {
            Error::Custom(msg.to_string())
        }
    }

    impl serde::ser::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: core::fmt::Display,
        {
            Self::custom(msg)
        }
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::UnsupportedType(Some(t)) => write!(formatter, "unsupported {t} type"),
                Self::UnsupportedType(None) => write!(formatter, "unsupported rust type"),
                Self::UnsupportedNone => "unsupported None value".fmt(formatter),
                Self::KeyNotString => "map key was not a string".fmt(formatter),
                Self::Custom(s) => s.fmt(formatter),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for Error {}
    #[cfg(not(feature = "std"))]
    impl serde::de::StdError for Error {}
}
