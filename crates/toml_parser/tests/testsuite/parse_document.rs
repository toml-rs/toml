use snapbox::file;

use toml_parser::Source;
use toml_parser::parser::*;

#[track_caller]
fn t(input: &str, expected: impl snapbox::data::IntoData) {
    dbg!(input);
    let mut actual = crate::EventResults::new(input);

    let doc = Source::new(input);
    let tokens = doc.lex().into_vec();
    parse_document(&tokens, &mut actual.events, &mut actual.errors);

    actual.validate(expected);
}

#[track_caller]
fn t_recurse(input: &str, max_depth: u32, expected: impl snapbox::data::IntoData) {
    dbg!(input);
    let mut actual = crate::EventResults::new(input);

    let doc = Source::new(input);
    let tokens = doc.lex().into_vec();
    let mut recursion = RecursionGuard::new(&mut actual.events, max_depth);
    parse_document(&tokens, &mut recursion, &mut actual.errors);

    actual.validate(expected);
}

#[test]
fn document_empty() {
    t("", file![_].raw());
}

#[test]
fn document_ws() {
    t(r#"  "#, file![_].raw());
}

#[test]
fn document_key_string() {
    t(
        r#"hello.world = "a"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_string_comment() {
    t(
        r#" hello = 'darkness' # my old friend
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_ws() {
    t(
        r#" hello . darkness . my = 'old friend'
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table() {
    t(
        r#"[parent . child]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_missing_key() {
    t(
        r#"[]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_dot() {
    t(
        r#"[ . ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_dot_dot() {
    t(
        r#"[ . . ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_dot_key() {
    t(
        r#"[ . table ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_dot_dot_key() {
    t(
        r#"[ . table ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_key_dot() {
    t(
        r#"[ table . ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_key_dot_dot() {
    t(
        r#"[ table . . ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_table_key_dot_dot_key() {
    t(
        r#"[ table . . table ]
key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_missing_key() {
    t(
        r#"
 = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_dot() {
    t(
        r#"
 . = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_dot_dot() {
    t(
        r#"
 . . = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_dot_key() {
    t(
        r#"
 . key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_dot_dot_key() {
    t(
        r#"
 . . key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_dot() {
    t(
        r#"
key . = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_dot_dot() {
    t(
        r#"
key . . = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_dot_dot_key() {
    t(
        r#"
key . . key = "value"
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_missing_key() {
    t(
        r#"
parent = { = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_dot() {
    t(
        r#"
parent = { . = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_dot_dot() {
    t(
        r#"
parent = { . . = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_dot_key() {
    t(
        r#"
parent = { . key = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_dot_dot_key() {
    t(
        r#"
parent = { . . key = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_key_dot() {
    t(
        r#"
parent = { key . = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_key_dot_dot() {
    t(
        r#"
parent = { key . . = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_inline_table_key_dot_dot_key() {
    t(
        r#"
parent = { key . . key = "value" }
"#,
        file![_].raw(),
    );
}

#[test]
fn document_key_datetime() {
    t(
        r#"foo = 1979-05-27 # Comment
"#,
        file![_].raw(),
    );
}

#[test]
fn document_bom() {
    t(
        "\u{FEFF}
[package]
name = \"foo\"
version = \"0.0.1\"
authors = []
",
        file![_].raw(),
    );
}

#[test]
fn document_complex() {
    t(
        r#"
# This is a TOML document.

title = "TOML Example"

    [owner]
    name = "Tom Preston-Werner"
    dob = 1979-05-27T07:32:00-08:00 # First class dates

    [database]
    server = "192.168.1.1"
    ports = [ 8001, 8001, 8002 ]
    connection_max = 5000
    enabled = true

    [servers]

    # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
    ip = "10.0.0.1"
    dc = "eqdc10"

    [servers.beta]
    ip = "10.0.0.2"
    dc = "eqdc10"

    [clients]
    data = [ ["gamma", "delta"], [1, 2] ]

    # Line breaks are OK when inside arrays
hosts = [
    "alpha",
    "omega"
]

   'some.weird .stuff'   =  """
                         like
                         that
                      #   """ # this broke my syntax highlighting
   " also. like " = '''
that
'''
   double = 2e39 # this number looks familiar
# trailing comment"#,
        file![_].raw(),
    );
}

#[test]
fn document_invalid() {
    t(
        r#" hello = 'darkness' # my old friend
$"#,
        file![_].raw(),
    );
}

#[test]
fn document_invalid_comment() {
    t(
        " hello = 'darkness' # my old\0 friend
",
        file![_].raw(),
    );
}

#[test]
fn document_invalid_cr() {
    t(" hello = 'darkness' # my old friend\r", file![_].raw());
}

#[test]
fn inline_table_within_recursion_limit() {
    t_recurse(
        "
key = { a = { b = 1 } }
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn inline_table_outside_recursion_limit() {
    t_recurse(
        "
key = { a = { b = { c = { d = 1 } } } }
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn array_within_recursion_limit() {
    t_recurse(
        "
key = [[1]]
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn array_outside_recursion_limit() {
    t_recurse(
        "
key = [[[[1]]]]
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn array_and_inline_table_within_recursion_limit() {
    t_recurse(
        "
key = [{ a = 1 }]
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn array_and_inline_table_outside_recursion_limit() {
    t_recurse(
        "
key = [{ a = [{ b = [{ c = 1 }] }] }]
after = [10]
",
        2,
        file![_].raw(),
    );
}

#[test]
fn hex_with_spaces() {
    t(
        "
v=0x _
",
        file![_].raw(),
    );
}

#[test]
fn hex_with_bad_chars() {
    t(
        "
v=0xz_
",
        file![_].raw(),
    );
}

#[test]
fn float_with_bad_underscore() {
    t(
        "
v=1_.2
",
        file![_].raw(),
    );
}
