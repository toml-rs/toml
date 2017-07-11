use nom::{self, InputLength, Slice};
use ::decor::InternalString;
use parser::strings::{basic_string, literal_string};
use parser::Span;

#[inline]
fn is_unquoted_char(c: char) -> bool {
    match c {
        'A'...'Z' | 'a'...'z' | '0'...'9' | '-' | '_' => true,
        _ => false,
    }
}

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
named!(#[inline], unquoted_key(Span) -> Span,
       take_while1!(is_unquoted_char)
);

// key = unquoted-key / basic-string / literal-string
named!(pub key(Span) -> (InternalString, Span),
       with_input!(alt_complete!(
           basic_string
         | literal_string => { |s: Span| InternalString::from(s.fragment) }
         | unquoted_key   => { |s: Span| InternalString::from(s.fragment) }
       ))
);
