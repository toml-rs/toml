mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore([
            "valid/spec/float-0.toml",
            // Unreleased
            "valid/string/escape-esc.toml",
            "valid/string/hex-escape.toml",
            "valid/datetime/no-seconds.toml",
            "valid/inline-table/newline.toml",
        ])
        .unwrap();
    harness.test();
}
