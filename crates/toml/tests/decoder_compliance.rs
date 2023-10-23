mod decoder;

#[cfg(all(feature = "parse", feature = "display"))]
fn main() {
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.version("1.0.0");
    harness
        .ignore([
            "invalid/datetime/feb-30.toml",
            "invalid/datetime/feb-29.toml",
            "invalid/local-date/feb-30.toml",
            "invalid/local-date/feb-29.toml",
            "invalid/local-datetime/feb-29.toml",
            "invalid/local-datetime/feb-30.toml",
        ])
        .unwrap();
    harness.test();
}

#[cfg(not(all(feature = "parse", feature = "display")))]
fn main() {}
