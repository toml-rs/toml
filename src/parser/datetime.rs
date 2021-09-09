use crate::datetime::*;
use crate::parser::errors::CustomError;
use combine::parser::char::char;
use combine::parser::range::{take, take_while1};
use combine::stream::RangeStream;
use combine::*;

// ;; Date and Time (as defined in RFC 3339)

// date-time = offset-date-time / local-date-time / local-date / local-time
// offset-date-time = full-date "T" full-time
// local-date-time = full-date "T" partial-time
// local-date = full-date
// local-time = partial-time
// full-time = partial-time time-offset
parse!(date_time() -> Datetime, {
    choice!(
        (
            full_date(),
            optional((
                satisfy(is_time_delim),
                partial_time(),
                optional(time_offset()),
            ))
        )
            .map(|(date, opt)| {
                match opt {
                    // Offset Date-Time
                    Some((_, time, offset)) => {
                        Datetime { date: Some(date), time: Some(time), offset }
                    }
                    // Local Date
                    None => {
                        Datetime { date: Some(date), time: None, offset: None}
                    },
                }
            }),
        // Local Time
        partial_time()
            .message("While parsing a Time")
            .map(|t| {
                t.into()
            })
    )
        .message("While parsing a Date-Time")
});

// full-date      = date-fullyear "-" date-month "-" date-mday
parse!(full_date() -> Date, {
    (
        attempt((date_fullyear(), char('-'))),
        date_month(),
        char('-'),
        date_mday(),
    ).map(|((year, _), month, _, day)| {
        Date { year, month, day }
    })
});

// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
parse!(partial_time() -> Time, {
    (
        attempt((
            time_hour(),
            char(':'),
        )),
        time_minute(),
        char(':'),
        time_second(),
        optional(attempt(time_secfrac())),
    ).map(|((hour, _), minute, _, second, nanosecond)| {
        Time { hour, minute, second, nanosecond: nanosecond.unwrap_or_default() }
    })
});

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
parse!(time_offset() -> Offset, {
    attempt(satisfy(|c| c == 'Z' || c == 'z')).map(|_| Offset::Z)
        .or(
            (
                attempt(choice([char('+'), char('-')])),
                time_hour(),
                char(':'),
                time_minute(),
            ).map(|(sign, hours, _, minutes)| {
                let hours = hours as i8;
                let hours = match sign {
                    '+' => hours,
                    '-' => -hours,
                    _ => unreachable!("Parser prevents this"),
                };
                Offset::Custom { hours, minutes }
            })
        ).message("While parsing a Time Offset")
});

// date-fullyear  = 4DIGIT
parse!(date_fullyear() -> u16, {
    signed_digits(4).map(|d| d as u16)
});

// date-month     = 2DIGIT  ; 01-12
parse!(date_month() -> u8, {
    unsigned_digits(2).map(|d| d as u8).and_then(|v| {
        if (1..=12).contains(&v) {
            Ok(v)
        } else {
            Err(CustomError::OutOfRange)
        }
    })
});

// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
parse!(date_mday() -> u8, {
    unsigned_digits(2).map(|d| d as u8).and_then(|v| {
        if (1..=31).contains(&v) {
            Ok(v)
        } else {
            Err(CustomError::OutOfRange)
        }
    })
});

// time-delim     = "T" / %x20 ; T, t, or space
fn is_time_delim(c: char) -> bool {
    matches!(c, 'T' | 't' | ' ')
}

// time-hour      = 2DIGIT  ; 00-23
parse!(time_hour() -> u8, {
    unsigned_digits(2).map(|d| d as u8).and_then(|v| {
        if (0..=23).contains(&v) {
            Ok(v)
        } else {
            Err(CustomError::OutOfRange)
        }
    })
});

// time-minute    = 2DIGIT  ; 00-59
parse!(time_minute() -> u8, {
    unsigned_digits(2).map(|d| d as u8).and_then(|v| {
        if (0..=59).contains(&v) {
            Ok(v)
        } else {
            Err(CustomError::OutOfRange)
        }
    })
});

// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
parse!(time_second() -> u8, {
    unsigned_digits(2).map(|d| d as u8).and_then(|v| {
        if (0..=60).contains(&v) {
            Ok(v)
        } else {
            Err(CustomError::OutOfRange)
        }
    })
});

// time-secfrac   = "." 1*DIGIT
parse!(time_secfrac() -> u32, {
    char('.').and(take_while1(|c: char| c.is_digit(10))).and_then::<_, _, CustomError>(|(_, repr): (char, &str)| {
        let v = repr.parse::<u32>().map_err(|_| CustomError::OutOfRange)?;
        let consumed = repr.len();

        // scale the number accordingly.
        static SCALE: [u32; 10] =
            [0, 100_000_000, 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100, 10, 1];
        let scale = SCALE.get(consumed).ok_or(CustomError::OutOfRange)?;
        let v = v.checked_mul(*scale).ok_or(CustomError::OutOfRange)?;
        Ok(v)
    })
});

parse!(signed_digits(count: usize) -> i32, {
    take(*count).and_then(|s: &str| s.parse::<i32>())
});

parse!(unsigned_digits(count: usize) -> u32, {
    take(*count).and_then(|s: &str| s.parse::<u32>())
});
