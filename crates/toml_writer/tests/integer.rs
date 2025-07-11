use toml_writer::ToTomlValue;
use toml_writer::TomlIntegerFormat;

#[test]
fn positive() {
    assert_eq!(
        TomlIntegerFormat::new()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("42"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_decimal()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("42"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_hex_upper()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("0x2A"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_hex_lower()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("0x2a"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_octal()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("0o52"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_binary()
            .format(42)
            .map(|i| i.to_toml_value()),
        Some(String::from("0b101010"))
    );
}

#[test]
fn negative() {
    assert_eq!(
        TomlIntegerFormat::new()
            .format(-42)
            .map(|i| i.to_toml_value()),
        Some(String::from("-42"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_decimal()
            .format(-42)
            .map(|i| i.to_toml_value()),
        Some(String::from("-42"))
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_hex_upper()
            .format(-42)
            .map(|i| i.to_toml_value()),
        None
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_hex_lower()
            .format(-42)
            .map(|i| i.to_toml_value()),
        None
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_octal()
            .format(-42)
            .map(|i| i.to_toml_value()),
        None
    );
    assert_eq!(
        TomlIntegerFormat::new()
            .as_binary()
            .format(-42)
            .map(|i| i.to_toml_value()),
        None
    );
}
