#[cfg(feature = "easy")]
mod easy_decoder;
#[cfg(feature = "easy")]
mod easy_encoder;

#[cfg(feature = "easy")]
fn main() {
    let encoder = easy_encoder::Encoder;
    let decoder = easy_decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness
        .ignore([
            // Can't verify until decoder is fixed
            "valid/string/multiline-quotes.toml",
        ])
        .unwrap();
    harness.test();
}

#[cfg(not(feature = "easy"))]
fn main() {}
