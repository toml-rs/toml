use combine::easy::Errors as ParseError;
use combine::stream::easy::Error;
use combine::stream::position::SourcePosition;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

/// Type representing a TOML parse error
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TomlError {
    message: String,
}

impl TomlError {
    pub(crate) fn new(error: ParseError<char, &str, SourcePosition>, input: &str) -> Self {
        Self {
            message: format!("{}", FancyError::new(error, input)),
        }
    }

    pub(crate) fn from_unparsed(pos: SourcePosition, input: &str) -> Self {
        Self::new(
            ParseError::new(pos, CustomError::UnparsedLine.into()),
            input,
        )
    }
}

/// Displays a TOML parse error
///
/// # Example
///
/// TOML parse error at line 1, column 10
///   |
/// 1 | 00:32:00.a999999
///   |          ^
/// Unexpected `a`
/// Expected `digit`
/// While parsing a Time
/// While parsing a Date-Time
impl Display for TomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for TomlError {
    fn description(&self) -> &'static str {
        "TOML parse error"
    }
}

#[derive(Debug)]
pub(crate) struct FancyError<'a> {
    error: ParseError<char, &'a str, SourcePosition>,
    input: &'a str,
}

impl<'a> FancyError<'a> {
    pub(crate) fn new(error: ParseError<char, &'a str, SourcePosition>, input: &'a str) -> Self {
        Self { error, input }
    }
}

impl<'a> Display for FancyError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let SourcePosition { line, column } = self.error.position;

        let offset = line.to_string().len();
        let content = self
            .input
            .split('\n')
            .nth((line - 1) as usize)
            .expect("line");

        writeln!(f, "TOML parse error at line {}, column {}", line, column)?;

        //   |
        for _ in 0..=offset {
            write!(f, " ")?;
        }
        writeln!(f, "|")?;

        // 1 | 00:32:00.a999999
        write!(f, "{} | ", line)?;
        writeln!(f, "{}", content)?;

        //   |          ^
        for _ in 0..=offset {
            write!(f, " ")?;
        }
        write!(f, "|")?;
        for _ in 0..column {
            write!(f, " ")?;
        }
        writeln!(f, "^")?;

        Error::fmt_errors(self.error.errors.as_ref(), f)
    }
}

#[derive(Debug, Clone)]
pub enum CustomError {
    MixedArrayType { got: String, expected: String },
    DuplicateKey { key: String, table: String },
    InvalidHexEscape(u32),
    UnparsedLine,
}

impl StdError for CustomError {
    fn description(&self) -> &'static str {
        "TOML parse error"
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            CustomError::MixedArrayType {
                ref got,
                ref expected,
            } => {
                writeln!(f, "Mixed types in array: {} and {}", expected, got)
            }
            CustomError::DuplicateKey { ref key, ref table } => {
                writeln!(f, "Duplicate key `{}` in `{}` table", key, table)
            }
            CustomError::InvalidHexEscape(ref h) => {
                writeln!(f, "Invalid hex escape code: {:x} ", h)
            }
            CustomError::UnparsedLine => writeln!(f, "Could not parse the line"),
        }
    }
}
