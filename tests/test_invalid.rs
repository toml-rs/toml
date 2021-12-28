use toml_edit::Document;

fn main() {
    let action = std::env::var("INVALID_TOML");
    let action = action.as_deref().unwrap_or("verify");
    let action = match action {
        "overwrite" => Action::Overwrite,
        "ignore" => Action::Ignore,
        "verify" => Action::Verify,
        _ => panic!(
            "Unrecognized action {}, expected `overwrite`, `ignore`, or `verify`",
            action
        ),
    };

    fs_snapshot::Harness::new(
        "tests/fixtures/invalid",
        move |input_path| {
            let name = input_path.file_name().unwrap().to_str().unwrap().to_owned();
            let expected = input_path.with_extension("stderr");
            fs_snapshot::Test {
                name,
                kind: "".into(),
                is_ignored: action == Action::Ignore,
                is_bench: false,
                data: fs_snapshot::Case {
                    fixture: input_path,
                    expected,
                },
            }
        },
        move |input_path| {
            let raw = std::fs::read_to_string(input_path).map_err(|e| e.to_string())?;
            match raw.parse::<Document>() {
                Ok(_) => Err("Parsing unexpectedly succeeded".to_owned()),
                Err(err) => Ok(err.to_string()),
            }
        },
    )
    .select(["*.toml"])
    .overwrite(action == Action::Overwrite)
    .test()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Action {
    Overwrite,
    Verify,
    Ignore,
}
