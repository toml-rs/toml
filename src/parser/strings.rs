use crate::parser::errors::CustomError;
use crate::parser::trivia::{
    from_utf8_unchecked, is_non_ascii, is_wschar, newline, ws, ws_newlines,
};
use combine::error::Commit;
use combine::parser::byte::{byte, bytes, hex_digit};
use combine::parser::range::{range, recognize, take_while, take_while1};
use combine::stream::RangeStream;
use combine::*;
use std::borrow::Cow;
use std::char;

// ;; String

// string = ml-basic-string / basic-string / ml-literal-string / literal-string
parse!(string() -> String, {
    choice((
        ml_basic_string(),
        basic_string(),
        ml_literal_string(),
        literal_string().map(|s: &'a str| s.into()),
    ))
});

// ;; Basic String

// basic-string = quotation-mark *basic-char quotation-mark
parse!(basic_string() -> String, {
    between(
        byte(QUOTATION_MARK), byte(QUOTATION_MARK),
        many(basic_chars())
    )
    .message("While parsing a Basic String")
});

// quotation-mark = %x22            ; "
pub(crate) const QUOTATION_MARK: u8 = b'"';

// basic-char = basic-unescaped / escaped
parse!(basic_chars() -> Cow<'a, str>, {
    choice((
        // Deviate from the official grammar by batching the unescaped chars so we build a string a
        // chunk at a time, rather than a `char` at a time.
        take_while1(is_basic_unescaped).and_then(std::str::from_utf8).map(Cow::Borrowed),
        escaped().map(|c| Cow::Owned(String::from(c))),
    ))
});

// basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
#[inline]
fn is_basic_unescaped(c: u8) -> bool {
    is_wschar(c) | matches!(c, 0x21 | 0x23..=0x5B | 0x5D..=0x7E) | is_non_ascii(c)
}

// escaped = escape escape-seq-char
parse!(escaped() -> char, {
    satisfy(|c| c == ESCAPE)
        .then(|_| parser(move |input| {
            escale_seq_char().parse_stream(input).into_result()
        }))
});

// escape = %x5C                    ; \
const ESCAPE: u8 = b'\\';

// escape-seq-char =  %x22         ; "    quotation mark  U+0022
// escape-seq-char =/ %x5C         ; \    reverse solidus U+005C
// escape-seq-char =/ %x62         ; b    backspace       U+0008
// escape-seq-char =/ %x66         ; f    form feed       U+000C
// escape-seq-char =/ %x6E         ; n    line feed       U+000A
// escape-seq-char =/ %x72         ; r    carriage return U+000D
// escape-seq-char =/ %x74         ; t    tab             U+0009
// escape-seq-char =/ %x75 4HEXDIG ; uXXXX                U+XXXX
// escape-seq-char =/ %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX
parse!(escale_seq_char() -> char, {
    satisfy(is_escape_seq_char)
        .message("While parsing escape sequence")
        .then(|c| {
            parser(move |input| {
                match c {
                    b'b'  => Ok(('\u{8}', Commit::Peek(()))),
                    b'f'  => Ok(('\u{c}', Commit::Peek(()))),
                    b'n'  => Ok(('\n',    Commit::Peek(()))),
                    b'r'  => Ok(('\r',    Commit::Peek(()))),
                    b't'  => Ok(('\t',    Commit::Peek(()))),
                    b'u'  => hexescape(4).parse_stream(input).into_result(),
                    b'U'  => hexescape(8).parse_stream(input).into_result(),
                    b'\\' => Ok(('\\',    Commit::Peek(()))),
                    b'"'  => Ok(('"',     Commit::Peek(()))),
                    _ => unreachable!("{:?} filtered out by is_escape_seq_char", c),
                }
            })
        })
});

#[inline]
fn is_escape_seq_char(c: u8) -> bool {
    matches!(
        c,
        b'"' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't' | b'u' | b'U'
    )
}

parse!(hexescape(n: usize) -> char, {
    recognize(skip_count_min_max(
        *n, *n,
        hex_digit(),
    ))
        .map(|b: &[u8]| unsafe { from_utf8_unchecked(b, "`is_ascii_digit` filters out on-ASCII") })
        .and_then(|s| u32::from_str_radix(s, 16))
        .and_then(|h| char::from_u32(h).ok_or(CustomError::InvalidHexEscape(h)))
});

// ;; Multiline Basic String

// ml-basic-string = ml-basic-string-delim [ newline ] ml-basic-body
//                   ml-basic-string-delim
parse!(ml_basic_string() -> String, {
    (
        range(ML_BASIC_STRING_DELIM),
        (
            optional(newline()),
            ml_basic_body(),
        ).map(|t| t.1),
        // Deviate from grammar by pulling mll_quotes into here so we can handle the confusion with
        // it and the closing delim
        choice((
            attempt((
                bytes(b"\"\""), range(ML_BASIC_STRING_DELIM)
            )).map(|_| Some("\"\"")),
            attempt((
                bytes(b"\""), range(ML_BASIC_STRING_DELIM)
            )).map(|_| Some("\"")),
            range(ML_BASIC_STRING_DELIM).map(|_| None),
        )),
    ).map(|(_, mut b, q)| {
        if let Some(q) = q {
            b.push_str(q);
        }
        b
    }).message("While parsing a Multiline Basic String")
});

// ml-basic-string-delim = 3quotation-mark
const ML_BASIC_STRING_DELIM: &[u8] = b"\"\"\"";

// ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]
parse!(ml_basic_body() -> String, {
    (
        many(mlb_content()),
        many(attempt((
            mlb_quotes(),
            many1(mlb_content()),
        ).map(|(q, c): (&str, String)| {
            let mut total = q.to_owned();
            total.push_str(&c);
            total
        }))),
        // Deviate: see `ml_basic_string`
        //optional(mll_quotes()),
    ).map(|(mut c, qc): (String, String)| {
        c.push_str(&qc);
        c
    })
});

// mlb-content = mlb-char / newline / mlb-escaped-nl
// mlb-char = mlb-unescaped / escaped
parse!(mlb_content() -> Cow<'a, str>, {
    choice((
        // Deviate from the official grammar by batching the unescaped chars so we build a string a
        // chunk at a time, rather than a `char` at a time.
        take_while1(is_mlb_unescaped).and_then(std::str::from_utf8).map(Cow::Borrowed),
        attempt(escaped().map(|c| Cow::Owned(String::from(c)))),
        newline().map(|_| Cow::Borrowed("\n")),
        mlb_escaped_nl().map(|_| Cow::Borrowed("")),
    ))
});

// mlb-quotes = 1*2quotation-mark
parse!(mlb_quotes() -> &'a str, {
    choice((
        attempt(bytes(b"\"\"")),
        attempt(bytes(b"\"")),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`bytes` out npn-ASCII") }
    })
});

// mlb-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
#[inline]
fn is_mlb_unescaped(c: u8) -> bool {
    is_wschar(c) | matches!(c, 0x21 | 0x23..=0x5B | 0x5D..=0x7E) | is_non_ascii(c)
}

// mlb-escaped-nl = escape ws newline *( wschar / newline
// When the last non-whitespace character on a line is a \,
// it will be trimmed along with all whitespace
// (including newlines) up to the next non-whitespace
// character or closing delimiter.
parse!(mlb_escaped_nl() -> (), {
    skip_many1(attempt((
        byte(ESCAPE),
        ws(),
        ws_newlines(),
    )))
});

// ;; Literal String

// literal-string = apostrophe *literal-char apostrophe
parse!(literal_string() -> &'a str, {
    between(
        byte(APOSTROPHE), byte(APOSTROPHE),
        take_while(is_literal_char)
    ).and_then(std::str::from_utf8)
        .message("While parsing a Literal String")
});

// apostrophe = %x27 ; ' apostrophe
pub(crate) const APOSTROPHE: u8 = b'\'';

// literal-char = %x09 / %x20-26 / %x28-7E / non-ascii
#[inline]
fn is_literal_char(c: u8) -> bool {
    matches!(c, 0x09 | 0x20..=0x26 | 0x28..=0x7E) | is_non_ascii(c)
}

// ;; Multiline Literal String

// ml-literal-string = ml-literal-string-delim [ newline ] ml-literal-body
//                     ml-literal-string-delim
parse!(ml_literal_string() -> String, {
    (
        range(ML_LITERAL_STRING_DELIM),
        (
            optional(newline()),
            ml_literal_body(),
        ).map(|t| t.1.replace("\r\n", "\n")),
        // Deviate from grammar by pulling mll_quotes into here so we can handle the confusion with
        // it and the closing delim
        choice((
            attempt((
                bytes(b"''"), range(ML_LITERAL_STRING_DELIM)
            )).map(|_| Some("''")),
            attempt((
                bytes(b"'"), range(ML_LITERAL_STRING_DELIM)
            )).map(|_| Some("'")),
            range(ML_LITERAL_STRING_DELIM).map(|_| None),
        ))
    ).map(|(_, mut b, q)| {
        if let Some(q) = q {
            b.push_str(q);
        }
        b
    }).message("While parsing a Multiline Literal String")
});

// ml-literal-string-delim = 3apostrophe
const ML_LITERAL_STRING_DELIM: &[u8] = b"'''";

// ml-literal-body = *mll-content *( mll-quotes 1*mll-content ) [ mll-quotes ]
parse!(ml_literal_body() -> &'a str, {
    recognize((
        skip_many(mll_content()),
        skip_many(attempt((mll_quotes(), skip_many1(mll_content())))),
        // Deviate: see ml_literal_string
        //optional(mll_quotes()),
    )).and_then(std::str::from_utf8)
});

// mll-content = mll-char / newline
parse!(mll_content() -> u8, {
    choice((
        satisfy(is_mll_char),
        newline().map(|_| b'\n')
    ))
});

// mll-char = %x09 / %x20-26 / %x28-7E / non-ascii
#[inline]
fn is_mll_char(c: u8) -> bool {
    matches!(c, 0x09 | 0x20..=0x26 | 0x28..=0x7E) | is_non_ascii(c)
}

// mll-quotes = 1*2apostrophe
parse!(mll_quotes() -> &'a str, {
    choice((
        attempt(bytes(b"''")),
        attempt(bytes(b"'")),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`bytes` out npn-ASCII") }
    })
});
