use crate::decor::InternalString;
use crate::parser::strings::{basic_string, literal_string};
use combine::parser::range::{recognize_with_value, take_while1};
use combine::stream::RangeStream;
use combine::*;

#[inline]
fn is_unquoted_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_')
}

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
parse!(unquoted_key() -> &'a str, {
    take_while1(is_unquoted_char)
});

// key = unquoted-key / basic-string / literal-string
parse!(key() -> (&'a str, InternalString), {
    recognize_with_value(choice((
        basic_string(),
        literal_string().map(|s: &'a str| s.into()),
        unquoted_key().map(|s: &'a str| s.into()),
    )))
});
