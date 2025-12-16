use snapbox::file;

use toml_parser::parser::*;
use toml_parser::Source;

#[track_caller]
fn t(input: &str, expected: impl snapbox::data::IntoData) {
    dbg!(input);
    let mut actual = crate::EventResults::new(input);

    let doc = Source::new(input);
    let tokens = doc.lex().into_vec();
    parse_value(&tokens, &mut actual.events, &mut actual.errors);

    actual.validate(expected);
}

#[test]
fn value_empty() {
    t("", file![_].raw());
}

#[test]
fn value_datetime() {
    t("1979-05-27T00:32:00.999999", file![_].raw());
}

#[test]
fn value_negative_integer() {
    t("-239", file![_].raw());
}

#[test]
fn value_float() {
    t("1e200", file![_].raw());
}

#[test]
fn value_integer_with_seps() {
    t("9_224_617.445_991_228_313", file![_].raw());
}

#[test]
fn value_ml_string_literal_single_line() {
    t(r"'''I [dw]on't need \d{2} apples'''", file![_].raw());
}

#[test]
fn value_ml_string_literal_multiline() {
    t(
        r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#,
        file![_].raw(),
    );
}

#[test]
fn value_string_escaped_unicode() {
    t(r#""Jos\u00E9\n""#, file![_].raw());
}

#[test]
fn value_string_escaped() {
    t(r#""\\\"\b/\f\n\r\t\u00E9\U000A0000""#, file![_].raw());
}

#[test]
fn value_inline_table() {
    t(r#"{ hello = "world", a = 1}"#, file![_].raw());
}

#[test]
fn value_array() {
    t(
        r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        file![_].raw(),
    );
}

#[test]
fn array_empty() {
    t(r#"[]"#, file![_].raw());
}

#[test]
fn array_whitespace() {
    t(r#"[   ]"#, file![_].raw());
}

#[test]
fn array_integers_newlines() {
    t(
        r#"[
1, 2, 3
]"#,
        file![_].raw(),
    );
}

#[test]
fn array_integers_trailing_comment() {
    t(
        r#"[
1,
2, # this is ok
]"#,
        file![_].raw(),
    );
}

#[test]
fn array_comments_only() {
    t(
        r#"[# comment
# comment2


]"#,
        file![_].raw(),
    );
}

#[test]
fn array_integers_surrounded_comment() {
    t(
        r#"[# comment
# comment2
    1

#sd
,
# comment3

]"#,
        file![_].raw(),
    );
}

#[test]
fn array_integer() {
    t(r#"[1]"#, file![_].raw());
}

#[test]
fn array_trailing_comma() {
    t(r#"[1,]"#, file![_].raw());
}

#[test]
fn array_string_types() {
    t(
        r#"[ "all", 'strings', """are the same""", '''type''']"#,
        file![_].raw(),
    );
}

#[test]
fn array_integers_trailing_comma() {
    t(r#"[ 100, -2,]"#, file![_].raw());
}

#[test]
fn array_integers() {
    t(r#"[1, 2, 3]"#, file![_].raw());
}

#[test]
fn array_floats() {
    t(r#"[1.1, 2.1, 3.1]"#, file![_].raw());
}

#[test]
fn array_strings() {
    t(r#"["a", "b", "c"]"#, file![_].raw());
}

#[test]
fn array_nested_same_type() {
    t(r#"[ [ 1, 2 ], [3, 4, 5] ]"#, file![_].raw());
}

#[test]
fn array_nested_multiple_types() {
    t(r#"[ [ 1, 2 ], ["a", "b", "c"] ]"#, file![_].raw());
}

#[test]
fn array_inline_table() {
    t(
        r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        file![_].raw(),
    );
}

#[test]
fn array_no_close() {
    t(r#"["#, file![_].raw());
}

#[test]
fn array_only_comma() {
    t(r#"[,]"#, file![_].raw());
}

#[test]
fn array_leading_comma() {
    t(r#"[,2]"#, file![_].raw());
}

#[test]
fn array_extra_trailing_comma() {
    t(r#"[1e165,,]"#, file![_].raw());
}

#[test]
fn datetime_offset_lower_t() {
    t("1979-05-27t07:32:00Z", file![_].raw());
}

#[test]
fn datetime_offset_space() {
    t("1979-05-27 07:32:00Z", file![_].raw());
}

#[test]
fn datetime_offset_z() {
    t("1979-05-27T07:32:00Z", file![_].raw());
}

#[test]
fn datetime_offset_lower_z() {
    t("1979-05-27T07:32:00z", file![_].raw());
}

#[test]
fn datetime_offset_hour() {
    t("1979-05-27T00:32:00-07:00", file![_].raw());
}

#[test]
fn datetime_offset_minutes() {
    t("1979-05-27T00:32:00-00:36", file![_].raw());
}

#[test]
fn datetime_local() {
    t("1979-05-27T07:32:00", file![_].raw());
}

#[test]
fn datetime_local_fractional_seconds() {
    t("1979-05-27T00:32:00.999999", file![_].raw());
}

#[test]
fn datetime_local_fractional_seconds_truncated() {
    t("1987-07-05T17:45:00.123456789012345Z", file![_].raw());
}

#[test]
fn datetime_date() {
    t("1979-05-27", file![_].raw());
}

#[test]
fn datetime_time() {
    t("07:32:00", file![_].raw());
}

#[test]
fn datetime_time_fractional_seconds() {
    t("00:32:00.999999", file![_].raw());
}

#[test]
fn inline_table_empty() {
    t(r#"{}"#, file![_].raw());
}

#[test]
fn inline_table_ws() {
    t(r#"{   }"#, file![_].raw());
}

#[test]
fn inline_table_key_value() {
    t(r#"{a = 1e165}"#, file![_].raw());
}

#[test]
fn inline_table_multiple_key_values() {
    t(r#"{ hello = "world", a = 1}"#, file![_].raw());
}

#[test]
fn inline_table_dotted_key() {
    t(r#"{ hello.world = "a" }"#, file![_].raw());
}

#[test]
fn inline_table_only_comma() {
    t(r#"{,}"#, file![_].raw());
}

#[test]
fn inline_table_leading_comma() {
    t(r#"{ , hello  = "a" }"#, file![_].raw());
}

#[test]
fn inline_table_trailing_comma() {
    t(r#"{ hello  = "a", }"#, file![_].raw());
}

#[test]
fn inline_table_no_comma() {
    t(r#"{ hello = "a" world = "b" }"#, file![_].raw());
}

#[test]
fn inline_table_no_eq() {
    t(r#"{ hello  "a" }"#, file![_].raw());
}

#[test]
fn inline_table_no_close() {
    t(r#"{a = 1e165"#, file![_].raw());
}

#[test]
fn inline_table_repeated_key() {
    t(r#"{ hello = "world", a = 2, hello = 1}"#, file![_].raw());
}

#[test]
fn integer() {
    t("42", file![_].raw());
}

#[test]
fn integer_positive() {
    t("+99", file![_].raw());
}

#[test]
fn integer_zero() {
    t("0", file![_].raw());
}

#[test]
fn integer_negative() {
    t("-17", file![_].raw());
}

#[test]
fn integer_one_sep() {
    t("1_000", file![_].raw());
}

#[test]
fn integer_multiple_seps() {
    t("5_349_221", file![_].raw());
}

#[test]
fn integer_every_other_sep() {
    t("1_2_3_4_5", file![_].raw());
}

#[test]
fn integer_hex() {
    t("0xF", file![_].raw());
}

#[test]
fn integer_hex_neg() {
    t("0x-F", file![_].raw());
}

#[test]
fn integer_neg_hex() {
    t("-0xF", file![_].raw());
}

#[test]
fn integer_oct_sep() {
    t("0o0_755", file![_].raw());
}

#[test]
fn integer_bin_sep() {
    t("0b1_0_1", file![_].raw());
}

#[test]
fn integer_i64_min() {
    t(&i64::MIN.to_string(), file![_].raw());
}

#[test]
fn integer_i64_max() {
    t(&i64::MAX.to_string(), file![_].raw());
}

#[test]
fn integer_exceed_i64() {
    t("1000000000000000000000000000000000", file![_].raw());
}

#[test]
fn float() {
    t("3.1419", file![_].raw());
}

#[test]
fn float_positive() {
    t("+1.0", file![_].raw());
}

#[test]
fn float_negative() {
    t("-0.01", file![_].raw());
}

#[test]
fn float_pos() {
    t("1e6", file![_].raw());
}

#[test]
fn float_exp_positive() {
    t("5e+22", file![_].raw());
}

#[test]
fn float_exp_negative() {
    t("6.626e-34", file![_].raw());
}

#[test]
fn float_negative_exp_negative() {
    t("-2E-2", file![_].raw());
}

#[test]
fn float_sep() {
    t("9_224_617.445_991_228_313", file![_].raw());
}

#[test]
fn float_f64_min() {
    t("-1.7976931348623157e+308", file![_].raw());
}

#[test]
fn float_f64_max() {
    t("1.7976931348623157e+308", file![_].raw());
}

#[test]
fn float_nan() {
    t("nan", file![_].raw());
}

#[test]
fn float_nan_positive() {
    t("+nan", file![_].raw());
}

#[test]
fn float_nan_negative() {
    t("-nan", file![_].raw());
}

#[test]
fn float_inf() {
    t("inf", file![_].raw());
}

#[test]
fn float_inf_positive() {
    t("+inf", file![_].raw());
}

#[test]
fn float_inf_negative() {
    t("-inf", file![_].raw());
}

#[test]
fn float_exceed_f64() {
    t("1e+400", file![_].raw());
}

#[test]
fn string_escaped() {
    t(
        r#""I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF. \U0002070E""#,
        file![_].raw(),
    );
}

#[test]
fn string_escaped_escape() {
    t(r#""\e There is no escape! \e""#, file![_].raw());
}

#[test]
fn string_ml_string_multiple_lines() {
    t(
        r#""""
Roses are red
Violets are blue""""#,
        file![_].raw(),
    );
}

#[test]
fn string_confusing_quotes_1() {
    t(r#"""" \""" """"#, file![_].raw());
}

#[test]
fn string_confusing_quotes_2() {
    t(r#"""" \\""""#, file![_].raw());
}

#[test]
fn string_confusing_quotes_3() {
    t(r#""""  """#, file![_].raw());
}

#[test]
fn string_confusing_quotes_4() {
    t(r#""""  \""""#, file![_].raw());
}

#[test]
fn string() {
    t(
        r#""""
The quick brown \


  fox jumps over \
    the lazy dog.""""#,
        file![_].raw(),
    );
}

#[test]
fn string_trailing_slash_1() {
    t(
        r#""""\
       """"#,
        file![_].raw(),
    );
}

#[test]
fn string_trailing_slash_2() {
    t(
        r#""""
\
  \
""""#,
        file![_].raw(),
    );
}

#[test]
fn string_trailing_slash_3() {
    t(
        r#""""\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """"#,
        file![_].raw(),
    );
}

#[test]
fn string_literal_path_1() {
    t(r"'C:\Users\nodejs\templates'", file![_].raw());
}

#[test]
fn string_literal_path_2() {
    t(r"'\\ServerX\admin$\system32\'", file![_].raw());
}

#[test]
fn string_literal_quotes() {
    t(r#"'Tom "Dubs" Preston-Werner'"#, file![_].raw());
}

#[test]
fn string_literal_escaped_1() {
    t(r"'<\i\c*\s*>'", file![_].raw());
}

#[test]
fn string_literal_escaped_2() {
    t(r"'''I [dw]on't need \d{2} apples'''", file![_].raw());
}

#[test]
fn string_literal_confusing_quotes() {
    t(r#"''''one_quote''''"#, file![_].raw());
}

#[test]
fn string_ml_string_literal_multiple_lines() {
    t(
        r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#,
        file![_].raw(),
    );
}

#[test]
fn string_missing_opening() {
    t(
        r#"invalid url to tqdm-4.66.0-py3-none-any.whl""#,
        file![_].raw(),
    );
}
