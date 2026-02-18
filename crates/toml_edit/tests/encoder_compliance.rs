mod decoder;
mod encoder;

fn main() {
    let valid_ext = walkdir::WalkDir::new("tests/fixtures/valid")
        .sort_by_file_name()
        .into_iter()
        .map(Result::unwrap)
        .filter(|e| e.path().extension() == Some(std::ffi::OsStr::new("toml")))
        .map(|e| {
            let name = e
                .path()
                .strip_prefix("tests/fixtures")
                .unwrap()
                .to_owned()
                .into();
            let fixture = std::fs::read(e.path()).unwrap().into();
            let expected_path = e.path().with_extension("json");
            let expected = std::fs::read(expected_path).unwrap().into();
            toml_test_data::Valid {
                name,
                fixture,
                expected,
            }
        })
        .collect::<Vec<_>>();

    let encoder = encoder::Encoder;
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness.version("1.1.0");
    harness.extend_valid(valid_ext);
    harness.test();
}
