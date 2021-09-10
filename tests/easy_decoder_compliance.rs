#[cfg(feature = "easy")]
mod easy_decoder;

#[cfg(feature = "easy")]
fn main() {
    let decoder = easy_decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore(["valid/string/multiline-quotes.toml"])
        .unwrap();
    harness.test();
}

#[cfg(not(feature = "easy"))]
fn main() {}
