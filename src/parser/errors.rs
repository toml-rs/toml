use crate::Key;
use combine::easy::Errors as ParseError;
use combine::stream::easy::Error;
use combine::stream::position::SourcePosition;
use itertools::Itertools;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

/// Type representing a TOML parse error
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TomlError {
    message: String,
    line_col: Option<(usize, usize)>,
}

impl TomlError {
    pub(crate) fn new(error: ParseError<u8, &[u8], usize>, input: &[u8]) -> Self {
        let fancy = FancyError::new(error, input);
        let message = fancy.to_string();
        let line_col = Some((fancy.position.line as usize, fancy.position.column as usize));
        Self { message, line_col }
    }

    pub(crate) fn from_unparsed(pos: usize, input: &[u8]) -> Self {
        Self::new(
            ParseError::new(pos, CustomError::UnparsedLine.into()),
            input,
        )
    }

    pub(crate) fn custom(message: String) -> Self {
        Self {
            message,
            line_col: None,
        }
    }

    /// Produces a (line, column) pair of the position of the error if available
    ///
    /// All indexes are 0-based.
    pub fn line_col(&self) -> Option<(usize, usize)> {
        self.line_col
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
    errors: Vec<Error<char, String>>,
    position: SourcePosition,
    input: &'a [u8],
}

impl<'a> FancyError<'a> {
    pub(crate) fn new(error: ParseError<u8, &'a [u8], usize>, input: &'a [u8]) -> Self {
        let position = translate_position(input, error.position);
        let errors: Vec<_> = error
            .errors
            .into_iter()
            .map(|e| {
                e.map_token(char::from)
                    .map_range(|s| String::from_utf8_lossy(s).into_owned())
            })
            .collect();
        Self {
            errors,
            position,
            input,
        }
    }
}

impl<'a> Display for FancyError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let SourcePosition { line, column } = self.position;

        let line_num = line + 1;
        let col_num = column + 1;
        let offset = line_num.to_string().len();
        let content = self
            .input
            .split(|b| *b == b'\n')
            .nth((line) as usize)
            .expect("valid line number");
        let content = String::from_utf8_lossy(content);

        writeln!(
            f,
            "TOML parse error at line {}, column {}",
            line_num, col_num
        )?;

        //   |
        for _ in 0..=offset {
            write!(f, " ")?;
        }
        writeln!(f, "|")?;

        // 1 | 00:32:00.a999999
        write!(f, "{} | ", line_num)?;
        writeln!(f, "{}", content)?;

        //   |          ^
        for _ in 0..=offset {
            write!(f, " ")?;
        }
        write!(f, "|")?;
        for _ in 0..=column {
            write!(f, " ")?;
        }
        writeln!(f, "^")?;

        Error::fmt_errors(self.errors.as_ref(), f)
    }
}

fn translate_position(input: &[u8], index: usize) -> SourcePosition {
    if input.is_empty() {
        return SourcePosition {
            line: 0,
            column: index as i32,
        };
    }

    let safe_index = index.min(input.len() - 1);
    let column_offset = index - safe_index;
    let index = safe_index;

    let nl = input[0..index]
        .iter()
        .rev()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .map(|(nl, _)| index - nl - 1);
    let line_start = match nl {
        Some(nl) => nl + 1,
        None => 0,
    };
    let line = input[0..line_start].iter().filter(|b| **b == b'\n').count();
    let line = line as i32;

    let column = std::str::from_utf8(&input[line_start..=index])
        .map(|s| s.chars().count() - 1)
        .unwrap_or_else(|_| index - line_start);
    let column = (column + column_offset) as i32;

    SourcePosition { line, column }
}

#[cfg(test)]
mod test_translate_position {
    use super::*;

    #[test]
    fn empty() {
        let input = b"";
        let index = 0;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 0, column: 0 });
    }

    #[test]
    fn start() {
        let input = b"Hello";
        let index = 0;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 0, column: 0 });
    }

    #[test]
    fn end() {
        let input = b"Hello";
        let index = input.len() - 1;
        let position = translate_position(&input[..], index);
        assert_eq!(
            position,
            SourcePosition {
                line: 0,
                column: input.len() as i32 - 1
            }
        );
    }

    #[test]
    fn after() {
        let input = b"Hello";
        let index = input.len();
        let position = translate_position(&input[..], index);
        assert_eq!(
            position,
            SourcePosition {
                line: 0,
                column: input.len() as i32
            }
        );
    }

    #[test]
    fn first_line() {
        let input = b"Hello\nWorld\n";
        let index = 2;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 0, column: 2 });
    }

    #[test]
    fn end_of_line() {
        let input = b"Hello\nWorld\n";
        let index = 5;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 0, column: 5 });
    }

    #[test]
    fn start_of_second_line() {
        let input = b"Hello\nWorld\n";
        let index = 6;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 1, column: 0 });
    }

    #[test]
    fn second_line() {
        let input = b"Hello\nWorld\n";
        let index = 8;
        let position = translate_position(&input[..], index);
        assert_eq!(position, SourcePosition { line: 1, column: 2 });
    }
}

#[derive(Debug, Clone)]
pub(crate) enum CustomError {
    DuplicateKey {
        key: String,
        table: Option<Vec<Key>>,
    },
    DottedKeyExtendWrongType {
        key: Vec<Key>,
        actual: &'static str,
    },
    InvalidHexEscape(u32),
    UnparsedLine,
    OutOfRange,
}

impl StdError for CustomError {
    fn description(&self) -> &'static str {
        "TOML parse error"
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CustomError::DuplicateKey { key, table } => {
                if let Some(table) = table {
                    if table.is_empty() {
                        writeln!(f, "Duplicate key `{}` in document root", key)
                    } else {
                        let path = table.iter().join(".");
                        writeln!(f, "Duplicate key `{}` in table `{}`", key, path)
                    }
                } else {
                    writeln!(f, "Duplicate key `{}`", key)
                }
            }
            CustomError::DottedKeyExtendWrongType { key, actual } => {
                let path = key.iter().join(".");
                writeln!(
                    f,
                    "Dotted key `{}` attempted to extend non-table type ({})",
                    path, actual
                )
            }
            CustomError::InvalidHexEscape(h) => {
                writeln!(f, "Invalid hex escape code: {:x} ", h)
            }
            CustomError::UnparsedLine => writeln!(f, "Could not parse the line"),
            CustomError::OutOfRange => writeln!(f, "Value is out of range"),
        }
    }
}
