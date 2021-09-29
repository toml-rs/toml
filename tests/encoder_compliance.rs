mod decoder;
mod encoder;

fn main() {
    let encoder = encoder::Encoder;
    let decoder = decoder::Decoder;
    let harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness.test();
}
