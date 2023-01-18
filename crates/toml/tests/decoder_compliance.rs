mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore([
            "invalid/control/comment-cr.toml",
            "invalid/control/comment-del.toml",
            "invalid/datetime/hour-over.toml",
            "invalid/integer/positive-bin.toml",
            "invalid/integer/positive-hex.toml",
            "valid/inline-table/newline.toml",
            "valid/spec/float-0.toml",
            "valid/spec/table-9.toml",
            // Unreleased
            "valid/string/escape-esc.toml",
            "valid/string/hex-escape.toml",
            "valid/datetime/no-seconds.toml",
        ])
        .unwrap();
    harness.test();
}
