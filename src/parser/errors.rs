use std::fmt::{Display, Formatter, Result};
use std::error::Error as StdError;
use combine::ParseError;
use combine::state::SourcePosition;
use combine::primitives::Error;


/// Type representing a TOML parse error
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TomlError {
    message: String,
}

impl TomlError {
    pub(crate) fn new(error: ParseError<SourcePosition, char, &str>, input: &str) -> Self {
        Self {
            message: format!("{}", FancyError::new(error, input)),
        }
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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
    error: ParseError<SourcePosition, char, &'a str>,
    input: &'a str,
}

impl<'a> FancyError<'a> {
    pub(crate) fn new(error: ParseError<SourcePosition, char, &'a str>, input: &'a str) -> Self {
        Self {
            error: error,
            input: input,
        }
    }
}

impl<'a> Display for FancyError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let SourcePosition { line, column } = self.error.position;

        let offset = line.to_string().len();
        let content = self.input
            .split('\n')
            .nth((line - 1) as usize)
            .expect("line");

        writeln!(f, "TOML parse error at line {}, column {}", line, column)?;

        //   |
        for _ in 0..(offset + 1) {
            write!(f, " ")?;
        }
        writeln!(f, "|")?;

        // 1 | 00:32:00.a999999
        write!(f, "{} | ", line)?;
        writeln!(f, "{}", content)?;

        //   |          ^
        for _ in 0..(offset + 1) {
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
pub(crate) enum CustomError {
    MixedArrayType { got: String, expected: String },
    DuplicateKey { key: String, table: String },
    InvalidHexEscape(u32),
}

impl StdError for CustomError {
    fn description(&self) -> &'static str {
        "TOML parse error"
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            CustomError::MixedArrayType {
                ref got,
                ref expected,
            } => writeln!(f, "Mixed types in array: {} and {}", expected, got),
            CustomError::DuplicateKey { ref key, ref table } => {
                writeln!(f, "Duplicate key `{}` in `{}` table", key, table)
            }
            CustomError::InvalidHexEscape(ref h) => {
                writeln!(f, "Invalid hex escape code: {:x} ", h)
            }
        }
    }
}
