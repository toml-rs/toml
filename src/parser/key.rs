use crate::parser::strings::{basic_string, literal_string};
use crate::repr::InternalString;
use combine::parser::range::{recognize_with_value, take_while1};
use combine::stream::RangeStream;
use combine::*;

// key = simple-key / dotted-key

// simple-key = quoted-key / unquoted-key
parse!(simple_key() -> (&'a str, InternalString), {
    recognize_with_value(choice((
        quoted_key(),
        unquoted_key().map(|s: &'a str| s.into()),
    )))
});

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
parse!(unquoted_key() -> &'a str, {
    take_while1(is_unquoted_char)
});

#[inline]
fn is_unquoted_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_')
}

// quoted-key = basic-string / literal-string
parse!(quoted_key() -> InternalString, {
    choice((
        basic_string(),
        literal_string().map(|s: &'a str| s.into()),
    ))
});

// dotted-key = simple-key 1*( dot-sep simple-key )
