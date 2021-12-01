use crate::datetime::*;
use crate::parser::errors::CustomError;
use crate::parser::trivia::from_utf8_unchecked;
use combine::parser::byte::byte;
use combine::parser::range::{recognize, take_while1};
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
                attempt((
                    satisfy(is_time_delim),
                    look_ahead(time_hour())
                )),
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
        attempt((date_fullyear(), byte(b'-'))),
        date_month(),
        byte(b'-'),
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
            byte(b':'),
        )),
        time_minute(),
        byte(b':'),
        time_second(),
        optional(attempt(time_secfrac())),
    ).map(|((hour, _), minute, _, second, nanosecond)| {
        Time { hour, minute, second, nanosecond: nanosecond.unwrap_or_default() }
    })
});

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
parse!(time_offset() -> Offset, {
    attempt(satisfy(|c| c == b'Z' || c == b'z')).map(|_| Offset::Z)
        .or(
            (
                attempt(choice([byte(b'+'), byte(b'-')])),
                time_hour(),
                byte(b':'),
                time_minute(),
            ).map(|(sign, hours, _, minutes)| {
                let hours = hours as i8;
                let hours = match sign {
                    b'+' => hours,
                    b'-' => -hours,
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
fn is_time_delim(c: u8) -> bool {
    matches!(c, b'T' | b't' | b' ')
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
    static SCALE: [u32; 10] =
        [0, 100_000_000, 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100, 10, 1];
    byte(b'.').and(take_while1(|c: u8| c.is_ascii_digit())).and_then::<_, _, CustomError>(|(_, repr): (u8, &[u8])| {
        let mut repr = unsafe { from_utf8_unchecked(repr, "`is_ascii_digit` filters out on-ASCII") };
        let max_digits = SCALE.len() - 1;
        if max_digits < repr.len() {
            // Millisecond precision is required. Further precision of fractional seconds is
            // implementation-specific. If the value contains greater precision than the
            // implementation can support, the additional precision must be truncated, not rounded.
            repr = &repr[0..max_digits];
        }

        let v = repr.parse::<u32>().map_err(|_| CustomError::OutOfRange)?;
        let num_digits = repr.len();

        // scale the number accordingly.
        let scale = SCALE.get(num_digits).ok_or(CustomError::OutOfRange)?;
        let v = v.checked_mul(*scale).ok_or(CustomError::OutOfRange)?;
        Ok(v)
    })
});

parse!(signed_digits(count: usize) -> i32, {
    recognize(skip_count_min_max(
        *count, *count,
        satisfy(|c: u8| c.is_ascii_digit()),
    )).and_then(|b: &[u8]| {
        let s = unsafe { from_utf8_unchecked(b, "`is_ascii_digit` filters out on-ASCII") };
        s.parse::<i32>()
    })
});

parse!(unsigned_digits(count: usize) -> u32, {
    recognize(skip_count_min_max(
        *count, *count,
        satisfy(|c: u8| c.is_ascii_digit()),
    )).and_then(|b: &[u8]| {
        let s = unsafe { from_utf8_unchecked(b, "`is_ascii_digit` filters out on-ASCII") };
        s.parse::<u32>()
    })
});
