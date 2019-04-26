use combine::char::{char, digit};
use combine::range::{range, recognize};
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
                satisfy(|c| '1' <= c && c <= '9'),
                skip_many((
                    optional(char('_')),
                    skip_many1(digit()),
                )),
            ).map(|t| t.0),
        )),
    ))
});

parse!(integer() -> i64, {
    parse_integer()
        .and_then(|s| s.replace("_", "").parse())
        .message("While parsing an Integer")
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
