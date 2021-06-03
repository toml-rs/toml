use crate::value;
use combine::parser::char::{char, digit};
use combine::parser::range::{recognize, recognize_with_value};
use combine::parser::repeat::{skip_count_min_max, SkipCountMinMax};
use combine::stream::RangeStream;
use combine::*;

#[inline]
pub fn repeat<I: Stream, P: Parser<I>>(count: usize, parser: P) -> SkipCountMinMax<I, P> {
    skip_count_min_max(count, count, parser)
}

// ;; Date and Time (as defined in RFC 3339)

// date-time = offset-date-time / local-date-time / local-date / local-time
// offset-date-time = full-date "T" full-time
// local-date-time = full-date "T" partial-time
// local-date = full-date
// local-time = partial-time
// full-time = partial-time time-offset
parse!(date_time() -> value::DateTime, {
    choice!(
        recognize_with_value((
            full_date(),
            optional((
                char('T'),
                partial_time(),
                optional(time_offset()),
            ))
        ))
            .and_then(|(s, (_, opt))| {
                match opt {
                    // Offset Date-Time
                    Some((_, _, Some(_))) => {
                        chrono::DateTime::parse_from_rfc3339(s)
                            .map(value::DateTime::OffsetDateTime)
                    }
                    // Local Date-Time
                    Some(_) => {
                        s.parse::<chrono::NaiveDateTime>()
                            .map(value::DateTime::LocalDateTime)
                    }
                    // Local Date
                    None => {
                        s.parse::<chrono::NaiveDate>()
                            .map(value::DateTime::LocalDate)
                    }
                }

            }),
        // Local Time
        recognize(partial_time())
            .and_then(str::parse)
            .message("While parsing a Time")
            .map(value::DateTime::LocalTime)
    )
        .message("While parsing a Date-Time")
});

// full-date      = date-fullyear "-" date-month "-" date-mday
// date-fullyear  = 4DIGIT
// date-month     = 2DIGIT  ; 01-12
// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
parse!(full_date() -> &'a str, {
    recognize((
        attempt((repeat(4, digit()), char('-'))),
        repeat(2, digit()),
        char('-'),
        repeat(2, digit()),
    ))
});

// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
// time-hour      = 2DIGIT  ; 00-23
// time-minute    = 2DIGIT  ; 00-59
// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
// time-secfrac   = "." 1*DIGIT
parse!(partial_time() -> (), {
    (
        attempt((
            repeat(2, digit()),
            char(':'),
        )),
        repeat(2, digit()),
        char(':'),
        repeat(2, digit()),
        optional(attempt(char('.')).and(skip_many1(digit()))),
    ).map(|_| ())
});

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
parse!(time_offset() -> (), {
    attempt(char('Z')).map(|_| ())
        .or(
            (
                attempt(choice([char('+'), char('-')])),
                repeat(2, digit()),
                char(':'),
                repeat(2, digit()),
            ).map(|_| ())
        ).message("While parsing a Time Offset")
});
