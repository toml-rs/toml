use crate::parser::errors::CustomError;
use crate::value;
use chrono::TimeZone;
use combine::parser::char::{char, digit};
use combine::parser::range::{recognize, take};
use combine::stream::RangeStream;
use combine::*;

// ;; Date and Time (as defined in RFC 3339)

// date-time = offset-date-time / local-date-time / local-date / local-time
// offset-date-time = full-date "T" full-time
// local-date-time = full-date "T" partial-time
// local-date = full-date
// local-time = partial-time
// full-time = partial-time time-offset
parse!(date_time() -> value::DateTime, {
    choice!(
        (
            full_date(),
            optional((
                char('T'),
                partial_time(),
                optional(time_offset()),
            ))
        )
            .map(|(d, opt)| {
                match opt {
                    // Offset Date-Time
                    Some((_, t, Some(o))) => {
                        let dt = chrono::NaiveDateTime::new(d, t);
                        value::DateTime::OffsetDateTime(
                            o.from_local_datetime(&dt).unwrap()
                        )
                    }
                    // Local Date-Time
                    Some((_, t, None)) => {
                        value::DateTime::LocalDateTime(
                            chrono::NaiveDateTime::new(d, t)
                        )
                    }
                    // Local Date
                    None => {
                        value::DateTime::LocalDate(d)
                    }
                }

            }),
        // Local Time
        partial_time()
            .message("While parsing a Time")
            .map(value::DateTime::LocalTime)
    )
        .message("While parsing a Date-Time")
});

// full-date      = date-fullyear "-" date-month "-" date-mday
parse!(full_date() -> chrono::NaiveDate, {
    (
        attempt((date_fullyear(), char('-'))),
        date_month(),
        char('-'),
        date_mday(),
    ).and_then(|((year, _), month, _, day)| {
        chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| CustomError::DateOutOfRange { year, month, day })
    })
});

// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
// time-secfrac   = "." 1*DIGIT
parse!(partial_time() -> chrono::NaiveTime, {
    recognize((
        attempt((
            time_hour(),
            char(':'),
        )),
        time_minute(),
        char(':'),
        time_second(),
        optional(attempt(char('.')).and(skip_many1(digit()))),
    )).and_then(|s: &str| s.parse::<chrono::NaiveTime>())
});

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
parse!(time_offset() -> chrono::FixedOffset, {
    attempt(char('Z')).map(|_| chrono::FixedOffset::east(0))
        .or(
            (
                attempt(choice([char('+'), char('-')])),
                time_hour(),
                char(':'),
                time_minute(),
            ).and_then(|(sign, hour, _, minute)| {
                const SEC: i32 = 1;
                const MIN: i32 = 60 * SEC;
                const HOUR: i32 = 60 * MIN;
                let secs = (hour as i32) * HOUR + (minute as i32) * MIN;
                match sign {
                    '+' => chrono::FixedOffset::east_opt(secs),
                    '-' => chrono::FixedOffset::west_opt(secs),
                    _ => unreachable!("Parser prevents this"),
                }.ok_or_else(||CustomError::OffsetOutOfRange { sign, hour, minute})
            })
        ).message("While parsing a Time Offset")
});

// date-fullyear  = 4DIGIT
parse!(date_fullyear() -> i32, {
    signed_digits(4)
});

// date-month     = 2DIGIT  ; 01-12
parse!(date_month() -> u32, {
    unsigned_digits(2)
});

// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
parse!(date_mday() -> u32, {
    unsigned_digits(2)
});

// time-hour      = 2DIGIT  ; 00-23
parse!(time_hour() -> u32, {
    unsigned_digits(2)
});

// time-minute    = 2DIGIT  ; 00-59
parse!(time_minute() -> u32, {
    unsigned_digits(2)
});

// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
parse!(time_second() -> u32, {
    unsigned_digits(2)
});

parse!(signed_digits(count: usize) -> i32, {
    take(*count).and_then(|s: &str| s.parse::<i32>())
});

parse!(unsigned_digits(count: usize) -> u32, {
    take(*count).and_then(|s: &str| s.parse::<u32>())
});
