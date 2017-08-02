extern crate toml_edit;

use toml_edit::{Key, Value};

macro_rules! parse {
    ($s:expr, $ty:ty) => (
        {
            let v = $s.parse::<$ty>();
            assert!(v.is_ok());
            v.unwrap()
        }
    );
}

macro_rules! parse_value {
    ($s:expr) => (parse!($s, Value));
}

macro_rules! test_key {
    ($s:expr, $expected:expr) => (
        {
            let key = parse!($s, Key);
            assert_eq!(key.get(), $expected);
        }
    );
}

macro_rules! parse_error {
    ($input:expr, $ty:ty, $err_msg:expr) => (
        {
            let res = $input.parse::<$ty>();
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert!(err.to_string().find($err_msg).is_some());
        }
    );
}

#[test]
fn test_parse_error() {
    parse_error!("'hello'bla", Value, "InvalidValue");
    parse_error!(r#"["", 2]"#, Value, "MixedArrayType");
    parse_error!(r#"{a = 2"#, Value, "UnterminatedInlineTable");

    parse_error!("abc\n", Key, "InvalidKey");
    parse_error!("", Key, "InvalidKey");
    parse_error!("'hello'bla", Key, "InvalidKey");
}

#[test]
fn test_key_from_str() {
    test_key!("a", "a");
    test_key!(r#"'hello key'"#, "hello key");
    test_key!(r#""Jos\u00E9""#, "Jos\u{00E9}");
}

#[test]
fn test_value_from_str() {
    assert!(parse_value!("1979-05-27T00:32:00.999999").is_date_time());
    assert!(parse_value!("-239").is_integer());
    assert!(parse_value!("1e200").is_float());
    assert!(parse_value!("9_224_617.445_991_228_313").is_float());
    assert!(parse_value!(r#""basic string\nJos\u00E9\n""#).is_str());
    assert!(
        parse_value!(
            r#""""
multiline basic string
""""#
        ).is_str()
    );
    assert!(parse_value!(r#"'literal string\ \'"#).is_str());
    assert!(
        parse_value!(
            r#"'''multiline
literal \ \
string'''"#
        ).is_str()
    );
    assert!(parse_value!(r#"{ hello = "world", a = 1}"#).is_inline_table());
    assert!(
        parse_value!(r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#).is_array()
    );
}
