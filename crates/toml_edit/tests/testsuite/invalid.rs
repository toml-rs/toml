use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[test]
fn basic_string_escape() {
    let toml_input = "a = \"\u{7f}\"";
    let err = toml_input.parse::<toml_edit::DocumentMut>().unwrap_err();
    assert_data_eq!(
        err.to_string(),
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = ""
  |      ^
invalid basic string

"#]]
        .raw()
    );
}

#[test]
fn literal_escape() {
    let toml_input = "a = '\u{7f}'";
    let err = toml_input.parse::<toml_edit::DocumentMut>().unwrap_err();
    assert_data_eq!(
        err.to_string(),
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = ''
  |      ^
invalid literal string

"#]]
        .raw()
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
