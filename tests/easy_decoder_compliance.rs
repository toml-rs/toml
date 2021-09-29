#[cfg(feature = "easy")]
mod easy_decoder;

#[cfg(feature = "easy")]
fn main() {
    let decoder = easy_decoder::Decoder;
    let harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.test();
}

#[cfg(not(feature = "easy"))]
fn main() {}
