use nom;
use std::mem;
use parser::Span;
use std::fmt::{Display, Formatter, Result};
use std::error;

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    unparsed_line: String,
    line_number: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub(crate) enum ErrorKind {
    Unknown = 1001,
    ExpectedNewlineOrEof,
    ExpectedEquals,
    InvalidCharInString,
    InvalidEscapeChar,
    UnterminatedString,
    UnterminatedInlineTable,
    UnterminatedArray,
    InvalidValue,
    InvalidNumber,
    InvalidHeader,
    InvalidDateTime,
    MixedArrayType,
    DuplicateKey,
    #[doc(hidden)]
    __Nonexhaustive,
}

pub(crate) fn is_custom<E>(e: &nom::Err<E>) -> bool {
    use nom::Err;
    let kind = match *e {
        Err::Code(ref e) | Err::Node(ref e, _) | Err::Position(ref e, _) | Err::NodePosition(ref e, _, _) => e
    };
    is_custom_kind(kind)
}

fn is_custom_kind(k: &nom::ErrorKind) -> bool {
    match *k {
        nom::ErrorKind::Custom(..) => true,
        _ => false,
    }
}

unsafe fn to_custom(k: &nom::ErrorKind) -> ErrorKind {
    match *k {
        nom::ErrorKind::Custom(code) => mem::transmute(code),
        _ => unreachable!("call `to_custom` only on custom errors"),
    }
}

pub(crate) fn to_error(e: &nom::Err<Span>) -> Error {
    find_innermost_custom(e)
        .unwrap_or_else(|| Error::new(ErrorKind::Unknown,
                                      Span::new(&format!("{:?}", e))))
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, span: Span) -> Self {
        let s = span.fragment;
        Self {
            kind: kind,
            unparsed_line: s[..s.find('\n').unwrap_or_else(|| s.len())].into(),
            line_number: span.line,
        }
    }
}

fn find_innermost_custom(e: &nom::Err<Span>) -> Option<Error> {
    use nom::Err;
    match *e {
        Err::NodePosition(ref k, s, ref v) => {
            if let Some(err) = v.iter().last() {
                let last = find_innermost_custom(err);
                if last.is_some() {
                    return last;
                }
            }
            debug_assert!(is_custom(e));
            Some(Error::new(unsafe { to_custom(k) }, s))
        }
        Err::Position(ref k, s) if is_custom(e) => Some(Error::new(unsafe { to_custom(k) }, s)),
        _ => None,
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TOML parse error {:?} at line {}: {}",
               self.kind, self.line_number, self.unparsed_line)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "TOML parse error"
    }
}
