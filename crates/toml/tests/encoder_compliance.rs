mod decoder;
mod encoder;

fn main() {
    let encoder = encoder::Encoder;
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness
        .ignore([
            "valid/array/mixed-string-table.toml",
            "valid/inline-table/nest.toml",
        ])
        .unwrap();
    harness.test();
}
