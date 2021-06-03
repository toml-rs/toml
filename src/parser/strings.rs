use crate::decor::InternalString;
use crate::parser::errors::CustomError;
use crate::parser::trivia::{newline, ws, ws_newlines};
use combine::error::{Commit, Info};
use combine::parser::char::char;
use combine::parser::range::{range, take, take_while};
use combine::stream::RangeStream;
use combine::*;
use std::char;

// ;; String

// string = ml-basic-string / basic-string / ml-literal-string / literal-string
parse!(string() -> InternalString, {
    choice((
        ml_basic_string(),
        basic_string(),
        ml_literal_string(),
        literal_string().map(|s: &'a str| s.into()),
    ))
});

// basic-unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
#[inline]
fn is_basic_unescaped(c: char) -> bool {
    matches!(c, '\u{20}'..='\u{21}' | '\u{23}'..='\u{5B}' | '\u{5D}'..='\u{10FFFF}')
}

// escaped = escape ( %x22 /          ; "    quotation mark  U+0022
//                    %x5C /          ; \    reverse solidus U+005C
//                    %x2F /          ; /    solidus         U+002F
//                    %x62 /          ; b    backspace       U+0008
//                    %x66 /          ; f    form feed       U+000C
//                    %x6E /          ; n    line feed       U+000A
//                    %x72 /          ; r    carriage return U+000D
//                    %x74 /          ; t    tab             U+0009
//                    %x75 4HEXDIG /  ; uXXXX                U+XXXX
//                    %x55 8HEXDIG )  ; UXXXXXXXX            U+XXXXXXXX
#[inline]
fn is_escape_char(c: char) -> bool {
    matches!(
        c,
        '\\' | '"' | 'b' | '/' | 'f' | 'n' | 'r' | 't' | 'u' | 'U'
    )
}

parse!(escape() -> char, {
    satisfy(is_escape_char)
        .message("While parsing escape sequence")
        .then(|c| {
            parser(move |input| {
                match c {
                    'b'  => Ok(('\u{8}', Commit::Peek(()))),
                    'f'  => Ok(('\u{c}', Commit::Peek(()))),
                    'n'  => Ok(('\n',    Commit::Peek(()))),
                    'r'  => Ok(('\r',    Commit::Peek(()))),
                    't'  => Ok(('\t',    Commit::Peek(()))),
                    'u'  => hexescape(4).parse_stream(input).into_result(),
                    'U'  => hexescape(8).parse_stream(input).into_result(),
                    // ['\\', '"', '/']
                    _    => Ok((c,       Commit::Peek(()))),
                }
            })
        })
});

parse!(hexescape(n: usize) -> char, {
    take(*n)
        .and_then(|s| u32::from_str_radix(s, 16))
        .and_then(|h| char::from_u32(h).ok_or(CustomError::InvalidHexEscape(h)))
});

// escape = %x5C                    ; \
const ESCAPE: char = '\\';

// basic-char = basic-unescaped / escaped
parse!(basic_char() -> char, {
    satisfy(|c| is_basic_unescaped(c) || c == ESCAPE)
        .then(|c| parser(move |input| {
            match c {
                ESCAPE => escape().parse_stream(input).into_result(),
                _      => Ok((c, Commit::Peek(()))),
            }
        }))
});

// quotation-mark = %x22            ; "
const QUOTATION_MARK: char = '"';

// basic-string = quotation-mark *basic-char quotation-mark
parse!(basic_string() -> InternalString, {
    between(char(QUOTATION_MARK), char(QUOTATION_MARK),
            many(basic_char()))
        .message("While parsing a Basic String")
});

// ;; Multiline Basic String

// ml-basic-unescaped = %x20-5B / %x5D-10FFFF
#[inline]
fn is_ml_basic_unescaped(c: char) -> bool {
    matches!(c, '\u{20}'..='\u{5B}' | '\u{5D}'..='\u{10FFFF}')
}

// ml-basic-string-delim = 3quotation-mark
const ML_BASIC_STRING_DELIM: &str = "\"\"\"";

// ml-basic-char = ml-basic-unescaped / escaped
parse!(ml_basic_char() -> char, {
    satisfy(|c| is_ml_basic_unescaped(c) || c == ESCAPE)
        .then(|c| parser(move |input| {
            match c {
                ESCAPE => escape().parse_stream(input).into_result(),
                _      => Ok((c, Commit::Peek(()))),
            }
        }))
});

// When the last non-whitespace character on a line is a \,
// it will be trimmed along with all whitespace
// (including newlines) up to the next non-whitespace
// character or closing delimiter.
parse!(try_eat_escaped_newline() -> (), {
    skip_many(attempt((
        char(ESCAPE),
        ws(),
        ws_newlines(),
    )))
});

// ml-basic-body = *( ( escape ws-newline ) / ml-basic-char / newline )
parse!(ml_basic_body() -> InternalString, {
    //  A newline immediately following the opening delimiter will be trimmed.
    optional(newline())
        .skip(try_eat_escaped_newline())
        .with(
            many(
                not_followed_by(range(ML_BASIC_STRING_DELIM).map(Info::Range))
                    .with(
                        choice((
                            // `TOML parsers should feel free to normalize newline
                            //  to whatever makes sense for their platform.`
                            newline(),
                            ml_basic_char(),
                        ))
                    )
                    .skip(try_eat_escaped_newline())
            )
        )
});

// ml-basic-string = ml-basic-string-delim ml-basic-body ml-basic-string-delim
parse!(ml_basic_string() -> InternalString, {
    between(range(ML_BASIC_STRING_DELIM),
            range(ML_BASIC_STRING_DELIM),
            ml_basic_body())
        .message("While parsing a Multiline Basic String")
});

// ;; Literal String

// apostrophe = %x27 ; ' apostrophe
const APOSTROPHE: char = '\'';

// literal-char = %x09 / %x20-26 / %x28-10FFFF
#[inline]
fn is_literal_char(c: char) -> bool {
    matches!(c, '\u{09}' | '\u{20}'..='\u{26}' | '\u{28}'..='\u{10FFFF}')
}

// literal-string = apostrophe *literal-char apostrophe
parse!(literal_string() -> &'a str, {
    between(char(APOSTROPHE), char(APOSTROPHE),
            take_while(is_literal_char))
        .message("While parsing a Literal String")
});

// ;; Multiline Literal String

// ml-literal-string-delim = 3apostrophe
const ML_LITERAL_STRING_DELIM: &str = "'''";

// ml-literal-char = %x09 / %x20-10FFFF
#[inline]
fn is_ml_literal_char(c: char) -> bool {
    matches!(c, '\u{09}' | '\u{20}'..='\u{10FFFF}')
}

// ml-literal-body = *( ml-literal-char / newline )
parse!(ml_literal_body() -> InternalString, {
    //  A newline immediately following the opening delimiter will be trimmed.
    optional(newline())
        .with(
            many(
                not_followed_by(range(ML_LITERAL_STRING_DELIM).map(Info::Range))
                    .with(
                        choice((
                            // `TOML parsers should feel free to normalize newline
                            //  to whatever makes sense for their platform.`
                            newline(),
                            satisfy(is_ml_literal_char),
                        ))
                    )
            )
        )
});

// ml-literal-string = ml-literal-string-delim ml-literal-body ml-literal-string-delim
parse!(ml_literal_string() -> InternalString, {
    between(range(ML_LITERAL_STRING_DELIM),
            range(ML_LITERAL_STRING_DELIM),
            ml_literal_body())
        .message("While parsing a Multiline Literal String")
});
