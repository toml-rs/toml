mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.ignore(["valid/spec/float-0.toml"]).unwrap();
    harness.version("1.0.0");
    harness.test();
}
