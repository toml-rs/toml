use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[track_caller]
fn t(toml: &str, expected: impl IntoData) {
    dbg!(toml);
    match toml.parse::<toml_edit::DocumentMut>() {
        Ok(s) => panic!("parsed to: {s:#?}"),
        Err(e) => assert_data_eq!(e.to_string(), expected.raw()),
    }
}

#[test]
fn basic_string_escape() {
    t(
        "a = \"\u{7f}\"",
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = ""
  |      ^
invalid basic string

"#]],
    );
}

#[test]
fn literal_escape() {
    t(
        "a = '\u{7f}'",
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = ''
  |      ^
invalid literal string

"#]],
    );
}

#[test]
fn emoji_error_span() {
    let input = "😀";
    let err = input.parse::<toml_edit::DocumentMut>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, input);
}

#[test]
fn text_error_span() {
    let input = "asdf";
    let err = input.parse::<toml_edit::DocumentMut>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, "");
}

#[test]
fn fuzzed_68144_error_span() {
    let input = "\"\\ᾂr\"";
    let err = input.parse::<toml_edit::DocumentMut>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, "ᾂ");
}
