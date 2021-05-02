use toml_edit::Document;

fn run(toml: &str, msg: &str) {
    let doc = toml.parse::<Document>();
    assert!(doc.is_err());

    let err = match doc {
        Err(e) => e.to_string(),
        _ => unreachable!(""),
    };
    assert!(err.contains(msg));
}

macro_rules! t(
    ($name:ident, $msg:expr, $toml:expr) => (
        #[test]
        fn $name() {
            run(include_str!($toml), $msg);
        }
    )
);

t!(
    test_array_mixed_types_arrays_and_ints,
    "Mixed types in array",
    "fixtures/invalid/array-mixed-types-arrays-and-ints.toml"
);
t!(
    test_array_mixed_types_ints_and_floats,
    "Mixed types in array",
    "fixtures/invalid/array-mixed-types-ints-and-floats.toml"
);
t!(
    test_array_mixed_types_strings_and_ints,
    "Mixed types in array",
    "fixtures/invalid/array-mixed-types-strings-and-ints.toml"
);
t!(
    test_datetime_malformed_no_leads,
    "While parsing a Date-Time",
    "fixtures/invalid/datetime-malformed-no-leads.toml"
);
t!(
    test_datetime_malformed_no_secs,
    "While parsing a Date-Time",
    "fixtures/invalid/datetime-malformed-no-secs.toml"
);
t!(
    test_datetime_malformed_no_t,
    "",
    "fixtures/invalid/datetime-malformed-no-t.toml"
);
t!(
    test_datetime_malformed_with_milli,
    "While parsing a Date-Time",
    "fixtures/invalid/datetime-malformed-with-milli.toml"
);
t!(
    test_duplicate_keys,
    "Duplicate key",
    "fixtures/invalid/duplicate-keys.toml"
);
t!(
    test_duplicate_key_table,
    "Duplicate key",
    "fixtures/invalid/duplicate-key-table.toml"
);
t!(
    test_duplicate_tables,
    "Duplicate key",
    "fixtures/invalid/duplicate-tables.toml"
);
t!(
    test_empty_implicit_table,
    "While parsing a Table Header",
    "fixtures/invalid/empty-implicit-table.toml"
);
t!(
    test_empty_table,
    "While parsing a Table Header",
    "fixtures/invalid/empty-table.toml"
);
t!(
    test_float_leading_zero_neg,
    "Unexpected `3`",
    "fixtures/invalid/float-leading-zero-neg.toml"
);
t!(
    test_float_leading_zero_pos,
    "Unexpected `3`",
    "fixtures/invalid/float-leading-zero-pos.toml"
);
t!(
    test_float_leading_zero,
    "Unexpected `3`",
    "fixtures/invalid/float-leading-zero.toml"
);
t!(
    test_float_no_leading_zero,
    "Unexpected `.`",
    "fixtures/invalid/float-no-leading-zero.toml"
);
t!(
    test_float_no_trailing_digits,
    "While parsing a Float",
    "fixtures/invalid/float-no-trailing-digits.toml"
);
t!(
    test_float_underscore_after_point,
    "While parsing a Float",
    "fixtures/invalid/float-underscore-after-point.toml"
);
t!(
    test_float_underscore_after,
    "column 11",
    "fixtures/invalid/float-underscore-after.toml"
);
t!(
    test_float_underscore_before_point,
    "column 9",
    "fixtures/invalid/float-underscore-before-point.toml"
);
t!(
    test_float_underscore_before,
    "column 7",
    "fixtures/invalid/float-underscore-before.toml"
);
t!(
    test_integer_leading_zero_neg,
    "",
    "fixtures/invalid/integer-leading-zero-neg.toml"
);
t!(
    test_integer_leading_zero_pos,
    "",
    "fixtures/invalid/integer-leading-zero-pos.toml"
);
t!(
    test_integer_leading_zero,
    "",
    "fixtures/invalid/integer-leading-zero.toml"
);
t!(
    test_integer_underscore_after,
    "",
    "fixtures/invalid/integer-underscore-after.toml"
);
t!(
    test_integer_underscore_before,
    "",
    "fixtures/invalid/integer-underscore-before.toml"
);
t!(
    test_integer_underscore_double,
    "",
    "fixtures/invalid/integer-underscore-double.toml"
);
t!(
    test_integer_invalid_hex_char,
    "",
    "fixtures/invalid/integer-invalid-hex-char.toml"
);
t!(
    test_integer_invalid_octal_char,
    "",
    "fixtures/invalid/integer-invalid-octal-char.toml"
);
t!(
    test_integer_invalid_binary_char,
    "",
    "fixtures/invalid/integer-invalid-binary-char.toml"
);
t!(
    test_key_after_array,
    "",
    "fixtures/invalid/key-after-array.toml"
);
t!(
    test_key_after_table,
    "",
    "fixtures/invalid/key-after-table.toml"
);
t!(test_key_empty, "", "fixtures/invalid/key-empty.toml");
t!(test_key_hash, "", "fixtures/invalid/key-hash.toml");
t!(test_key_newline, "", "fixtures/invalid/key-newline.toml");
t!(test_key_no_eol, "", "fixtures/invalid/key-no-eol.toml");
t!(
    test_key_open_bracket,
    "",
    "fixtures/invalid/key-open-bracket.toml"
);
t!(
    test_key_single_open_bracket,
    "",
    "fixtures/invalid/key-single-open-bracket.toml"
);
t!(test_key_space, "", "fixtures/invalid/key-space.toml");
t!(
    test_key_start_bracket,
    "",
    "fixtures/invalid/key-start-bracket.toml"
);
t!(
    test_key_two_equals,
    "",
    "fixtures/invalid/key-two-equals.toml"
);
t!(test_llbrace, "", "fixtures/invalid/llbrace.toml");
t!(test_rrbrace, "", "fixtures/invalid/rrbrace.toml");
t!(
    test_string_bad_byte_escape,
    "",
    "fixtures/invalid/string-bad-byte-escape.toml"
);
t!(
    test_string_bad_escape,
    "",
    "fixtures/invalid/string-bad-escape.toml"
);
t!(
    test_string_bad_surrogate,
    "",
    "fixtures/invalid/string-bad-surrogate.toml"
);
t!(
    test_string_bad_uni_esc,
    "",
    "fixtures/invalid/string-bad-uni-esc.toml"
);
t!(
    test_string_byte_escapes,
    "",
    "fixtures/invalid/string-byte-escapes.toml"
);
t!(
    test_string_no_close,
    "",
    "fixtures/invalid/string-no-close.toml"
);
t!(
    test_table_array_implicit,
    "",
    "fixtures/invalid/table-array-implicit.toml"
);
t!(
    test_table_array_malformed_bracket,
    "",
    "fixtures/invalid/table-array-malformed-bracket.toml"
);
t!(
    test_table_array_malformed_empty,
    "",
    "fixtures/invalid/table-array-malformed-empty.toml"
);
t!(test_table_empty, "", "fixtures/invalid/table-empty.toml");
t!(
    test_table_nested_brackets_close,
    "",
    "fixtures/invalid/table-nested-brackets-close.toml"
);
t!(
    test_table_nested_brackets_open,
    "",
    "fixtures/invalid/table-nested-brackets-open.toml"
);
t!(
    test_table_whitespace,
    "",
    "fixtures/invalid/table-whitespace.toml"
);
t!(
    test_table_with_pound,
    "",
    "fixtures/invalid/table-with-pound.toml"
);
t!(
    test_text_after_array_entries,
    "",
    "fixtures/invalid/text-after-array-entries.toml"
);
t!(
    test_text_after_integer,
    "",
    "fixtures/invalid/text-after-integer.toml"
);
t!(
    test_text_after_string,
    "",
    "fixtures/invalid/text-after-string.toml"
);
t!(
    test_text_after_table,
    "",
    "fixtures/invalid/text-after-table.toml"
);
t!(
    test_text_before_array_separator,
    "",
    "fixtures/invalid/text-before-array-separator.toml"
);
t!(
    test_text_in_array,
    "",
    "fixtures/invalid/text-in-array.toml"
);
