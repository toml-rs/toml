use toml_edit::DocumentMut;

fn main() {
    let ext = walkdir::WalkDir::new("tests/fixtures/invalid")
        .sort_by_file_name()
        .into_iter()
        .map(Result::unwrap)
        .filter(|e| e.path().extension() == Some(std::ffi::OsStr::new("toml")))
        .map(|e| {
            let name = e.path().strip_prefix("tests/fixtures").unwrap().to_owned();
            let fixture = std::fs::read(e.path()).unwrap();
            libtest_mimic::Trial::test(name.display().to_string(), move || {
                let expect_path =
                    std::path::Path::new("tests/snapshots").join(name.with_extension("stderr"));
                let err = match run_case(&fixture) {
                    Ok(()) => "".to_owned(),
                    Err(err) => err,
                };
                snapbox::assert_data_eq!(err, snapbox::Data::read_from(&expect_path, None).raw());
                Ok(())
            })
        })
        .collect::<Vec<_>>();

    let args = libtest_mimic::Arguments::from_args();
    let tests = toml_test_data::invalid()
        .map(|case| {
            let name = case.name;
            let fixture = case.fixture;
            libtest_mimic::Trial::test(name.display().to_string(), || {
                let expect_path =
                    std::path::Path::new("tests/snapshots").join(name.with_extension("stderr"));
                let err = match run_case(fixture) {
                    Ok(()) => "".to_owned(),
                    Err(err) => err,
                };
                snapbox::assert_data_eq!(err, snapbox::Data::read_from(&expect_path, None).raw());
                Ok(())
            })
        })
        .chain(ext)
        .collect();
    libtest_mimic::run(&args, tests).exit()
}

fn run_case(input: &[u8]) -> Result<(), String> {
    let raw = std::str::from_utf8(input).map_err(|e| e.to_string())?;
    let _ = raw.parse::<DocumentMut>().map_err(|e| e.to_string())?;
    Ok(())
}
