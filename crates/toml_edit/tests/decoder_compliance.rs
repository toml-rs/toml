mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore([
            "invalid/control/comment-cr.toml",
            "valid/string/escape-esc.toml",
            "invalid/table/duplicate-key-dotted-table.toml",
            "invalid/table/duplicate-key-dotted-table2.toml",
        ])
        .unwrap();
    harness.test();
}
