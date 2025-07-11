#![cfg(feature = "alloc")]

use snapbox::prelude::*;
use snapbox::str;

use toml_writer::ToTomlValue;
use toml_writer::TomlInteger;
use toml_writer::TomlIntegerFormat;
use toml_writer::WriteTomlValue;

#[track_caller]
fn t<N: Copy + core::fmt::Debug + PartialOrd<i32>>(value: N, expected: impl IntoData)
where
    TomlInteger<N>: WriteTomlValue,
{
    let results = IntegerResults {
        value,
        decimal: TomlIntegerFormat::new()
            .as_decimal()
            .format(value)
            .map(|i| i.to_toml_value()),
        hex_upper: TomlIntegerFormat::new()
            .as_hex_upper()
            .format(value)
            .map(|i| i.to_toml_value()),
        hex_lower: TomlIntegerFormat::new()
            .as_hex_lower()
            .format(value)
            .map(|i| i.to_toml_value()),
        octal: TomlIntegerFormat::new()
            .as_octal()
            .format(value)
            .map(|i| i.to_toml_value()),
        binary: TomlIntegerFormat::new()
            .as_binary()
            .format(value)
            .map(|i| i.to_toml_value()),
    };
    snapbox::assert_data_eq!(results.to_debug(), expected.raw());
}

#[derive(Debug)]
#[allow(dead_code)]
struct IntegerResults<N: core::fmt::Debug> {
    value: N,
    decimal: Option<String>,
    hex_upper: Option<String>,
    hex_lower: Option<String>,
    octal: Option<String>,
    binary: Option<String>,
}

#[test]
fn positive() {
    t(
        42,
        str![[r#"
IntegerResults {
    value: 42,
    decimal: Some(
        "42",
    ),
    hex_upper: Some(
        "0x2A",
    ),
    hex_lower: Some(
        "0x2a",
    ),
    octal: Some(
        "0o52",
    ),
    binary: Some(
        "0b101010",
    ),
}

"#]],
    );
}

#[test]
fn negative() {
    t(
        -42,
        str![[r#"
IntegerResults {
    value: -42,
    decimal: Some(
        "-42",
    ),
    hex_upper: None,
    hex_lower: None,
    octal: None,
    binary: None,
}

"#]],
    );
}
