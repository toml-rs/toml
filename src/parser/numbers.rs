use crate::parser::trivia::from_utf8_unchecked;
use combine::parser::byte::{byte, bytes, digit, hex_digit, oct_digit};
use combine::parser::range::{range, recognize};
use combine::stream::RangeStream;
use combine::*;

// ;; Boolean

// boolean = true / false
pub(crate) const TRUE: &[u8] = b"true";
pub(crate) const FALSE: &[u8] = b"false";
parse!(boolean() -> bool, {
    choice((
        (byte(TRUE[0]), range(&TRUE[1..]),),
        (byte(FALSE[0]), range(&FALSE[1..]),),
    )).map(|p| p.0 == b't')
});
parse!(true_() -> bool, {
    range(crate::parser::numbers::TRUE).map(|_| true)
});
parse!(false_() -> bool, {
    range(crate::parser::numbers::FALSE).map(|_| false)
});

// ;; Integer

// integer = dec-int / hex-int / oct-int / bin-int
parse!(integer() -> i64, {
    choice!(
        attempt(hex_int()),
        attempt(oct_int()),
        attempt(bin_int()),
        dec_int()
            .and_then(|s| s.replace("_", "").parse())
            .message("While parsing an Integer")
    )
});

// dec-int = [ minus / plus ] unsigned-dec-int
// unsigned-dec-int = DIGIT / digit1-9 1*( DIGIT / underscore DIGIT )
parse!(dec_int() -> &'a str, {
    recognize((
        optional(choice([byte(b'-'), byte(b'+')])),
        choice((
            byte(b'0'),
            (
                satisfy(|c| (b'1'..=b'9').contains(&c)),
                skip_many((
                    optional(byte(b'_')),
                    skip_many1(digit()),
                )),
            ).map(|t| t.0),
        )),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`digit` and `_` filter out npn-ASCII") }
    })
});

// hex-prefix = %x30.78               ; 0x
// hex-int = hex-prefix HEXDIG *( HEXDIG / underscore HEXDIG )
parse!(hex_int() -> i64, {
    bytes(b"0x").with(
        recognize((
            hex_digit(),
            skip_many((
                optional(byte(b'_')),
                skip_many1(hex_digit()),
            )),
        ).map(|t| t.0)
    )).and_then(|b: &[u8]| {
        let s = unsafe { from_utf8_unchecked(b, "`hex_digit` and `_` filter out npn-ASCII") };
        i64::from_str_radix(&s.replace("_", ""), 16)
    }).message("While parsing a hexadecimal Integer")
});

// oct-prefix = %x30.6F               ; 0o
// oct-int = oct-prefix digit0-7 *( digit0-7 / underscore digit0-7 )
parse!(oct_int() -> i64, {
    bytes(b"0o").with(
        recognize((
            oct_digit(),
            skip_many((
                optional(byte(b'_')),
                skip_many1(oct_digit()),
            )),
        ).map(|t| t.0)
    )).and_then(|b: &[u8]| {
        let s = unsafe { from_utf8_unchecked(b, "`oct_digit` and `_` filter out npn-ASCII") };
        i64::from_str_radix(&s.replace("_", ""), 8)
    }).message("While parsing a octal Integer")
});

// bin-prefix = %x30.62               ; 0b
// bin-int = bin-prefix digit0-1 *( digit0-1 / underscore digit0-1 )
parse!(bin_int() -> i64, {
    bytes(b"0b").with(
        recognize((
            satisfy(|c: u8| c == b'0' || c == b'1'),
            skip_many((
                optional(byte(b'_')),
                skip_many1(satisfy(|c: u8| c == b'0' || c == b'1')),
            )),
        ).map(|t| t.0)
    )).and_then(|b: &[u8]| {
        let s = unsafe { from_utf8_unchecked(b, "`is_digit` and `_` filter out npn-ASCII") };
        i64::from_str_radix(&s.replace("_", ""), 2)
    }).message("While parsing a binary Integer")
});

// ;; Float

// float = float-int-part ( exp / frac [ exp ] )
// float =/ special-float
// float-int-part = dec-int
parse!(float() -> f64, {
    choice((
        parse_float()
            .and_then(|s| s.replace("_", "").parse()),
        special_float(),
    )).message("While parsing a Float")
});

parse!(parse_float() -> &'a str, {
    recognize((
        attempt((dec_int(), look_ahead(one_of([b'e', b'E', b'.'])))),
        choice((
            exp(),
            (
                frac(),
                optional(exp()),
            ).map(|_| "")
        )),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`dec_int`, `one_of`, `exp`, and `frac` filter out npn-ASCII") }
    })
});

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
parse!(frac() -> &'a str, {
    recognize((
        byte(b'.'),
        parse_zero_prefixable_int(),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`.` and `parse_zero_prefixable_int` filter out npn-ASCII") }
    })
});

// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
parse!(parse_zero_prefixable_int() -> &'a str, {
    recognize((
        skip_many1(digit()),
        skip_many((
            optional(byte(b'_')),
            skip_many1(digit()),
        )),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`digit` and `_` filter out npn-ASCII") }
    })
});

// exp = "e" float-exp-part
// float-exp-part = [ minus / plus ] zero-prefixable-int
parse!(exp() -> &'a str, {
    recognize((
        one_of([b'e', b'E']),
        optional(one_of([b'+', b'-'])),
        parse_zero_prefixable_int(),
    )).map(|b: &[u8]| {
        unsafe { from_utf8_unchecked(b, "`one_of` and `parse_zero_prefixable_int` filter out npn-ASCII") }
    })
});

// special-float = [ minus / plus ] ( inf / nan )
parse!(special_float() -> f64, {
    attempt(optional(one_of([b'+', b'-'])).and(choice((inf(), nan()))).map(|(s, f)| {
        match s {
            Some(b'+') | None => f,
            Some(b'-') => -f,
            _ => unreachable!("one_of should prevent this"),
        }
    }))
});

// inf = %x69.6e.66  ; inf
pub(crate) const INF: &[u8] = b"inf";
parse!(inf() -> f64, {
    bytes(INF).map(|_| f64::INFINITY)
});

// nan = %x6e.61.6e  ; nan
pub(crate) const NAN: &[u8] = b"nan";
parse!(nan() -> f64, {
    bytes(NAN).map(|_| f64::NAN)
});
