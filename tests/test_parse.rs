extern crate toml_edit;

use toml_edit::Value;

macro_rules! parse_value {
    ($s:expr) => (
        {
            let v = $s.parse::<Value>();
            assert!(v.is_ok());
            v.unwrap()
        }
    );
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
