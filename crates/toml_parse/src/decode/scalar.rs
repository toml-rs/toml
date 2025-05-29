use winnow::stream::ContainsToken as _;
use winnow::stream::FindSlice as _;
use winnow::stream::Offset as _;
use winnow::stream::Stream as _;

use crate::decode::StringBuilder;
use crate::ErrorSink;
use crate::Expected;
use crate::ParseError;
use crate::Raw;
use crate::Span;

const ALLOCATION_ERROR: &str = "could not allocate for string";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ScalarKind {
    String,
    Boolean(bool),
    DateTime,
    Float,
    Integer(IntegerRadix),
}

impl ScalarKind {
    pub fn description(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Boolean(_) => "boolean",
            Self::DateTime => "date-time",
            Self::Float => "float",
            Self::Integer(radix) => radix.description(),
        }
    }

    fn invalid_description(&self) -> &'static str {
        match self {
            Self::String => "invalid string",
            Self::Boolean(_) => "invalid boolean",
            Self::DateTime => "invalid date-time",
            Self::Float => "invalid float",
            Self::Integer(radix) => radix.invalid_description(),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum IntegerRadix {
    #[default]
    Dec,
    Hex,
    Oct,
    Bin,
}

impl IntegerRadix {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Dec => "integer",
            Self::Hex => "hexadecimal",
            Self::Oct => "octal",
            Self::Bin => "binary",
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Dec => 10,
            Self::Hex => 16,
            Self::Oct => 8,
            Self::Bin => 2,
        }
    }

    fn invalid_description(&self) -> &'static str {
        match self {
            Self::Dec => "invalid integer number",
            Self::Hex => "invalid hexadecimal number",
            Self::Oct => "invalid octal number",
            Self::Bin => "invalid binary number",
        }
    }
}

pub(crate) fn decode_unquoted_scalar<'i>(
    raw: Raw<'i>,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    let s = raw.as_str();
    let Some(first) = s.as_bytes().first() else {
        return decode_invalid(raw, output, error);
    };
    match first {
        // number starts
        b'+' | b'-' |
        // Report as if they were numbers because its most likely a typo
        b'_' =>  {
            #[cfg(feature = "unsafe")] // SAFETY: ascii digit ensures UTF-8 boundary
            let rest = unsafe { raw.as_str().get_unchecked(1..) };
            #[cfg(not(feature = "unsafe"))]
            let rest = &raw.as_str()[1..];

            if rest == "nan" || rest == "inf"{
                let kind = ScalarKind::Float;
                decode_as_is(raw, kind, output, error)
            } else if is_float(raw.as_str()) {
                let kind = ScalarKind::Float;
                decode_float_or_integer(raw.as_str(), raw, kind, output, error)
            } else {
                let kind = ScalarKind::Integer(IntegerRadix::Dec);
                decode_float_or_integer(raw.as_str(), raw, kind, output, error)
            }
        }
        // Date/number starts
        b'0' => {
            if s.len() == 1 {
                let kind = ScalarKind::Integer(IntegerRadix::Dec);
                decode_float_or_integer(raw.as_str(), raw, kind, output, error)
            } else {
                match s.as_bytes()[1] {
                    b'x' | b'X' =>  {
                        let kind = ScalarKind::Integer(IntegerRadix::Hex);
                        let stream = &raw.as_str()[2..];
                        ensure_no_sign(stream, raw, error);
                        decode_float_or_integer(stream, raw, kind, output, error)
                    },
                    b'o' | b'O' =>  {
                        let kind = ScalarKind::Integer(IntegerRadix::Oct);
                        let stream = &raw.as_str()[2..];
                        ensure_no_sign(stream, raw, error);
                        decode_float_or_integer(stream, raw, kind, output, error)
                    },
                    b'b' | b'B' =>  {
                        let kind = ScalarKind::Integer(IntegerRadix::Bin);
                        let stream = &raw.as_str()[2..];
                        ensure_no_sign(stream, raw, error);
                        decode_float_or_integer(stream, raw, kind, output, error)
                    },
                    b'd' | b'D' =>  {
                        let kind = ScalarKind::Integer(IntegerRadix::Dec);
                        let stream = &raw.as_str()[2..];
                        error.report_error(ParseError::new("redundant integer number prefix").with_context(Span::new_unchecked(0, raw.len())).with_expected(&[]).with_unexpected(Span::new_unchecked(0, 2)));
                        decode_float_or_integer(stream, raw, kind, output, error)
                    },
                    _ => {
                        decode_datetime_or_float_or_integer(raw, output, error)
                    }
                }
            }
        },
        b'1'..=b'9' => decode_datetime_or_float_or_integer(raw, output, error),
        // Report as if they were numbers because its most likely a typo
        b'.' => decode_as_is(raw, ScalarKind::Float, output, error),
        b't' | b'T' => {
            let symbol = "true";
            let expected = &[Expected::Literal("true")];
            let kind = ScalarKind::Boolean(true);
            decode_symbol(raw, symbol, kind, expected, output, error)
        }
        b'f' | b'F' => {
            let symbol = "false";
            let expected = &[Expected::Literal("false")];
            let kind = ScalarKind::Boolean(false);
            decode_symbol(raw, symbol, kind, expected, output, error)
        }
        b'i' | b'I' => {
            let symbol = "inf";
            let expected = &[Expected::Literal("inf")];
            let kind = ScalarKind::Float;
            decode_symbol(raw, symbol, kind, expected, output, error)
        }
        b'n' | b'N' => {
            let symbol = "nan";
            let expected = &[Expected::Literal("nan")];
            let kind = ScalarKind::Float;
            decode_symbol(raw, symbol, kind, expected, output, error)
        }
        _ => decode_invalid(raw, output, error),
    }
}

pub(crate) fn decode_datetime_or_float_or_integer<'i>(
    raw: Raw<'i>,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    let Some(digit_end) = raw
        .as_bytes()
        .offset_for(|b| !(b'0'..=b'9').contains_token(b))
    else {
        let kind = ScalarKind::Integer(IntegerRadix::Dec);
        let stream = raw.as_str();
        return decode_float_or_integer(stream, raw, kind, output, error);
    };

    #[cfg(feature = "unsafe")] // SAFETY: ascii digits ensures UTF-8 boundary
    let rest = unsafe { &raw.as_str().get_unchecked(digit_end..) };
    #[cfg(not(feature = "unsafe"))]
    let rest = &raw.as_str()[digit_end..];

    if rest.starts_with("-") || rest.starts_with(":") {
        decode_as_is(raw, ScalarKind::DateTime, output, error)
    } else if is_float(rest) {
        let kind = ScalarKind::Float;
        let stream = raw.as_str();
        decode_float_or_integer(stream, raw, kind, output, error)
    } else if rest.starts_with("_") {
        let kind = ScalarKind::Integer(IntegerRadix::Dec);
        let stream = raw.as_str();
        decode_float_or_integer(stream, raw, kind, output, error)
    } else {
        decode_invalid(raw, output, error)
    }
}

pub(crate) fn ensure_no_sign(value: &str, raw: Raw<'_>, error: &mut dyn ErrorSink) {
    let invalid = ['+', '-'];
    if value.starts_with(invalid) {
        let pos = raw.as_str().find(invalid).unwrap();
        error.report_error(
            ParseError::new("unexpected sign")
                .with_context(Span::new_unchecked(0, raw.len()))
                .with_expected(&[])
                .with_unexpected(Span::new_unchecked(pos, pos + 1)),
        );
    }
}

pub(crate) fn decode_float_or_integer<'i>(
    stream: &'i str,
    raw: Raw<'i>,
    kind: ScalarKind,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    output.clear();

    let underscore = "_";

    if has_underscore(stream) {
        if stream.starts_with(underscore) {
            error.report_error(
                ParseError::new("`_` may only go between digits")
                    .with_context(Span::new_unchecked(0, raw.len()))
                    .with_expected(&[])
                    .with_unexpected(Span::new_unchecked(0, underscore.len())),
            );
        }
        if 1 < stream.len() && stream.ends_with(underscore) {
            let start = stream.offset_from(&raw.as_str());
            let end = start + stream.len();
            error.report_error(
                ParseError::new("`_` may only go between digits")
                    .with_context(Span::new_unchecked(0, raw.len()))
                    .with_expected(&[])
                    .with_unexpected(Span::new_unchecked(end - underscore.len(), end)),
            );
        }

        for part in stream.split(underscore) {
            let part_start = part.offset_from(&raw.as_str());
            if 0 < part_start {
                let first = part.as_bytes().first().copied().unwrap_or(b'0');
                if !is_any_digit(first, kind) {
                    let start = part_start - 1;
                    let end = part_start;
                    debug_assert_eq!(&raw.as_str()[start..end], underscore);
                    error.report_error(
                        ParseError::new("`_` may only go between digits")
                            .with_context(Span::new_unchecked(0, raw.len()))
                            .with_unexpected(Span::new_unchecked(start, end)),
                    );
                }
            }
            let part_end = part_start + part.len();
            if 1 < part.len() && part_end < raw.len() {
                let last = part.as_bytes().last().copied().unwrap_or(b'0');
                if !is_any_digit(last, kind) {
                    let start = part_end;
                    let end = start + underscore.len();
                    debug_assert_eq!(&raw.as_str()[start..end], underscore);
                    error.report_error(
                        ParseError::new("`_` may only go between digits")
                            .with_context(Span::new_unchecked(0, raw.len()))
                            .with_unexpected(Span::new_unchecked(start, end)),
                    );
                }
            }

            if !part.is_empty() && !output.push_str(part) {
                error.report_error(
                    ParseError::new(ALLOCATION_ERROR)
                        .with_unexpected(Span::new_unchecked(part_start, part_end)),
                );
            }
        }
    } else {
        if !output.push_str(stream) {
            error.report_error(
                ParseError::new(ALLOCATION_ERROR)
                    .with_unexpected(Span::new_unchecked(0, raw.len())),
            );
        }
    }

    kind
}

fn is_any_digit(b: u8, kind: ScalarKind) -> bool {
    if kind == ScalarKind::Float {
        is_dec_integer_digit(b)
    } else {
        is_any_integer_digit(b)
    }
}

fn is_any_integer_digit(b: u8) -> bool {
    (b'0'..=b'9', b'a'..=b'f', b'A'..=b'F').contains_token(b)
}

fn is_dec_integer_digit(b: u8) -> bool {
    (b'0'..=b'9').contains_token(b)
}

fn has_underscore(raw: &str) -> bool {
    raw.as_bytes().find_slice(b'_').is_some()
}

fn is_float(raw: &str) -> bool {
    raw.as_bytes().find_slice((b'.', b'e', b'E')).is_some()
}

pub(crate) fn decode_as_is<'i>(
    raw: Raw<'i>,
    kind: ScalarKind,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    let kind = decode_as(raw, raw.as_str(), kind, output, error);
    kind
}

pub(crate) fn decode_as<'i>(
    raw: Raw<'i>,
    symbol: &'i str,
    kind: ScalarKind,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    output.clear();
    if !output.push_str(symbol) {
        error.report_error(
            ParseError::new(ALLOCATION_ERROR).with_unexpected(Span::new_unchecked(0, raw.len())),
        );
    }
    kind
}

pub(crate) fn decode_symbol<'i>(
    raw: Raw<'i>,
    symbol: &'static str,
    kind: ScalarKind,
    expected: &'static [Expected],
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    if raw.as_str() != symbol {
        error.report_error(
            ParseError::new(kind.invalid_description())
                .with_context(Span::new_unchecked(0, raw.len()))
                .with_expected(expected)
                .with_unexpected(Span::new_unchecked(0, raw.len())),
        );
    }

    decode_as(raw, symbol, kind, output, error)
}

pub(crate) fn decode_invalid<'i>(
    raw: Raw<'i>,
    output: &mut dyn StringBuilder<'i>,
    error: &mut dyn ErrorSink,
) -> ScalarKind {
    error.report_error(
        ParseError::new("string values must be quoted")
            .with_context(Span::new_unchecked(0, raw.len()))
            .with_expected(&[Expected::Description("literal string")])
            .with_unexpected(Span::new_unchecked(0, raw.len())),
    );

    output.clear();
    if !output.push_str(raw.as_str()) {
        error.report_error(
            ParseError::new(ALLOCATION_ERROR).with_unexpected(Span::new_unchecked(0, raw.len())),
        );
    }
    ScalarKind::String
}
