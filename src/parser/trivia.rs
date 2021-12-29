use combine::parser::byte::{byte, crlf, newline as lf};
use combine::parser::range::{recognize, take_while, take_while1};
use combine::stream::RangeStream;
use combine::*;

pub(crate) unsafe fn from_utf8_unchecked<'b>(
    bytes: &'b [u8],
    safety_justification: &'static str,
) -> &'b str {
    if cfg!(debug_assertions) {
        // Catch problems more quickly when testing
        std::str::from_utf8(bytes).expect(safety_justification)
    } else {
        std::str::from_utf8_unchecked(bytes)
    }
}

// wschar = ( %x20 /              ; Space
//            %x09 )              ; Horizontal tab
#[inline]
pub(crate) fn is_wschar(c: u8) -> bool {
    matches!(c, b' ' | b'\t')
}

// ws = *wschar
parse!(ws() -> &'a str, {
    take_while(is_wschar).map(|b| {
        unsafe { from_utf8_unchecked(b, "`is_wschar` filters out on-ASCII") }
    })
});

// non-ascii = %x80-D7FF / %xE000-10FFFF
#[inline]
pub(crate) fn is_non_ascii(c: u8) -> bool {
    // - ASCII is 0xxxxxxx
    // - First byte for UTF-8 is 11xxxxxx
    // - Subsequent UTF-8 bytes are 10xxxxxx
    matches!(c, 0x80..=0xff)
}

// non-eol = %x09 / %x20-7E / non-ascii
#[inline]
fn is_non_eol(c: u8) -> bool {
    matches!(c, 0x09 | 0x20..=0x7E) | is_non_ascii(c)
}

// comment-start-symbol = %x23 ; #
pub(crate) const COMMENT_START_SYMBOL: u8 = b'#';

// comment = comment-start-symbol *non-eol
parse!(comment() -> &'a [u8], {
    recognize((
        byte(COMMENT_START_SYMBOL),
        take_while(is_non_eol),
    ))
});

// newline = ( %x0A /              ; LF
//             %x0D.0A )           ; CRLF
pub(crate) const LF: u8 = b'\n';
pub(crate) const CR: u8 = b'\r';
parse!(newline() -> char, {
    choice((lf(), crlf()))
        .map(|_| '\n')
        .expected("newline")
});

// ws-newline       = *( wschar / newline )
parse!(ws_newline() -> &'a str, {
    recognize(
        skip_many(choice((
            newline().map(|_| &b"\n"[..]),
            take_while1(is_wschar),
        ))),
    ).map(|b| {
        unsafe { from_utf8_unchecked(b, "`is_wschar` and `newline` filters out on-ASCII") }
    })
});

// ws-newlines      = newline *( wschar / newline )
parse!(ws_newlines() -> &'a str, {
    recognize((
        newline(),
        ws_newline(),
    )).map(|b| {
        unsafe { from_utf8_unchecked(b, "`is_wschar` and `newline` filters out on-ASCII") }
    })
});

// note: this rule is not present in the original grammar
// ws-comment-newline = *( ws-newline-nonempty / comment )
parse!(ws_comment_newline() -> &'a [u8], {
    recognize(
        skip_many(
            choice((
                skip_many1(
                    choice((
                        take_while1(is_wschar),
                        newline().map(|_| &b"\n"[..]),
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
parse!(line_trailing() -> &'a [u8], {
    recognize((
        ws(),
        optional(comment()),
    )).skip(line_ending())
});
