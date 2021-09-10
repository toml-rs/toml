mod decoder;
mod encoder;

fn main() {
    let encoder = encoder::Encoder;
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness
        .ignore([
            // Can't verify until decoder is fixed
            "valid/string/multiline-quotes.toml",
        ])
        .unwrap();
    harness.test();
}
