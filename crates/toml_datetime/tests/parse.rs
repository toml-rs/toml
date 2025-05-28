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
