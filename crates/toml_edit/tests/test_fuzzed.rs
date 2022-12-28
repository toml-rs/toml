use toml_edit::Document;

fn main() {
    snapbox::harness::Harness::new(
        "tests/fixtures/fuzzed",
        move |input_path| {
            let name = input_path.file_name().unwrap().to_str().unwrap().to_owned();
            let expected = input_path.with_extension("stderr");
            snapbox::harness::Case {
                name,
                expected,
                fixture: input_path,
            }
        },
        move |input_path| {
            let raw = std::fs::read_to_string(input_path).map_err(|e| e.to_string())?;
            let content = match raw.parse::<Document>() {
                Ok(doc) => format!("Passed: {:?}", doc),
                Err(err) => err.to_string(),
            };
            Ok::<_, String>(content)
        },
    )
    .select(["*.toml"])
    .action_env("FUZZED_TOML")
    .test()
}
