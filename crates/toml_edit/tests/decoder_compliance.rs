mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore([
            "invalid/control/comment-cr.toml",
            "invalid/table/append-with-dotted-keys-2.toml",
        ])
        .unwrap();
    harness.test();
}
