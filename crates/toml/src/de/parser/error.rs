use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

/// A TOML parse error
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct TomlError {
    message: String,
    raw: Option<std::sync::Arc<str>>,
    keys: Vec<String>,
    span: Option<std::ops::Range<usize>>,
}

impl TomlError {
    pub(crate) fn new(raw: std::sync::Arc<str>, error: toml_parse::ParseError) -> Self {
        let mut message = String::new();
        message.push_str(error.description());
        if let Some(expected) = error.expected() {
            message.push_str(", expected ");
            if expected.is_empty() {
                message.push_str("nothing");
            } else {
                for (i, expected) in expected.iter().enumerate() {
                    if i != 0 {
                        message.push_str(", ");
                    }
                    match expected {
                        toml_parse::Expected::Literal(desc) => {
                            message.push_str(&render_literal(desc));
                        }
                        toml_parse::Expected::Description(desc) => message.push_str(desc),
                        _ => message.push_str("etc"),
                    }
                }
            }
        }

        let span = error.unexpected().map(|span| span.start()..span.end());

        Self {
            message,
            raw: Some(raw),
            keys: Vec::new(),
            span,
        }
    }

    /// The start/end index into the original document where the error occurred
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }
}

fn render_literal(literal: &str) -> String {
    match literal {
        "\n" => "newline".to_owned(),
        "`" => "'`'".to_owned(),
        s if s.chars().all(|c| c.is_ascii_control()) => {
            format!("`{}`", s.escape_debug())
        }
        s => format!("`{s}`"),
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
        let mut context = false;
        if let (Some(raw), Some(span)) = (&self.raw, self.span()) {
            context = true;

            let (line, column) = translate_position(raw.as_bytes(), span.start);
            let line_num = line + 1;
            let col_num = column + 1;
            let gutter = line_num.to_string().len();
            let content = raw.split('\n').nth(line).expect("valid line number");
            let highlight_len = span.end - span.start;
            // Allow highlight to go one past the line
            let highlight_len = highlight_len.min(content.len().saturating_sub(column));

            writeln!(f, "TOML parse error at line {line_num}, column {col_num}")?;
            //   |
            for _ in 0..=gutter {
                write!(f, " ")?;
            }
            writeln!(f, "|")?;

            // 1 | 00:32:00.a999999
            write!(f, "{line_num} | ")?;
            writeln!(f, "{content}")?;

            //   |          ^
            for _ in 0..=gutter {
                write!(f, " ")?;
            }
            write!(f, "|")?;
            for _ in 0..=column {
                write!(f, " ")?;
            }
            // The span will be empty at eof, so we need to make sure we always print at least
            // one `^`
            write!(f, "^")?;
            for _ in 1..highlight_len {
                write!(f, "^")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", self.message)?;
        if !context && !self.keys.is_empty() {
            writeln!(f, "in `{}`", self.keys.join("."))?;
        }

        Ok(())
    }
}

impl StdError for TomlError {
    fn description(&self) -> &'static str {
        "TOML parse error"
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

    let column = std::str::from_utf8(&input[line_start..=index])
        .map(|s| s.chars().count() - 1)
        .unwrap_or_else(|_| index - line_start);
    let column = column + column_offset;

    (line, column)
}

pub(crate) struct TomlSink<'i, S> {
    source: toml_parse::Source<'i>,
    raw: Option<std::sync::Arc<str>>,
    sink: S,
}

impl<'i, S: Default> TomlSink<'i, S> {
    pub(crate) fn new(source: toml_parse::Source<'i>) -> Self {
        Self {
            source,
            raw: None,
            sink: Default::default(),
        }
    }

    pub(crate) fn into_inner(self) -> S {
        self.sink
    }
}

impl<'i> toml_parse::ErrorSink for TomlSink<'i, Option<TomlError>> {
    fn report_error(&mut self, error: toml_parse::ParseError) {
        if self.sink.is_none() {
            let raw = self
                .raw
                .get_or_insert_with(|| std::sync::Arc::from(self.source.input()));
            let error = TomlError::new(raw.clone(), error);
            self.sink = Some(error);
        }
    }
}

impl<'i> toml_parse::ErrorSink for TomlSink<'i, Vec<TomlError>> {
    fn report_error(&mut self, error: toml_parse::ParseError) {
        let raw = self
            .raw
            .get_or_insert_with(|| std::sync::Arc::from(self.source.input()));
        let error = TomlError::new(raw.clone(), error);
        self.sink.push(error);
    }
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
