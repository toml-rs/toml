use snapbox::file;

use toml_parser::Source;
use toml_parser::parser::*;

#[track_caller]
fn t(input: &str, expected: impl snapbox::data::IntoData) {
    dbg!(input);
    let mut actual = crate::EventResults::new(input);

    let doc = Source::new(input);
    let tokens = doc.lex().into_vec();
    parse_simple_key(&tokens, &mut actual.events, &mut actual.errors);

    actual.validate(expected);
}

#[test]
fn simple_key_empty() {
    t("", file![_].raw());
}

#[test]
fn simple_key_unquoted_ascii() {
    t("a", file![_].raw());
}

#[test]
fn simple_key_string() {
    t(r#""hello\n ""#, file![_].raw());
}

#[test]
fn simple_key_string_literal() {
    t(r"'hello\n '", file![_].raw());
}
