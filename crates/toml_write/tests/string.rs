#![cfg(feature = "alloc")]
#![allow(clippy::dbg_macro)] // unsure why config isn't working

use proptest::prelude::*;
use snapbox::prelude::*;
use snapbox::str;

use toml_write::ToTomlKey;
use toml_write::ToTomlValue;
use toml_write::TomlKeyBuilder;
use toml_write::TomlStringBuilder;

#[track_caller]
fn t(decoded: &str, expected: impl IntoData) {
    let key = TomlKeyBuilder::new(decoded);
    let string = TomlStringBuilder::new(decoded);
    dbg!(&key);
    dbg!(&string);
    let results = StringResults {
        decoded,
        key_default: key.as_default().to_toml_key(),
        key_unquoted: key.as_unquoted().map(|k| k.to_toml_key()),
        key_literal: key.as_literal().map(|k| k.to_toml_key()),
        key_basic_pretty: key.as_basic_pretty().map(|k| k.to_toml_key()),
        key_basic: key.as_basic().to_toml_key(),
        string_default: string.as_default().to_toml_value(),
        string_literal: string.as_literal().map(|k| k.to_toml_value()),
        string_ml_literal: string.as_ml_literal().map(|k| k.to_toml_value()),
        string_basic_pretty: string.as_basic_pretty().map(|k| k.to_toml_value()),
        string_ml_basic_pretty: string.as_ml_basic_pretty().map(|k| k.to_toml_value()),
        string_basic: string.as_basic().to_toml_value(),
        string_ml_basic: string.as_ml_basic().to_toml_value(),
    };
    snapbox::assert_data_eq!(results.to_debug(), expected.raw());

    // Verify defaults are compatible with the old TOML parser so new Cargo doesn't cause an MSRV
    // bump
    let toml = format!("{} = {}", results.key_default, results.string_default);
    dbg!(&toml);
    let value = toml.parse::<toml_old::Value>();
    let value = match value {
        Ok(value) => value,
        Err(err) => panic!("could not parse: {err}"),
    };
    let table = value.as_table().unwrap();
    let (key, value) = table.iter().next().unwrap();
    assert_eq!(key, decoded);
    assert_eq!(value.as_str().unwrap(), decoded);
}

#[derive(Debug)]
#[allow(dead_code)]
struct StringResults<'i> {
    decoded: &'i str,
    key_default: String,
    key_unquoted: Option<String>,
    key_literal: Option<String>,
    key_basic_pretty: Option<String>,
    key_basic: String,
    string_default: String,
    string_literal: Option<String>,
    string_ml_literal: Option<String>,
    string_basic_pretty: Option<String>,
    string_ml_basic_pretty: Option<String>,
    string_basic: String,
    string_ml_basic: String,
}

#[test]
fn empty() {
    t(
        "",
        str![[r#"
StringResults {
    decoded: "",
    key_default: "\"\"",
    key_unquoted: None,
    key_literal: Some(
        "''",
    ),
    key_basic_pretty: Some(
        "\"\"",
    ),
    key_basic: "\"\"",
    string_default: "\"\"",
    string_literal: Some(
        "''",
    ),
    string_ml_literal: Some(
        "''''''",
    ),
    string_basic_pretty: Some(
        "\"\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"\"\"\"",
    ),
    string_basic: "\"\"",
    string_ml_basic: "\"\"\"\"\"\"",
}

"#]],
    );
}

#[test]
fn alpha() {
    t(
        "helloworld",
        str![[r#"
StringResults {
    decoded: "helloworld",
    key_default: "helloworld",
    key_unquoted: Some(
        "helloworld",
    ),
    key_literal: Some(
        "'helloworld'",
    ),
    key_basic_pretty: Some(
        "\"helloworld\"",
    ),
    key_basic: "\"helloworld\"",
    string_default: "\"helloworld\"",
    string_literal: Some(
        "'helloworld'",
    ),
    string_ml_literal: Some(
        "'''helloworld'''",
    ),
    string_basic_pretty: Some(
        "\"helloworld\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"helloworld\"\"\"",
    ),
    string_basic: "\"helloworld\"",
    string_ml_basic: "\"\"\"helloworld\"\"\"",
}

"#]],
    );
}

#[test]
fn ident() {
    t(
        "_hello-world_",
        str![[r#"
StringResults {
    decoded: "_hello-world_",
    key_default: "_hello-world_",
    key_unquoted: Some(
        "_hello-world_",
    ),
    key_literal: Some(
        "'_hello-world_'",
    ),
    key_basic_pretty: Some(
        "\"_hello-world_\"",
    ),
    key_basic: "\"_hello-world_\"",
    string_default: "\"_hello-world_\"",
    string_literal: Some(
        "'_hello-world_'",
    ),
    string_ml_literal: Some(
        "'''_hello-world_'''",
    ),
    string_basic_pretty: Some(
        "\"_hello-world_\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"_hello-world_\"\"\"",
    ),
    string_basic: "\"_hello-world_\"",
    string_ml_basic: "\"\"\"_hello-world_\"\"\"",
}

"#]],
    );
}

#[test]
fn one_single_quote() {
    t(
        "'hello'world'",
        str![[r#"
StringResults {
    decoded: "'hello'world'",
    key_default: "\"'hello'world'\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: Some(
        "\"'hello'world'\"",
    ),
    key_basic: "\"'hello'world'\"",
    string_default: "\"'hello'world'\"",
    string_literal: None,
    string_ml_literal: Some(
        "''''hello'world''''",
    ),
    string_basic_pretty: Some(
        "\"'hello'world'\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"'hello'world'\"\"\"",
    ),
    string_basic: "\"'hello'world'\"",
    string_ml_basic: "\"\"\"'hello'world'\"\"\"",
}

"#]],
    );
}

#[test]
fn two_single_quote() {
    t(
        "''hello''world''",
        str![[r#"
StringResults {
    decoded: "''hello''world''",
    key_default: "\"''hello''world''\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: Some(
        "\"''hello''world''\"",
    ),
    key_basic: "\"''hello''world''\"",
    string_default: "\"''hello''world''\"",
    string_literal: None,
    string_ml_literal: Some(
        "'''''hello''world'''''",
    ),
    string_basic_pretty: Some(
        "\"''hello''world''\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"''hello''world''\"\"\"",
    ),
    string_basic: "\"''hello''world''\"",
    string_ml_basic: "\"\"\"''hello''world''\"\"\"",
}

"#]],
    );
}

#[test]
fn three_single_quote() {
    t(
        "'''hello'''world'''",
        str![[r#"
StringResults {
    decoded: "'''hello'''world'''",
    key_default: "\"'''hello'''world'''\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: Some(
        "\"'''hello'''world'''\"",
    ),
    key_basic: "\"'''hello'''world'''\"",
    string_default: "\"'''hello'''world'''\"",
    string_literal: None,
    string_ml_literal: None,
    string_basic_pretty: Some(
        "\"'''hello'''world'''\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"'''hello'''world'''\"\"\"",
    ),
    string_basic: "\"'''hello'''world'''\"",
    string_ml_basic: "\"\"\"'''hello'''world'''\"\"\"",
}

"#]],
    );
}

#[test]
fn one_double_quote() {
    t(
        r#""hello"world""#,
        str![[r#"
StringResults {
    decoded: "\"hello\"world\"",
    key_default: "'\"hello\"world\"'",
    key_unquoted: None,
    key_literal: Some(
        "'\"hello\"world\"'",
    ),
    key_basic_pretty: None,
    key_basic: "\"\\\"hello\\\"world\\\"\"",
    string_default: "'\"hello\"world\"'",
    string_literal: Some(
        "'\"hello\"world\"'",
    ),
    string_ml_literal: Some(
        "'''\"hello\"world\"'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: Some(
        "\"\"\"\"hello\"world\"\"\"\"",
    ),
    string_basic: "\"\\\"hello\\\"world\\\"\"",
    string_ml_basic: "\"\"\"\"hello\"world\"\"\"\"",
}

"#]],
    );
}

#[test]
fn two_double_quote() {
    t(
        r#"""hello""world"""#,
        str![[r#"
StringResults {
    decoded: "\"\"hello\"\"world\"\"",
    key_default: "'\"\"hello\"\"world\"\"'",
    key_unquoted: None,
    key_literal: Some(
        "'\"\"hello\"\"world\"\"'",
    ),
    key_basic_pretty: None,
    key_basic: "\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\"",
    string_default: "'\"\"hello\"\"world\"\"'",
    string_literal: Some(
        "'\"\"hello\"\"world\"\"'",
    ),
    string_ml_literal: Some(
        "'''\"\"hello\"\"world\"\"'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: Some(
        "\"\"\"\"\"hello\"\"world\"\"\"\"\"",
    ),
    string_basic: "\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\"",
    string_ml_basic: "\"\"\"\"\"hello\"\"world\"\"\"\"\"",
}

"#]],
    );
}

#[test]
fn three_double_quote() {
    t(
        r#""""hello"""world""""#,
        str![[r#"
StringResults {
    decoded: "\"\"\"hello\"\"\"world\"\"\"",
    key_default: "'\"\"\"hello\"\"\"world\"\"\"'",
    key_unquoted: None,
    key_literal: Some(
        "'\"\"\"hello\"\"\"world\"\"\"'",
    ),
    key_basic_pretty: None,
    key_basic: "\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\"",
    string_default: "'\"\"\"hello\"\"\"world\"\"\"'",
    string_literal: Some(
        "'\"\"\"hello\"\"\"world\"\"\"'",
    ),
    string_ml_literal: Some(
        "'''\"\"\"hello\"\"\"world\"\"\"'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: None,
    string_basic: "\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\"",
    string_ml_basic: "\"\"\"\"\"\\\"hello\"\"\\\"world\"\"\\\"\"\"\"",
}

"#]],
    );
}

#[test]
fn mixed_quote_1() {
    t(
        r#""'"#,
        str![[r#"
StringResults {
    decoded: "\"'",
    key_default: "\"\\\"'\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: None,
    key_basic: "\"\\\"'\"",
    string_default: "\"\"\"\"'\"\"\"",
    string_literal: None,
    string_ml_literal: Some(
        "'''\"''''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: Some(
        "\"\"\"\"'\"\"\"",
    ),
    string_basic: "\"\\\"'\"",
    string_ml_basic: "\"\"\"\"'\"\"\"",
}

"#]],
    );
}

#[test]
fn mixed_quote_2() {
    t(
        r#"mixed quoted \"start\" 'end'' mixed quote"#,
        str![[r#"
StringResults {
    decoded: "mixed quoted \\\"start\\\" 'end'' mixed quote",
    key_default: "\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: None,
    key_basic: "\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\"",
    string_default: "'''mixed quoted \\\"start\\\" 'end'' mixed quote'''",
    string_literal: None,
    string_ml_literal: Some(
        "'''mixed quoted \\\"start\\\" 'end'' mixed quote'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: None,
    string_basic: "\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\"",
    string_ml_basic: "\"\"\"mixed quoted \\\\\"start\\\\\" 'end'' mixed quote\"\"\"",
}

"#]],
    );
}

#[test]
fn escape() {
    t(
        r#"\windows\system32\"#,
        str![[r#"
StringResults {
    decoded: "\\windows\\system32\\",
    key_default: "'\\windows\\system32\\'",
    key_unquoted: None,
    key_literal: Some(
        "'\\windows\\system32\\'",
    ),
    key_basic_pretty: None,
    key_basic: "\"\\\\windows\\\\system32\\\\\"",
    string_default: "'\\windows\\system32\\'",
    string_literal: Some(
        "'\\windows\\system32\\'",
    ),
    string_ml_literal: Some(
        "'''\\windows\\system32\\'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: None,
    string_basic: "\"\\\\windows\\\\system32\\\\\"",
    string_ml_basic: "\"\"\"\\\\windows\\\\system32\\\\\"\"\"",
}

"#]],
    );
}

#[test]
fn cr() {
    t(
        "\rhello\rworld\r",
        str![[r#"
StringResults {
    decoded: "\rhello\rworld\r",
    key_default: "\"\\rhello\\rworld\\r\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: None,
    key_basic: "\"\\rhello\\rworld\\r\"",
    string_default: "\"\\rhello\\rworld\\r\"",
    string_literal: None,
    string_ml_literal: None,
    string_basic_pretty: None,
    string_ml_basic_pretty: None,
    string_basic: "\"\\rhello\\rworld\\r\"",
    string_ml_basic: "\"\"\"\\rhello\\rworld\\r\"\"\"",
}

"#]],
    );
}

#[test]
fn lf() {
    t(
        "\nhello\nworld\n",
        str![[r#"
StringResults {
    decoded: "\nhello\nworld\n",
    key_default: "\"\\nhello\\nworld\\n\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: None,
    key_basic: "\"\\nhello\\nworld\\n\"",
    string_default: "\"\"\"\n\nhello\nworld\n\"\"\"",
    string_literal: None,
    string_ml_literal: Some(
        "'''\n\nhello\nworld\n'''",
    ),
    string_basic_pretty: None,
    string_ml_basic_pretty: Some(
        "\"\"\"\n\nhello\nworld\n\"\"\"",
    ),
    string_basic: "\"\\nhello\\nworld\\n\"",
    string_ml_basic: "\"\"\"\n\nhello\nworld\n\"\"\"",
}

"#]],
    );
}

#[test]
fn crlf() {
    t(
        "\r\nhello\r\nworld\r\n",
        str![[r#"
StringResults {
    decoded: "\r\nhello\r\nworld\r\n",
    key_default: "\"\\r\\nhello\\r\\nworld\\r\\n\"",
    key_unquoted: None,
    key_literal: None,
    key_basic_pretty: None,
    key_basic: "\"\\r\\nhello\\r\\nworld\\r\\n\"",
    string_default: "\"\"\"\n\\r\nhello\\r\nworld\\r\n\"\"\"",
    string_literal: None,
    string_ml_literal: None,
    string_basic_pretty: None,
    string_ml_basic_pretty: None,
    string_basic: "\"\\r\\nhello\\r\\nworld\\r\\n\"",
    string_ml_basic: "\"\"\"\n\\r\nhello\\r\nworld\\r\n\"\"\"",
}

"#]],
    );
}

#[test]
fn tab() {
    t(
        "\thello\tworld\t",
        str![[r#"
StringResults {
    decoded: "\thello\tworld\t",
    key_default: "\"\\thello\\tworld\\t\"",
    key_unquoted: None,
    key_literal: Some(
        "'\thello\tworld\t'",
    ),
    key_basic_pretty: Some(
        "\"\\thello\\tworld\\t\"",
    ),
    key_basic: "\"\\thello\\tworld\\t\"",
    string_default: "\"\\thello\\tworld\\t\"",
    string_literal: Some(
        "'\thello\tworld\t'",
    ),
    string_ml_literal: Some(
        "'''\thello\tworld\t'''",
    ),
    string_basic_pretty: Some(
        "\"\\thello\\tworld\\t\"",
    ),
    string_ml_basic_pretty: Some(
        "\"\"\"\\thello\\tworld\\t\"\"\"",
    ),
    string_basic: "\"\\thello\\tworld\\t\"",
    string_ml_basic: "\"\"\"\\thello\\tworld\\t\"\"\"",
}

"#]],
    );
}

proptest! {
    /// Verify defaults are compatible with the old TOML parser so new Cargo doesn't cause an MSRV
    /// bump
    #[test]
    fn parseable(decoded in "\\PC*") {
        let key = TomlKeyBuilder::new(&decoded);
        let string = TomlStringBuilder::new(&decoded);

        let key_default = key.as_default().to_toml_key();
        let string_default = string.as_default().to_toml_value();

        let toml = format!("{key_default} = {string_default}");
        dbg!(&toml);
        let value = toml.parse::<toml_old::Value>();
        let value = match value {
            Ok(value) => value,
            Err(err) => panic!("could not parse: {err}"),
        };
        let table = value.as_table().unwrap();
        let (key, value) = table.iter().next().unwrap();
        assert_eq!(*key, decoded);
        assert_eq!(value.as_str().unwrap(), decoded);
    }
}
