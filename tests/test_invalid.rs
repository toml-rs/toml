use toml_edit::Document;

#[track_caller]
fn run_contains(toml: &str, msg: &str) {
    let doc = toml.parse::<Document>();

    let err = match doc {
        Err(e) => e.to_string(),
        _ => unreachable!("must fail"),
    };
    dbg!(msg);
    dbg!(&err);
    assert!(err.contains(msg));
}

#[track_caller]
fn run_exact(toml: &str, msg: &str) {
    let doc = toml.parse::<Document>();

    let err = match doc {
        Err(e) => e.to_string(),
        _ => unreachable!("must fail"),
    };
    assert_eq!(err, msg);
}

macro_rules! t_exact(
    ($name:ident, $toml:expr, $msg:expr, ) => (
        #[test]
        fn $name() {
            run_exact($toml, $msg);
        }
    )
);

macro_rules! t_file_contains(
    ($name:ident, $toml:expr, $msg:expr, ) => (
        #[test]
        fn $name() {
            run_contains(include_str!($toml), $msg);
        }
    )
);

t_file_contains!(
    test_datetime_malformed_no_leads,
    "fixtures/invalid/datetime-malformed-no-leads.toml",
    "While parsing a Date-Time",
);
t_file_contains!(
    test_datetime_malformed_no_secs,
    "fixtures/invalid/datetime-malformed-no-secs.toml",
    "While parsing a Date-Time",
);
t_file_contains!(
    test_datetime_malformed_no_t,
    "fixtures/invalid/datetime-malformed-no-t.toml",
    "",
);
t_file_contains!(
    test_datetime_malformed_with_milli,
    "fixtures/invalid/datetime-malformed-with-milli.toml",
    "While parsing a Date-Time",
);
t_file_contains!(
    test_duplicate_keys,
    "fixtures/invalid/duplicate-keys.toml",
    "Duplicate key",
);
t_file_contains!(
    test_duplicate_key_table,
    "fixtures/invalid/duplicate-key-table.toml",
    "Duplicate key",
);
t_file_contains!(
    test_duplicate_tables,
    "fixtures/invalid/duplicate-tables.toml",
    "Duplicate key",
);
t_file_contains!(
    test_duplicate_key_std_into_dotted,
    "fixtures/invalid/duplicate-key-std-into-dotted.toml",
    "Duplicate key",
);
t_file_contains!(
    test_duplicate_key_dotted_into_std,
    "fixtures/invalid/duplicate-key-dotted-into-std.toml",
    "Duplicate key",
);
t_file_contains!(
    test_empty_implicit_table,
    "fixtures/invalid/empty-implicit-table.toml",
    "While parsing a Table Header",
);
t_file_contains!(
    test_empty_table,
    "fixtures/invalid/empty-table.toml",
    "While parsing a Table Header",
);
t_file_contains!(
    test_float_leading_zero_neg,
    "fixtures/invalid/float-leading-zero-neg.toml",
    "Unexpected `3`",
);
t_file_contains!(
    test_float_leading_zero_pos,
    "fixtures/invalid/float-leading-zero-pos.toml",
    "Unexpected `3`",
);
t_file_contains!(
    test_float_leading_zero,
    "fixtures/invalid/float-leading-zero.toml",
    "Unexpected `3`",
);
t_file_contains!(
    test_float_no_leading_zero,
    "fixtures/invalid/float-no-leading-zero.toml",
    "Unexpected `.`",
);
t_file_contains!(
    test_float_no_trailing_digits,
    "fixtures/invalid/float-no-trailing-digits.toml",
    "While parsing a Float",
);
t_file_contains!(
    test_float_underscore_after_point,
    "fixtures/invalid/float-underscore-after-point.toml",
    "While parsing a Float",
);
t_file_contains!(
    test_float_underscore_after,
    "fixtures/invalid/float-underscore-after.toml",
    "column 11",
);
t_file_contains!(
    test_float_underscore_before_point,
    "fixtures/invalid/float-underscore-before-point.toml",
    "column 9",
);
t_file_contains!(
    test_float_underscore_before,
    "fixtures/invalid/float-underscore-before.toml",
    "column 7",
);
t_file_contains!(
    test_integer_leading_zero_neg,
    "fixtures/invalid/integer-leading-zero-neg.toml",
    "",
);
t_file_contains!(
    test_integer_leading_zero_pos,
    "fixtures/invalid/integer-leading-zero-pos.toml",
    "",
);
t_file_contains!(
    test_integer_leading_zero,
    "fixtures/invalid/integer-leading-zero.toml",
    "",
);
t_file_contains!(
    test_integer_underscore_after,
    "fixtures/invalid/integer-underscore-after.toml",
    "",
);
t_file_contains!(
    test_integer_underscore_before,
    "fixtures/invalid/integer-underscore-before.toml",
    "",
);
t_file_contains!(
    test_integer_underscore_double,
    "fixtures/invalid/integer-underscore-double.toml",
    "",
);
t_file_contains!(
    test_integer_invalid_hex_char,
    "fixtures/invalid/integer-invalid-hex-char.toml",
    "",
);
t_file_contains!(
    test_integer_invalid_octal_char,
    "fixtures/invalid/integer-invalid-octal-char.toml",
    "",
);
t_file_contains!(
    test_integer_invalid_binary_char,
    "fixtures/invalid/integer-invalid-binary-char.toml",
    "",
);
t_file_contains!(
    test_key_after_array,
    "fixtures/invalid/key-after-array.toml",
    "",
);
t_file_contains!(
    test_key_after_table,
    "fixtures/invalid/key-after-table.toml",
    "",
);
t_file_contains!(test_key_empty, "fixtures/invalid/key-empty.toml", "",);
t_file_contains!(test_key_hash, "fixtures/invalid/key-hash.toml", "",);
t_file_contains!(test_key_newline, "fixtures/invalid/key-newline.toml", "",);
t_file_contains!(test_key_no_eol, "fixtures/invalid/key-no-eol.toml", "",);
t_file_contains!(
    test_key_open_bracket,
    "fixtures/invalid/key-open-bracket.toml",
    "",
);
t_file_contains!(
    test_key_single_open_bracket,
    "fixtures/invalid/key-single-open-bracket.toml",
    "",
);
t_file_contains!(test_key_space, "fixtures/invalid/key-space.toml", "",);
t_file_contains!(
    test_key_start_bracket,
    "fixtures/invalid/key-start-bracket.toml",
    "",
);
t_file_contains!(
    test_key_two_equals,
    "fixtures/invalid/key-two-equals.toml",
    "",
);
t_file_contains!(test_llbrace, "fixtures/invalid/llbrace.toml", "",);
t_file_contains!(test_rrbrace, "fixtures/invalid/rrbrace.toml", "",);
t_file_contains!(
    test_string_bad_byte_escape,
    "fixtures/invalid/string-bad-byte-escape.toml",
    "",
);
t_file_contains!(
    test_string_bad_escape,
    "fixtures/invalid/string-bad-escape.toml",
    "",
);
t_file_contains!(
    test_string_bad_surrogate,
    "fixtures/invalid/string-bad-surrogate.toml",
    "",
);
t_file_contains!(
    test_string_bad_uni_esc,
    "fixtures/invalid/string-bad-uni-esc.toml",
    "",
);
t_file_contains!(
    test_string_byte_escapes,
    "fixtures/invalid/string-byte-escapes.toml",
    "",
);
t_file_contains!(
    test_string_no_close,
    "fixtures/invalid/string-no-close.toml",
    "",
);
t_file_contains!(
    test_table_array_implicit,
    "fixtures/invalid/table-array-implicit.toml",
    "",
);
t_file_contains!(
    test_table_array_malformed_bracket,
    "fixtures/invalid/table-array-malformed-bracket.toml",
    "",
);
t_file_contains!(
    test_table_array_malformed_empty,
    "fixtures/invalid/table-array-malformed-empty.toml",
    "",
);
t_file_contains!(test_table_empty, "fixtures/invalid/table-empty.toml", "",);
t_file_contains!(
    test_table_nested_brackets_close,
    "fixtures/invalid/table-nested-brackets-close.toml",
    "",
);
t_file_contains!(
    test_table_nested_brackets_open,
    "fixtures/invalid/table-nested-brackets-open.toml",
    "",
);
t_file_contains!(
    test_table_whitespace,
    "fixtures/invalid/table-whitespace.toml",
    "",
);
t_file_contains!(
    test_table_with_pound,
    "fixtures/invalid/table-with-pound.toml",
    "",
);
t_file_contains!(
    test_text_after_array_entries,
    "fixtures/invalid/text-after-array-entries.toml",
    "",
);
t_file_contains!(
    test_text_after_integer,
    "fixtures/invalid/text-after-integer.toml",
    "",
);
t_file_contains!(
    test_text_after_string,
    "fixtures/invalid/text-after-string.toml",
    "",
);
t_file_contains!(
    test_text_after_table,
    "fixtures/invalid/text-after-table.toml",
    "",
);
t_file_contains!(
    test_text_before_array_separator,
    "fixtures/invalid/text-before-array-separator.toml",
    "",
);
t_file_contains!(
    test_text_in_array,
    "fixtures/invalid/text-in-array.toml",
    "",
);

t_exact!(
    test_quote_suggestion_in_key_value_pair,
    "value= ZZZ",
    "TOML parse error at line 1, column 8
  |
1 | value= ZZZ
  |        ^
Unexpected `Z`
Expected `-`, `+`, `inf`, `nan`, `0x`, `0o` or `0b`
expected 4 more elements
expected 2 more elements
While parsing a Time
While parsing a hexadecimal Integer
While parsing a octal Integer
While parsing a binary Integer
While parsing an Integer
While parsing a Date-Time
While parsing a Float
",
);
t_exact!(
    test_quote_suggestion_in_array,
    "value=[ZZZ]",
    "TOML parse error at line 1, column 8
  |
1 | value=[ZZZ]
  |        ^
Unexpected `Z`
Expected `a newline` or `#`
",
);
t_exact!(
    test_quote_suggestion_in_inline_table,
    "value={key = ZZZ}",
    "TOML parse error at line 1, column 14
  |
1 | value={key = ZZZ}
  |              ^
Unexpected `Z`
Expected `-`, `+`, `inf`, `nan`, `0x`, `0o` or `0b`
expected 4 more elements
expected 2 more elements
While parsing a Time
While parsing a hexadecimal Integer
While parsing a octal Integer
While parsing a binary Integer
While parsing an Integer
While parsing a Date-Time
While parsing a Float
",
);
t_exact!(
    test_quote_suggestion_similar_to_constants_in_key_value_pair,
    "value= trust",
    "TOML parse error at line 1, column 9
  |
1 | value= trust
  |         ^
Unexpected `r`
Expected `rue`
",
);
