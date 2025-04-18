use snapbox::prelude::*;
use snapbox::str;

#[track_caller]
fn t(decoded: &str, expected: impl IntoData) {
    let results = StringResults {
        decoded,
        key_default: toml_edit::Key::new(decoded).to_string(),
        string_default: toml_edit::Value::from(decoded).to_string(),
    };
    snapbox::assert_data_eq!(results.to_debug(), expected.raw());
}

#[derive(Debug)]
#[allow(dead_code)]
struct StringResults<'i> {
    decoded: &'i str,
    key_default: String,
    string_default: String,
}

#[test]
fn empty() {
    t(
        "",
        str![[r#"
StringResults {
    decoded: "",
    key_default: "\"\"",
    string_default: "\"\"",
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
    string_default: "\"helloworld\"",
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
    string_default: "\"_hello-world_\"",
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
    string_default: "\"'hello'world'\"",
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
    string_default: "\"''hello''world''\"",
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
    string_default: "\"'''hello'''world'''\"",
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
    string_default: "'\"hello\"world\"'",
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
    string_default: "'\"\"hello\"\"world\"\"'",
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
    string_default: "'\"\"\"hello\"\"\"world\"\"\"'",
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
    string_default: "\"\"\"\"'\"\"\"",
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
    string_default: "'''mixed quoted \\\"start\\\" 'end'' mixed quote'''",
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
    string_default: "'\\windows\\system32\\'",
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
    string_default: "\"\\rhello\\rworld\\r\"",
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
    string_default: "\"\"\"\n\nhello\nworld\n\"\"\"",
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
    string_default: "\"\"\"\n\\r\nhello\\r\nworld\\r\n\"\"\"",
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
    string_default: "\"\\thello\\tworld\\t\"",
}

"#]],
    );
}
