use snapbox::prelude::*;
use snapbox::str;

#[track_caller]
fn t(input: &str, expected: impl IntoData) {
    let actual = input.parse::<toml_datetime::Datetime>();
    snapbox::assert_data_eq!(actual.to_debug(), expected.raw());
}

#[test]
fn only_t() {
    t(
        "T",
        str![[r#"
Err(
    DatetimeParseError {
        what: None,
        expected: Some(
            "year or hour",
        ),
    },
)

"#]],
    );
}

#[test]
fn only_tz() {
    t(
        "TZ",
        str![[r#"
Err(
    DatetimeParseError {
        what: None,
        expected: Some(
            "year or hour",
        ),
    },
)

"#]],
    );
}

#[test]
fn only_tz_dot() {
    t(
        "TZ.",
        str![[r#"
Err(
    DatetimeParseError {
        what: None,
        expected: Some(
            "year or hour",
        ),
    },
)

"#]],
    );
}

#[test]
fn time_without_seconds() {
    t(
        "13:37",
        str![[r#"
Ok(
    Datetime {
        date: None,
        time: Some(
            Time {
                hour: 13,
                minute: 37,
                second: None,
                nanosecond: None,
            },
        ),
        offset: None,
    },
)

"#]],
    );
}

#[test]
fn time_without_seconds_with_nanoseconds() {
    t(
        "13:37.0",
        str![[r#"
Err(
    DatetimeParseError {
        what: None,
        expected: None,
    },
)

"#]],
    );
}

#[test]
fn datetime_without_seconds() {
    t(
        "1979-05-27 07:32Z",
        str![[r#"
Ok(
    Datetime {
        date: Some(
            Date {
                year: 1979,
                month: 5,
                day: 27,
            },
        ),
        time: Some(
            Time {
                hour: 7,
                minute: 32,
                second: None,
                nanosecond: None,
            },
        ),
        offset: Some(
            Z,
        ),
    },
)

"#]],
    );
}
