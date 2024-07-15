#![no_main]

use toml_edit::DocumentMut;

libfuzzer_sys::fuzz_target!(|data: &[u8]| -> libfuzzer_sys::Corpus {
    let Ok(data) = std::str::from_utf8(data) else {
        return libfuzzer_sys::Corpus::Reject;
    };

    println!("parsing: {data:?}");
    let doc = match data.parse::<DocumentMut>() {
        Ok(doc) => doc,
        Err(err) => {
            println!("{err}");
            return libfuzzer_sys::Corpus::Keep;
        }
    };
    let toml = doc.to_string();
    println!("parsing: {toml:?}");
    let doc = toml.parse::<DocumentMut>();
    assert!(
        doc.is_ok(),
        "Failed to parse `doc.to_string()`: {}\n```\n{}\n```",
        doc.unwrap_err(),
        toml
    );
    let doc = doc.unwrap();
    assert_eq!(doc.to_string(), toml);
    libfuzzer_sys::Corpus::Keep
});
