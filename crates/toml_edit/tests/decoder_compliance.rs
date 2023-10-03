mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.version("1.0.0");
    harness.test();
}
