use combine::parser::char::{char, crlf, newline as lf};
use combine::parser::range::{recognize, take_while, take_while1};
use combine::stream::RangeStream;
use combine::*;

// wschar = ( %x20 /              ; Space
//            %x09 )              ; Horizontal tab
#[inline]
fn is_wschar(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

// ws = *wschar
parse!(ws() -> &'a str, {
    take_while(is_wschar)
});

// non-eol = %x09 / %x20-10FFFF
#[inline]
fn is_non_eol(c: char) -> bool {
    matches!(c, '\u{09}' | '\u{20}'..='\u{10FFFF}')
}

// comment-start-symbol = %x23 ; #
const COMMENT_START_SYMBOL: char = '#';

// comment = comment-start-symbol *non-eol
parse!(comment() -> &'a str, {
    recognize((
        attempt(char(COMMENT_START_SYMBOL)),
        take_while(is_non_eol),
    ))
});

// newline = ( %x0A /              ; LF
//             %x0D.0A )           ; CRLF
parse!(newline() -> char, {
    choice((lf(), crlf()))
        .map(|_| '\n')
        .expected("a newline")
});

// ws-newline       = *( wschar / newline )
parse!(ws_newline() -> &'a str, {
    recognize(
        skip_many(choice((
            newline().map(|_| "\n"),
            take_while1(is_wschar),
        ))),
    )
});

// ws-newlines      = newline *( wschar / newline )
parse!(ws_newlines() -> &'a str, {
    recognize((
        newline(),
        ws_newline(),
    ))
});

// note: this rule is not present in the original grammar
// ws-comment-newline = *( ws-newline-nonempty / comment )
parse!(ws_comment_newline() -> &'a str, {
    recognize(
        skip_many(
            choice((
                skip_many1(
                    choice((
                        take_while1(is_wschar),
                        newline().map(|_| "\n"),
                    ))
                ),
                comment().map(|_| ()),
            ))
        )
    )
});

// note: this rule is not present in the original grammar
// line-ending = newline / eof
parse!(line_ending() -> &'a str, {
    choice((
        newline().map(|_| "\n"),
        eof().map(|_| "")
    ))
});

// note: this rule is not present in the original grammar
// line-trailing = ws [comment] skip-line-ending
parse!(line_trailing() -> &'a str, {
    recognize((
        ws(),
        optional(comment()),
    )).skip(line_ending())
});
