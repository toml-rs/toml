mod decoder;

fn main() {
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
    harness.version("1.0.0");
    harness
        .ignore([
            "invalid/string/basic-byte-escapes.toml",
            "invalid/datetime/no-secs.toml",
            "invalid/local-datetime/no-secs.toml",
            "invalid/local-time/no-secs.toml",
            "invalid/inline-table/linebreak-01.toml",
            "invalid/inline-table/linebreak-02.toml",
            "invalid/inline-table/linebreak-03.toml",
            "invalid/inline-table/linebreak-04.toml",
            "invalid/inline-table/trailing-comma.toml",
        ])
        .unwrap();
    harness.snapshot_root("tests/snapshots");
    harness.extend_invalid(invalid_ext);
    harness.test();
}
