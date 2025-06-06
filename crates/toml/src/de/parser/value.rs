use serde_spanned::Spanned;

use crate::de::parser::array::on_array;
use crate::de::parser::inline_table::on_inline_table;
use crate::de::parser::prelude::*;
use crate::de::DeValue;

/// ```bnf
/// val = string / boolean / array / inline-table / date-time / float / integer
/// ```
pub(crate) fn value<'i>(
    input: &mut Input<'_>,
    source: toml_parse::Source<'i>,
    errors: &mut dyn ErrorSink,
) -> Spanned<DeValue<'i>> {
    #[cfg(feature = "debug")]
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
                #[cfg(feature = "debug")]
                trace(
                    &format!("unexpected {event:?}"),
                    anstyle::AnsiColor::Red.on_default(),
                );
            }
            EventKind::Whitespace => {
                #[cfg(feature = "debug")]
                trace(
                    &format!("unexpected {event:?}"),
                    anstyle::AnsiColor::Red.on_default(),
                );
            }
            EventKind::InlineTableOpen => {
                return on_inline_table(event, input, source, errors);
            }
            EventKind::ArrayOpen => {
                return on_array(event, input, source, errors);
            }
            EventKind::Scalar => {
                return on_scalar(event, source, errors);
            }
        }
    }

    Spanned::new(0..0, DeValue::Integer(0))
}

pub(crate) fn on_scalar<'i>(
    event: &toml_parse::parser::Event,
    source: toml_parse::Source<'i>,
    errors: &mut dyn ErrorSink,
) -> Spanned<DeValue<'i>> {
    #[cfg(feature = "debug")]
    let _scope = TraceScope::new("on_scalar");
    let value_span = event.span();
    let value_span = value_span.start()..value_span.end();

    #[cfg(feature = "unsafe")] // SAFETY: lexing and parsing all with same source
    let raw = unsafe { source.get_unchecked(event) };
    #[cfg(not(feature = "unsafe"))]
    let raw = source.get(event).unwrap();
    let mut decoded = std::borrow::Cow::Borrowed("");
    let kind = raw.decode_scalar(&mut decoded, errors);
    match kind {
        toml_parse::decoder::ScalarKind::String => {
            Spanned::new(value_span, DeValue::String(decoded))
        }
        toml_parse::decoder::ScalarKind::Boolean(value) => {
            Spanned::new(value_span, DeValue::Boolean(value))
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
            Spanned::new(value_span, DeValue::Datetime(value))
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
            Spanned::new(value_span, DeValue::Float(value))
        }
        toml_parse::decoder::ScalarKind::Integer(radix) => {
            let value = match i64::from_str_radix(&decoded, radix.value()) {
                Ok(value) => value,
                Err(_) => {
                    // Assuming the decoder fully validated it, leaving only overflow errors
                    errors.report_error(
                        ParseError::new("integer number overflowed").with_unexpected(event.span()),
                    );
                    i64::MAX
                }
            };
            Spanned::new(value_span, DeValue::Integer(value))
        }
    }
}
