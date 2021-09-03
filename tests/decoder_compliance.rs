mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness
        .ignore(["valid/string/multiline-quotes.toml"])
        .unwrap();
    harness.test();
}
