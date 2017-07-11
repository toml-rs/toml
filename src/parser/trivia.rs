use parser::Span;
use parser::errors::ErrorKind;
use nom;

// wschar = ( %x20 /              ; Space
//            %x09 )              ; Horizontal tab
#[inline]
fn is_wschar(c: char) -> bool {
    match c {
        ' ' | '\t' => true,
        _ => false,
    }
}

// ws = *wschar
named!(#[inline], pub ws(Span) -> Span,
       take_while!(is_wschar)
);

// comment-start-symbol = %x23 ; #
const COMMENT_START_SYMBOL: &str = "#";

// non-eol = %x09 / %x20-10FFFF
#[inline]
fn is_non_eol(c: char) -> bool {
    match c {
        '\u{09}' | '\u{20}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

// comment = comment-start-symbol *non-eol
named!(#[inline], pub comment(Span) -> Span,
    do_parse!(
        peek!(complete!(tag!(COMMENT_START_SYMBOL))) >>
     c: take_while!(is_non_eol) >>
        (c)
    )
);

// newline = ( %x0A /              ; LF
//             %x0D.0A )           ; CRLF
named!(#[inline], pub newline(Span) -> Span,
       alt_complete!(tag!("\n") | tag!("\r\n"))
);

// ws-newline       = *( wschar / newline )
named!(pub ws_newline(Span) -> Span,
       // (\/)_;;;;_(\/)
       recognize!(
           fold_many0!(
               alt_complete!(
                   take_while1!(is_wschar)
                 | newline
               ),
               (), |_, _| ()
           )
       )
);

// // ws-newlines      = newline *( wschar / newline )
named!(pub ws_newlines(Span) -> Span,
       do_parse!(
           peek!(newline) >>
       wn: ws_newline     >>
           (wn)
       )
);

// note: this rule is not present in the original grammar
// ws-comment-newline = *( ws-newline-nonempty / comment )
named!(pub ws_comment_newline(Span) -> Span,
       // (\/)_;;;;_(\/)
       recognize!(fold_many0!(
           alt_complete!(
               fold_many1!(
                   alt_complete!(
                       take_while1!(is_wschar)
                     | newline
                   ), (), |_, _| ()
               )
             | comment => { |_| () }
           ), (), |_, _| ()
       ))
);

// note: this rule is not present in the original grammar
named!(pub line_ending(Span) -> Span,
       err!(ErrorKind::ExpectedNewlineOrEof,
            alt_complete!(
                newline
              | eof!()
            )
       )
);

// note: this rule is not present in the original grammar
named!(pub line_trailing(Span) -> Span,
       recognize!(
           tuple!(
               ws,
               opt!(comment),
               line_ending
           )
       )
);
