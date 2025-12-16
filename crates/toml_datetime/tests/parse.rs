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
Err(
    DatetimeParseError {
        what: Some(
            "time",
        ),
        expected: Some(
            "`:` (MM:SS)",
        ),
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
        what: Some(
            "time",
        ),
        expected: Some(
            "`:` (MM:SS)",
        ),
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
Err(
    DatetimeParseError {
        what: Some(
            "time",
        ),
        expected: Some(
            "`:` (MM:SS)",
        ),
    },
)

"#]],
    );
}
