#[cfg(feature = "easy")]
mod easy_decoder;
#[cfg(feature = "easy")]
mod easy_encoder;

#[cfg(feature = "easy")]
fn main() {
    let encoder = easy_encoder::Encoder;
    let decoder = easy_decoder::Decoder;
    let harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness.test();
}

#[cfg(not(feature = "easy"))]
fn main() {}
