use nom;
use parser::errors::{ErrorKind, is_custom};
use parser::Span;

// ;; Boolean

// boolean = true / false
named!(#[inline], pub boolean(Span) -> bool,
       alt_complete!(
           tag!("true")  => { |_| true  }
         | tag!("false") => { |_| false }
       )
);

// ;; Integer

// integer = [ "-" / "+" ] int
// int = [1-9] 1*( DIGIT / _ DIGIT ) / DIGIT
named!(parse_integer(Span) -> Span,
    recognize!(
        tuple!(
            opt!(one_of!("+-")),
            alt_complete!(
                tuple!(
                    one_of!("123456789"),
                    fold_many1!(
                        tuple!(
                            opt!(complete!(tag!("_"))),
                            call!(nom::digit)
                        ), (), |_, _| ()
                    )
                )                 => { |_| () }
              | one_of!("0123456789") => { |_| () }
            )
        )
    )
);

pub fn integer(input: Span) -> nom::IResult<Span, i64> {
    let (rest, s) = try_parse!(input, parse_integer);

    match s.fragment.replace("_", "").parse::<i64>() {
        Ok(i) => nom::IResult::Done(rest, i),
        _ => e!(ErrorKind::InvalidNumber, rest),
    }
}

// ;; Float

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
named!(#[inline], frac(Span) -> Span,
       recognize!(
           complete!(
               tuple!(
                   tag!("."),
                   err!(ErrorKind::InvalidNumber,
                        call!(nom::digit)),
                   fold_many0!(
                       tuple!(opt!(complete!(tag!("_"))), call!(nom::digit)),
                       (), |_,_| ()
                   )
               )
           )
       )
);

// exp = e integer
// e = %x65 / %x45                    ; e E
named!(#[inline], exp(Span) -> Span,
       recognize!(complete!(tuple!(one_of!("eE"),
                                   err!(ErrorKind::InvalidNumber,
                                        call!(parse_integer)))))
);

// float = integer ( frac / ( frac exp ) / exp )
named!(parse_float(Span) -> Span,
       recognize!(
           tuple!(
               parse_integer,
               alt_custom!(e_kind!(nom::ErrorKind::Alt),
                   exp
                 | recognize!(tuple!(frac, opt!(exp)))
               )
           )
       )
);

pub fn float(input: Span) -> nom::IResult<Span, f64> {
    let (rest, s) = try_parse!(input, parse_float);

    match s.fragment.replace("_", "").parse::<f64>() {
        Ok(f) => nom::IResult::Done(rest, f),
        _ => e!(ErrorKind::InvalidNumber, rest),
    }
}
