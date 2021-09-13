use combine::parser::char::{char, digit, hex_digit, oct_digit, string};
use combine::parser::range::{range, recognize};
use combine::stream::RangeStream;
use combine::*;

// ;; Boolean

// boolean = true / false
parse!(boolean() -> bool, {
    choice((
        (char('t'), range("rue"),),
        (char('f'), range("alse"),),
    )).map(|p| p.0 == 't')
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
        optional(choice([char('-'), char('+')])),
        choice((
            char('0'),
            (
                satisfy(|c| ('1'..='9').contains(&c)),
                skip_many((
                    optional(char('_')),
                    skip_many1(digit()),
                )),
            ).map(|t| t.0),
        )),
    ))
});

// hex-prefix = %x30.78               ; 0x
// hex-int = hex-prefix HEXDIG *( HEXDIG / underscore HEXDIG )
parse!(hex_int() -> i64, {
    string("0x").with(
        recognize((
            hex_digit(),
            skip_many((
                optional(char('_')),
                skip_many1(hex_digit()),
            )),
        ).map(|t| t.0)
    )).and_then(|s: &str| i64::from_str_radix(&s.replace("_", ""), 16))
       .message("While parsing a hexadecimal Integer")
});

// oct-prefix = %x30.6F               ; 0o
// oct-int = oct-prefix digit0-7 *( digit0-7 / underscore digit0-7 )
parse!(oct_int() -> i64, {
    string("0o").with(
        recognize((
            oct_digit(),
            skip_many((
                optional(char('_')),
                skip_many1(oct_digit()),
            )),
        ).map(|t| t.0)
    )).and_then(|s: &str| i64::from_str_radix(&s.replace("_", ""), 8))
       .message("While parsing an octal Integer")
});

// bin-prefix = %x30.62               ; 0b
// bin-int = bin-prefix digit0-1 *( digit0-1 / underscore digit0-1 )
parse!(bin_int() -> i64, {
    string("0b").with(
        recognize((
            satisfy(|c: char| c.is_digit(0x2)),
            skip_many((
                optional(char('_')),
                skip_many1(satisfy(|c: char| c.is_digit(0x2))),
            )),
        ).map(|t| t.0)
    )).and_then(|s: &str| i64::from_str_radix(&s.replace("_", ""), 2))
       .message("While parsing a binary Integer")
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
        attempt((dec_int(), look_ahead(one_of("eE.".chars())))),
        choice((
            exp(),
            (
                frac(),
                optional(exp()),
            ).map(|_| "")
        )),
    ))
});

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
parse!(frac() -> &'a str, {
    recognize((
        char('.'),
        parse_zero_prefixable_int(),
    ))
});

// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
parse!(parse_zero_prefixable_int() -> &'a str, {
    recognize((
        skip_many1(digit()),
        skip_many((
            optional(char('_')),
            skip_many1(digit()),
        )),
    ))
});

// exp = "e" float-exp-part
// float-exp-part = [ minus / plus ] zero-prefixable-int
parse!(exp() -> &'a str, {
    recognize((
        one_of("eE".chars()),
        optional(one_of("+-".chars())),
        parse_zero_prefixable_int(),
    ))
});

// special-float = [ minus / plus ] ( inf / nan )
parse!(special_float() -> f64, {
    attempt(optional(one_of("+-".chars())).and(choice((inf(), nan()))).map(|(s, f)| {
        match s {
            Some('+') | None => f,
            Some('-') => -f,
            _ => unreachable!("one_of should prevent this"),
        }
    }))
});

// inf = %x69.6e.66  ; inf
parse!(inf() -> f64, {
    range("inf").map(|_| f64::INFINITY)
});

// nan = %x6e.61.6e  ; nan
parse!(nan() -> f64, {
    range("nan").map(|_| f64::NAN)
});
