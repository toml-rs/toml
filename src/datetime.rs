use std::str::FromStr;

use crate::parser;

/// A RFC-3339 formatted TOML date/time w/ timezone
///
/// # Examples
///
/// ```rust
/// let raw = "1996-12-19T16:39:57-08:00";
/// let toml: toml_edit::OffsetDateTime = raw.parse().unwrap();
/// let backagain = toml.to_string();
/// assert_eq!(raw, backagain);
/// ```
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct OffsetDateTime {
    pub(crate) inner: chrono::DateTime<chrono::FixedOffset>,
}

impl FromStr for OffsetDateTime {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|e| parser::TomlError::custom(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl std::fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = self.inner.to_rfc3339();
        s.fmt(f)
    }
}

/// A RFC-3339 formatted TOML date/time w/o timezone
///
/// # Examples
///
/// ```rust
/// let raw = "1996-12-19T16:39:57";
/// let toml: toml_edit::LocalDateTime = raw.parse().unwrap();
/// let backagain = toml.to_string();
/// assert_eq!(raw, backagain);
/// ```
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct LocalDateTime {
    pub(crate) inner: chrono::NaiveDateTime,
}

impl FromStr for LocalDateTime {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
            .map_err(|e| parser::TomlError::custom(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = self.inner.format("%Y-%m-%dT%H:%M:%S%.f");
        s.fmt(f)
    }
}

/// A RFC-3339 formatted TOML date
///
/// # Examples
///
/// ```rust
/// let raw = "1996-12-19";
/// let toml: toml_edit::LocalDate = raw.parse().unwrap();
/// let backagain = toml.to_string();
/// assert_eq!(raw, backagain);
/// ```
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct LocalDate {
    pub(crate) inner: chrono::NaiveDate,
}

impl FromStr for LocalDate {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .parse::<chrono::NaiveDate>()
            .map_err(|e| parser::TomlError::custom(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl std::fmt::Display for LocalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.inner.fmt(f)
    }
}

/// A RFC-3339 formatted TOML time
///
/// # Examples
///
/// ```rust
/// let raw = "23:56:04.000123456";
/// let toml: toml_edit::LocalTime = raw.parse().unwrap();
/// let backagain = toml.to_string();
/// assert_eq!(raw, backagain);
/// ```
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct LocalTime {
    pub(crate) inner: chrono::NaiveTime,
}

impl FromStr for LocalTime {
    type Err = parser::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .parse::<chrono::NaiveTime>()
            .map_err(|e| parser::TomlError::custom(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl std::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.inner.fmt(f)
    }
}
