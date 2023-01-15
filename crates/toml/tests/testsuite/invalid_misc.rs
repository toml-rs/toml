macro_rules! bad {
    ($toml:expr, $msg:expr) => {
        match $toml.parse::<toml::Value>() {
            Ok(s) => panic!("parsed to: {:#?}", s),
            Err(e) => snapbox::assert_eq($msg, e.to_string()),
        }
    };
}

#[test]
fn bad() {
    bad!(
        "a = 01",
        "\
TOML parse error at line 1, column 6
  |
1 | a = 01
  |      ^
expected newline, `#`
"
    );
    bad!(
        "a = 1__1",
        "\
TOML parse error at line 1, column 7
  |
1 | a = 1__1
  |       ^
invalid integer
expected digit
"
    );
    bad!(
        "a = 1_",
        "\
TOML parse error at line 1, column 7
  |
1 | a = 1_
  |       ^
invalid integer
expected digit
"
    );
    bad!(
        "''",
        "\
TOML parse error at line 1, column 3
  |
1 | ''
  |   ^
expected `.`, `=`
"
    );
    bad!(
        "a = 9e99999",
        "\
TOML parse error at line 1, column 5
  |
1 | a = 9e99999
  |     ^
invalid floating-point number
"
    );

    bad!(
        "a = \"\u{7f}\"",
        "\
TOML parse error at line 1, column 6
  |
1 | a = \"\u{7f}\"
  |      ^
invalid basic string
"
    );
    bad!(
        "a = '\u{7f}'",
        "\
TOML parse error at line 1, column 6
  |
1 | a = '\u{7f}'
  |      ^
invalid literal string
"
    );

    bad!(
        "a = -0x1",
        "\
TOML parse error at line 1, column 7
  |
1 | a = -0x1
  |       ^
expected newline, `#`
"
    );
    bad!(
        "a = 0x-1",
        "\
TOML parse error at line 1, column 7
  |
1 | a = 0x-1
  |       ^
invalid hexadecimal integer
"
    );

    // Dotted keys.
    bad!(
        "a.b.c = 1
         a.b = 2
        ",
        "\
TOML parse error at line 2, column 10
  |
2 |          a.b = 2
  |          ^
duplicate key `b` in document root
"
    );
    bad!(
        "a = 1
         a.b = 2",
        "\
TOML parse error at line 2, column 10
  |
2 |          a.b = 2
  |          ^
dotted key `a` attempted to extend non-table type (integer)
"
    );
    bad!(
        "a = {k1 = 1, k1.name = \"joe\"}",
        "\
TOML parse error at line 1, column 6
  |
1 | a = {k1 = 1, k1.name = \"joe\"}
  |      ^
dotted key `k1` attempted to extend non-table type (integer)
"
    );
}
