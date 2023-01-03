mod decoder;

fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.ignore(["invalid/control/comment-cr.toml"]).unwrap();
    harness.test();
}
