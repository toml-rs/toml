use std::char;
use nom::{self, InputLength};
use parser::errors::{ErrorKind, is_custom};
use parser::trivia::{ws, ws_newlines, newline};
use parser::Span;
use ::decor::InternalString;

// ;; String

// string = ml-basic-string / basic-string / ml-literal-string / literal-string
named!(pub string(Span) -> InternalString,
       alt_complete!(
           ml_basic_string
         | basic_string
         | ml_literal_string => { |s: Span| InternalString::from(s.fragment) }
         | literal_string    => { |s: Span| InternalString::from(s.fragment) }
       )
);

// basic-unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
#[inline]
fn is_basic_unescaped(c: char) -> bool {
    match c {
        '\u{20}'...'\u{21}' |
        '\u{23}'...'\u{5B}' |
        '\u{5D}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

// escape = %x5C                    ; \
const ESCAPE: &str = "\\";

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
named!(pub escaped(Span) -> char,
       do_parse!(
           complete!(tag!(ESCAPE)) >>
       c:  alt_custom!(e!(ErrorKind::InvalidEscapeChar),
                 tag!("\"") => { |_| '"'     }
               | tag!("\\") => { |_| '\\'    }
               | tag!("b")  => { |_| '\u{8}' } // \b
               | tag!("f")  => { |_| '\u{c}' } // \f
               | tag!("n")  => { |_| '\n'    }
               | tag!("r")  => { |_| '\r'    }
               | tag!("t")  => { |_| '\t'    }
               | do_parse!(
                   complete!(tag!("u"))      >>
                c: call!(parse_hexescape, 4) >>
                   (c)
                 )
               | do_parse!(
                   complete!(tag!("U"))      >>
                c: call!(parse_hexescape, 8) >>
                   (c)
                 )
           ) >>
           (c)
       )
);

fn parse_hexescape(input: Span, n: u8) -> nom::IResult<Span, char> {
    do_parse!(
        input,
     s: complete!(take!(n)) >>
     h: expr_res!(u32::from_str_radix(s.fragment, 16)) >>
     c: expr_opt!(char::from_u32(h)) >>
        (c)
    )
}

named!(#[inline], pub basic_unescaped(Span) -> Span,
       take_while1!(is_basic_unescaped)
);

// argh, https://github.com/rust-lang/rfcs/issues/1230
enum SpanOrChar<'a> {
    Span(Span<'a>),
    Char(char),
}

// basic-char = basic-unescaped / escaped
named!(#[inline], basic_chars(Span) -> SpanOrChar,
       do_parse!(
           not!(complete!(char!('"'))) >>
        c: alt_custom!(e!(ErrorKind::InvalidCharInString),
                       escaped           => { |c| SpanOrChar::Char(c) }
                     | basic_unescaped   => { |s| SpanOrChar::Span(s) }
           ) >>
           (c)
       )
);

// quotation-mark = %x22            ; "
const QUOTATION_MARK: &str = "\"";

#[inline]
fn escape_folder(mut acc: InternalString, c: &SpanOrChar) -> InternalString {
    match *c {
        SpanOrChar::Char(c) => acc.push(c),
        SpanOrChar::Span(s) => acc.push_str(s.fragment),
    }
    acc
}

// basic-string = quotation-mark *basic-char quotation-mark
named!(pub basic_string(Span) -> InternalString,
       delimited!(
           complete!(tag!(QUOTATION_MARK)),
           fold_many0_custom!(
               basic_chars,
               InternalString::new(),
               escape_folder
           ),
           err!(ErrorKind::UnterminatedString,
                complete!(tag!(QUOTATION_MARK)))
       )
);

// ;; Multiline Basic String

// note: this rule is modified (quote isn't included)
// ml-basic-unescaped = %x20-5B / %x5D-10FFFF
#[inline]
fn is_ml_basic_unescaped(c: char) -> bool {
    match c {
        '\u{20}'...'\u{21}' |
        '\u{23}'...'\u{5B}' |
        '\u{5D}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

named!(#[inline], ml_basic_unescaped(Span) -> Span,
       take_while1!(is_ml_basic_unescaped)
);

// ml-basic-char = ml-basic-unescaped / escaped
named!(#[inline], ml_basic_chars_or_newline(Span) -> SpanOrChar,
       alt_custom!(e!(ErrorKind::InvalidCharInString),
           // TOML parsers should feel free to normalize newline
           // to whatever makes sense for their platform.
           newline            => { |_| SpanOrChar::Char('\n') }
         | tag!("\"")         => { |_| SpanOrChar::Char('"') }
         | escaped            => { |c| SpanOrChar::Char(c) }
         | ml_basic_unescaped => { |s| SpanOrChar::Span(s) }
       )
);

// ml-basic-string-delim = 3quotation-mark
const ML_BASIC_STRING_DELIM: &str = "\"\"\"";

// When the last non-whitespace character on a line is a \,
// it will be trimmed along with all whitespace
// (including newlines) up to the next non-whitespace
// character or closing delimiter.
named!(#[inline], eat_escaped_newline(Span) -> (),
       fold_many0!(
           tuple!(
               tag!(ESCAPE),
               ws,
               ws_newlines
           ),
        // (\/)_(;;)_(\/)
           (), |_, _| ()
       )
);

// ml-basic-body = *( ( escape ws-newline ) / ml-basic-char / newline )
named!(ml_basic_body(Span) -> InternalString,
       do_parse!(
           //  A newline immediately following the opening delimiter will be trimmed.
           opt!(complete!(newline)) >>
           eat_escaped_newline >>
           body: fold_many0_custom!(
               do_parse!(
                   not!(tag!(ML_BASIC_STRING_DELIM)) >>
                c: ml_basic_chars_or_newline         >>
                   eat_escaped_newline               >>
                   (c)
               ),
               InternalString::new(),
               escape_folder
           ) >>
           (body)
       )
);

// ml-basic-string = ml-basic-string-delim ml-basic-body ml-basic-string-delim
named!(pub ml_basic_string(Span) -> InternalString,
       delimited!(
           complete!(tag!(ML_BASIC_STRING_DELIM)),
           ml_basic_body,
           err!(ErrorKind::UnterminatedString,
                complete!(tag!(ML_BASIC_STRING_DELIM)))
       )
);

// ;; Literal String

// apostrophe = %x27 ; ' apostrophe
const APOSTROPHE: &str = "'";

// literal-char = %x09 / %x20-26 / %x28-10FFFF
fn is_literal_char(c: char) -> bool {
    match c {
        '\u{09}' |
        '\u{20}'...'\u{26}' |
        '\u{28}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

named!(parse_literal_body(Span) -> Span,
       take_while!(is_literal_char)
);

macro_rules! string_literal_body (
    ($input:expr, $delim:ident, $verify:ident) => (
        do_parse!(
            $input,
         s: err!(ErrorKind::UnterminatedString,
                 complete!(take_until!($delim))) >>
            err!(ErrorKind::InvalidCharInString,
                 expr_opt!({
                     match $verify(s) {
                         nom::IResult::Done(rem, _) if rem.input_len() == 0 => {
                             Some(())
                         },
                         _ => None,
                     }
                 })
            ) >>
            (s)
        )
    );
);

// literal-string = apostrophe *literal-char apostrophe
named!(pub literal_string(Span) -> Span,
       delimited!(
           complete!(tag!(APOSTROPHE)),
           string_literal_body!(APOSTROPHE, parse_literal_body),
           tag!(APOSTROPHE)
       )
);


// ;; Multiline Literal String

// ml-literal-string-delim = 3apostrophe
const ML_LITERAL_STRING_DELIM: &str = "'''";

// ml-literal-char = %x09 / %x20-10FFFF
fn is_ml_literal_char(c: char) -> bool {
    match c {
        '\u{09}' | '\u{20}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

named!(parse_ml_literal_body(Span) -> (),
       fold_many0!(
           alt_complete!(
               take_while1!(is_ml_literal_char)
             | newline
           ), (), |_, _| ()
       )
);

// ml-literal-body = *( ml-literal-char / newline )
named!(pub ml_literal_body(Span) -> Span,
       do_parse!(
           //  A newline immediately following the opening delimiter will be trimmed.
           opt!(complete!(newline))                      >>
     body: string_literal_body!(ML_LITERAL_STRING_DELIM,
                                parse_ml_literal_body)   >>
           (body)
       )
);

// ml-literal-string = ml-literal-string-delim ml-literal-body ml-literal-string-delim
named!(pub ml_literal_string(Span) -> Span,
       delimited!(
           complete!(tag!(ML_LITERAL_STRING_DELIM)),
           ml_literal_body,
           tag!(ML_LITERAL_STRING_DELIM)
       )
);
