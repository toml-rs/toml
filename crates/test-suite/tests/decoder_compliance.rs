mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore([
            "invalid/integer/positive-hex.toml",
            "invalid/integer/positive-bin.toml",
            "invalid/control/comment-del.toml",
            "invalid/control/comment-cr.toml",
            "valid/string/escape-esc.toml",
            "invalid/datetime/hour-over.toml",
        ])
        .unwrap();
    harness.test();
}
