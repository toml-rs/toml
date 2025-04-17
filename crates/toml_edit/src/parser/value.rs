use crate::parser::array::on_array;
use crate::parser::inline_table::on_inline_table;
use crate::parser::prelude::*;
use crate::repr::{Formatted, Repr};
use crate::RawString;
use crate::Value;

/// ```bnf
/// val = string / boolean / array / inline-table / date-time / float / integer
/// ```
pub(crate) fn value(
    input: &mut Input<'_>,
    source: toml_parse::Source<'_>,
    errors: &mut dyn ErrorSink,
) -> Value {
    #[cfg(feature = "unstable-debug")]
    let _scope = TraceScope::new("value");
    if let Some(event) = input.next_token() {
        match event.kind() {
            EventKind::StdTableOpen
            | EventKind::ArrayTableOpen
            | EventKind::InlineTableClose
            | EventKind::ArrayClose
            | EventKind::ValueSep
            | EventKind::Comment
            | EventKind::Newline
            | EventKind::Error
            | EventKind::SimpleKey
            | EventKind::KeySep
            | EventKind::KeyValSep
            | EventKind::StdTableClose
            | EventKind::ArrayTableClose => {
                #[cfg(feature = "unstable-debug")]
                trace(
                    &format!("unexpected {event:?}"),
                    anstyle::AnsiColor::Red.on_default(),
                );
            }
            EventKind::Whitespace => {
                #[cfg(feature = "unstable-debug")]
                trace(
                    &format!("unexpected {event:?}"),
                    anstyle::AnsiColor::Red.on_default(),
                );
            }
            EventKind::InlineTableOpen => {
                return Value::InlineTable(on_inline_table(event, input, source, errors));
            }
            EventKind::ArrayOpen => {
                return Value::Array(on_array(event, input, source, errors));
            }
            EventKind::Scalar => {
                return on_scalar(event, source, errors);
            }
        }
    }

    Value::from(0)
}

pub(crate) fn on_scalar(
    event: &toml_parse::parser::Event,
    source: toml_parse::Source<'_>,
    errors: &mut dyn ErrorSink,
) -> Value {
    #[cfg(feature = "unstable-debug")]
    let _scope = TraceScope::new("on_scalar");
    let value_span = event.span();
    let value_raw = RawString::with_span(value_span.start()..value_span.end());

    #[cfg(feature = "unsafe")] // SAFETY: lexing and parsing all with same source
    let raw = unsafe { source.get_unchecked(event) };
    #[cfg(not(feature = "unsafe"))]
    let raw = source.get(event).unwrap();
    let mut decoded = std::borrow::Cow::Borrowed("");
    let kind = raw.decode_scalar(&mut decoded, errors);
    match kind {
        toml_parse::decoder::ScalarKind::String => {
            let mut f = Formatted::new(decoded.into());
            f.set_repr_unchecked(Repr::new_unchecked(value_raw));
            Value::String(f)
        }
        toml_parse::decoder::ScalarKind::Boolean(value) => {
            let mut f = Formatted::new(value);
            f.set_repr_unchecked(Repr::new_unchecked(value_raw));
            Value::Boolean(f)
        }
        toml_parse::decoder::ScalarKind::DateTime => {
            let value = match decoded.parse::<toml_datetime::Datetime>() {
                Ok(value) => value,
                Err(err) => {
                    errors.report_error(
                        ParseError::new(err.to_string()).with_unexpected(event.span()),
                    );
                    toml_datetime::Datetime {
                        date: None,
                        time: None,
                        offset: None,
                    }
                }
            };
            let mut f = Formatted::new(value);
            f.set_repr_unchecked(Repr::new_unchecked(value_raw));
            Value::Datetime(f)
        }
        toml_parse::decoder::ScalarKind::Float => {
            let value = match decoded.parse::<f64>() {
                Ok(value) => {
                    if value.is_infinite()
                        && !(decoded
                            .strip_prefix(['+', '-'])
                            .unwrap_or(&decoded)
                            .chars()
                            .all(|c| c.is_ascii_alphabetic()))
                    {
                        errors.report_error(
                            ParseError::new("floating-point number overflowed")
                                .with_unexpected(event.span()),
                        );
                    }
                    value
                }
                Err(_) => {
                    errors.report_error(
                        ParseError::new(kind.invalid_description()).with_unexpected(event.span()),
                    );
                    f64::NAN
                }
            };
            let mut f = Formatted::new(value);
            f.set_repr_unchecked(Repr::new_unchecked(value_raw));
            Value::Float(f)
        }
        toml_parse::decoder::ScalarKind::Integer(radix) => {
            let value = match i64::from_str_radix(&decoded, radix.value()) {
                Ok(value) => value,
                Err(_) => {
                    let is_valid = match radix {
                        toml_parse::decoder::IntegerRadix::Bin => |c: char| c == '0' || c == '1',
                        toml_parse::decoder::IntegerRadix::Oct => {
                            winnow::stream::AsChar::is_oct_digit
                        }
                        toml_parse::decoder::IntegerRadix::Dec => {
                            winnow::stream::AsChar::is_dec_digit
                        }
                        toml_parse::decoder::IntegerRadix::Hex => {
                            winnow::stream::AsChar::is_hex_digit
                        }
                    };
                    if decoded
                        .strip_prefix(['+', '-'])
                        .unwrap_or(&decoded)
                        .chars()
                        .all(is_valid)
                    {
                        errors.report_error(
                            ParseError::new("integer number overflowed")
                                .with_unexpected(event.span()),
                        );
                    } else {
                        errors.report_error(
                            ParseError::new(radix.invalid_description())
                                .with_unexpected(event.span()),
                        );
                    }
                    i64::MAX
                }
            };
            let mut f = Formatted::new(value);
            f.set_repr_unchecked(Repr::new_unchecked(value_raw));
            Value::Integer(f)
        }
    }
}
