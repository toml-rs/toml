use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[track_caller]
fn t(toml: &str, expected: impl IntoData) {
    dbg!(toml);
    match toml.parse::<crate::RustDocument>() {
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
fn stray_cr() {
    t(
        "\r",
        str![[r#"
TOML parse error at line 1, column 1
  |
1 | 
  | ^


"#]],
    );
    t(
        "a = [ \r ]",
        str![[r#"
TOML parse error at line 1, column 8
  |
1 | a = [ 
 ]
  |        ^


"#]],
    );
    t(
        "a = \"\"\"\r\"\"\"",
        str![[r#"
TOML parse error at line 1, column 8
  |
1 | a = """
"""
  |        ^
invalid multiline basic string

"#]],
    );
    t(
        "a = \"\"\"\\  \r  \"\"\"",
        str![[r#"
TOML parse error at line 1, column 10
  |
1 | a = """\  
  """
  |          ^
invalid escape sequence
expected `b`, `f`, `n`, `r`, `t`, `u`, `U`, `\`, `"`

"#]],
    );
    t(
        "a = '''\r'''",
        str![[r#"
TOML parse error at line 1, column 8
  |
1 | a = '''
'''
  |        ^
invalid multiline literal string

"#]],
    );
    t(
        "a = '\r'",
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = '
'
  |      ^
invalid literal string

"#]],
    );
    t(
        "a = \"\r\"",
        str![[r#"
TOML parse error at line 1, column 6
  |
1 | a = "
"
  |      ^
invalid basic string

"#]],
    );
}

#[test]
fn duplicate_key_with_crlf() {
    t(
        "\r\n\
         [t1]\r\n\
         [t2]\r\n\
         a = 1\r\n\
         a = 2\r\n\
         ",
        str![[r#"
TOML parse error at line 5, column 1
  |
5 | a = 2
  | ^
duplicate key `a` in table `t2`

"#]],
    );
}

#[test]
fn emoji_error_span() {
    let input = "😀";
    let err = input.parse::<crate::RustDocument>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, input);
}

#[test]
fn text_error_span() {
    let input = "asdf";
    let err = input.parse::<crate::RustDocument>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, "");
}

#[test]
fn fuzzed_68144_error_span() {
    let input = "\"\\ᾂr\"";
    let err = input.parse::<crate::RustDocument>().unwrap_err();
    dbg!(err.span());
    let actual = &input[err.span().unwrap()];
    assert_eq!(actual, "ᾂ");
}
