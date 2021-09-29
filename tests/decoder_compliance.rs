mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.test();
}
