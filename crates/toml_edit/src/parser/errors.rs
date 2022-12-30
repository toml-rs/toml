use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

use itertools::Itertools;

use crate::parser::prelude::*;
use crate::Key;

/// Type representing a TOML parse error
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TomlError {
    message: String,
    line_col: Option<(usize, usize)>,
}

impl TomlError {
    pub(crate) fn new(error: ParserError<'_>, original: Input<'_>) -> Self {
        use nom8::input::Offset;
        let offset = original.offset(error.input);
        let position = translate_position(original, offset);
        let message = ParserErrorDisplay {
            error: &error,
            original,
            position,
        }
        .to_string();
        let line_col = Some(position);
        Self { message, line_col }
    }

    #[cfg(feature = "serde")]
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

#[derive(Debug, PartialEq)]
pub(crate) struct ParserError<'b> {
    input: Input<'b>,
    context: Vec<Details>,
}

impl<'b> nom8::error::ParseError<Input<'b>> for ParserError<'b> {
    fn from_error_kind(input: Input<'b>, _kind: nom8::error::ErrorKind) -> Self {
        Self {
            input,
            context: Default::default(),
        }
    }

    fn append(_input: Input<'b>, _kind: nom8::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(_input: Input<'b>, _: char) -> Self {
        unimplemented!("this shouldn't be called with a binary parser")
    }

    fn or(self, other: Self) -> Self {
        other
    }
}

impl<'b> nom8::error::ContextError<Input<'b>, Context> for ParserError<'b> {
    fn add_context(_input: Input<'b>, ctx: Context, mut other: Self) -> Self {
        other.context.push(Details::Context(ctx));
        other
    }
}

impl<'b, E: std::error::Error + Send + Sync + 'static> nom8::error::FromExternalError<Input<'b>, E>
    for ParserError<'b>
{
    fn from_external_error(input: Input<'b>, _kind: nom8::error::ErrorKind, e: E) -> Self {
        Self {
            input,
            context: vec![Details::Cause(Box::new(e))],
        }
    }
}

struct ParserErrorDisplay<'a> {
    error: &'a ParserError<'a>,
    original: Input<'a>,
    position: (usize, usize),
}

impl<'a> std::fmt::Display for ParserErrorDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, column) = self.position;
        let line_num = line + 1;
        let col_num = column + 1;
        let gutter = line_num.to_string().len();
        let content = self
            .original
            .split(|b| *b == b'\n')
            .nth(line)
            .expect("valid line number");
        let content = String::from_utf8_lossy(content);

        let cause = self.error.context.iter().find_map(|c| match c {
            Details::Cause(c) => Some(c),
            _ => None,
        });
        let expression = self.error.context.iter().find_map(|c| match c {
            Details::Context(Context::Expression(c)) => Some(c),
            _ => None,
        });
        let expected = self
            .error
            .context
            .iter()
            .filter_map(|c| match c {
                Details::Context(Context::Expected(c)) => Some(c),
                _ => None,
            })
            .collect::<Vec<_>>();

        writeln!(
            f,
            "TOML parse error at line {}, column {}",
            line_num, col_num
        )?;
        //   |
        for _ in 0..=gutter {
            write!(f, " ")?;
        }
        writeln!(f, "|")?;

        // 1 | 00:32:00.a999999
        write!(f, "{} | ", line_num)?;
        writeln!(f, "{}", content)?;

        //   |          ^
        for _ in 0..=gutter {
            write!(f, " ")?;
        }
        write!(f, "|")?;
        for _ in 0..=column {
            write!(f, " ")?;
        }
        writeln!(f, "^")?;

        if let Some(expression) = expression {
            writeln!(f, "Invalid {}", expression)?;
        }

        if !expected.is_empty() {
            write!(f, "Expected ")?;
            for (i, expected) in expected.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", expected)?;
            }
            writeln!(f)?;
        }
        if let Some(cause) = &cause {
            write!(f, "{}", cause)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Details {
    Context(Context),
    Cause(Box<dyn std::error::Error + Send + Sync + 'static>),
}

// For tests
impl std::cmp::PartialEq for Details {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Context(lhs), Self::Context(rhs)) => lhs == rhs,
            (Self::Cause(lhs), Self::Cause(rhs)) => lhs.to_string() == rhs.to_string(),
            (_, _) => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Context {
    Expression(&'static str),
    Expected(ParserValue),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ParserValue {
    CharLiteral(char),
    StringLiteral(&'static str),
    Description(&'static str),
}

impl std::fmt::Display for ParserValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserValue::CharLiteral('\n') => "newline".fmt(f),
            ParserValue::CharLiteral('`') => "'`'".fmt(f),
            ParserValue::CharLiteral(c) if c.is_ascii_control() => {
                write!(f, "`{}`", c.escape_debug())
            }
            ParserValue::CharLiteral(c) => write!(f, "`{}`", c),
            ParserValue::StringLiteral(c) => write!(f, "`{}`", c),
            ParserValue::Description(c) => write!(f, "{}", c),
        }
    }
}

fn translate_position(input: &[u8], index: usize) -> (usize, usize) {
    if input.is_empty() {
        return (0, index);
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
    let line = line;

    let column = std::str::from_utf8(&input[line_start..=index])
        .map(|s| s.chars().count() - 1)
        .unwrap_or_else(|_| index - line_start);
    let column = column + column_offset;

    (line, column)
}

#[cfg(test)]
mod test_translate_position {
    use super::*;

    #[test]
    fn empty() {
        let input = b"";
        let index = 0;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, 0));
    }

    #[test]
    fn start() {
        let input = b"Hello";
        let index = 0;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, 0));
    }

    #[test]
    fn end() {
        let input = b"Hello";
        let index = input.len() - 1;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, input.len() - 1));
    }

    #[test]
    fn after() {
        let input = b"Hello";
        let index = input.len();
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, input.len()));
    }

    #[test]
    fn first_line() {
        let input = b"Hello\nWorld\n";
        let index = 2;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, 2));
    }

    #[test]
    fn end_of_line() {
        let input = b"Hello\nWorld\n";
        let index = 5;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (0, 5));
    }

    #[test]
    fn start_of_second_line() {
        let input = b"Hello\nWorld\n";
        let index = 6;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (1, 0));
    }

    #[test]
    fn second_line() {
        let input = b"Hello\nWorld\n";
        let index = 8;
        let position = translate_position(&input[..], index);
        assert_eq!(position, (1, 2));
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
    OutOfRange,
    #[cfg_attr(feature = "unbounded", allow(dead_code))]
    RecursionLimitExceeded,
}

impl CustomError {
    pub(crate) fn duplicate_key(path: &[Key], i: usize) -> Self {
        assert!(i < path.len());
        Self::DuplicateKey {
            key: path[i].to_repr().as_ref().as_raw().into(),
            table: Some(path[..i].to_vec()),
        }
    }

    pub(crate) fn extend_wrong_type(path: &[Key], i: usize, actual: &'static str) -> Self {
        assert!(i < path.len());
        Self::DottedKeyExtendWrongType {
            key: path[..=i].to_vec(),
            actual,
        }
    }
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
            CustomError::OutOfRange => writeln!(f, "Value is out of range"),
            CustomError::RecursionLimitExceeded => writeln!(f, "Recursion limit exceded"),
        }
    }
}
