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

// integer = [ "-" / "+" ] int
// int = [1-9] 1*( DIGIT / _ DIGIT ) / DIGIT
// modified: int = 0 / [1-9] *( DIGIT / _ DIGIT )
parse!(parse_integer() -> &'a str, {
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

parse!(integer() -> i64, {
    choice!(
        attempt(parse_hex_integer()),
        attempt(parse_octal_integer()),
        attempt(parse_binary_integer()),
        parse_integer()
            .and_then(|s| s.replace("_", "").parse())
            .message("While parsing an Integer")
    )
});

// hex-int = "0x" HEXDIGIT *( HEXDIGIT / _ HEXDIGIT )
parse!(parse_hex_integer() -> i64, {
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

// oct-int = "0o" digit0-7 *( digit0-7 / _ digit0-7 )
parse!(parse_octal_integer() -> i64, {
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

// bin-int = "0b" digit0-1 *( digit0-1 / _ digit0-1 )
parse!(parse_binary_integer() -> i64, {
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

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
parse!(frac() -> &'a str, {
    recognize((
        char('.'),
        skip_many1(digit()),
        skip_many((
            optional(char('_')),
            skip_many1(digit()),
        )),
    ))
});

// exp = e integer
// e = %x65 / %x45                    ; e E
parse!(exp() -> &'a str, {
    recognize((
        one_of("eE".chars()),
        parse_integer(),
    ))
});

// float = integer ( frac / ( frac exp ) / exp )
parse!(parse_float() -> &'a str, {
    recognize((
        attempt((parse_integer(), look_ahead(one_of("eE.".chars())))),
        choice((
            exp(),
            (
                frac(),
                optional(exp()),
            ).map(|_| "")
        )),
    ))
});

parse!(float() -> f64, {
    parse_float()
        .and_then(|s| s.replace("_", "").parse())
        .message("While parsing a Float")
});
