#![cfg(feature = "alloc")]

use snapbox::prelude::*;
use snapbox::str;

use toml_datetime::*;

#[track_caller]
fn t<T>(input: T, expected: impl IntoData)
where
    T: std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let actual = input.to_string();
    snapbox::assert_data_eq!(actual.to_debug(), expected.raw());
    let _ = actual.parse::<T>().unwrap();
}

#[test]
fn time_without_seconds() {
    t(
        Datetime {
            date: None,
            time: Some(Time {
                hour: 13,
                minute: 37,
                second: None,
                nanosecond: None,
            }),
            offset: None,
        },
        str![[r#"
"13:37"

"#]],
    );
}

#[test]
fn time_without_seconds_with_nanoseconds() {
    t(
        Datetime {
            date: None,
            time: Some(Time {
                hour: 13,
                minute: 37,
                second: None,
                nanosecond: Some(5),
            }),
            offset: None,
        },
        str![[r#"
"13:37:00.000000005"

"#]],
    );
}

#[test]
fn time_with_zero_seconds_and_nanoseconds() {
    t(
        Datetime {
            date: None,
            time: Some(Time {
                hour: 13,
                minute: 37,
                second: Some(0),
                nanosecond: Some(0),
            }),
            offset: None,
        },
        str![[r#"
"13:37:00.0"

"#]],
    );
}

#[test]
fn datetime_without_seconds() {
    t(
        Datetime {
            date: Some(Date {
                year: 1979,
                month: 5,
                day: 27,
            }),
            time: Some(Time {
                hour: 7,
                minute: 32,
                second: None,
                nanosecond: None,
            }),
            offset: Some(Offset::Z),
        },
        str![[r#"
"1979-05-27T07:32Z"

"#]],
    );
}
