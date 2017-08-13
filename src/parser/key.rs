use combine::*;
use combine::primitives::RangeStream;
use combine::range::{recognize_with_value, take_while1};
use parser::strings::{basic_string, literal_string};
use decor::InternalString;

#[inline]
fn is_unquoted_char(c: char) -> bool {
    match c {
        'A'...'Z' | 'a'...'z' | '0'...'9' | '-' | '_' => true,
        _ => false,
    }
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
