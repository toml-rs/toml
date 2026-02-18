mod decoder;

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

    let invalid_ext = walkdir::WalkDir::new("tests/fixtures/invalid")
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
            toml_test_data::Invalid { name, fixture }
        })
        .collect::<Vec<_>>();

    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::DecoderHarness::new(decoder);
    harness.version("1.1.0");
    harness.ignore([]).unwrap();
    harness.snapshot_root("tests/snapshots");
    harness.extend_valid(valid_ext);
    harness.extend_invalid(invalid_ext);
    harness.test();
}
