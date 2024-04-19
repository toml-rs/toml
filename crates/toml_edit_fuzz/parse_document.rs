#![no_main]

use toml_edit::Document;

libfuzzer_sys::fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        println!("parsing: {data:?}");
        let doc = data.parse::<Document>();
        if let Ok(doc) = doc {
            let toml = doc.to_string();
            println!("parsing: {toml:?}");
            let doc = toml.parse::<Document>();
            assert!(
                doc.is_ok(),
                "Failed to parse `doc.to_string()`: {}\n```\n{}\n```",
                doc.unwrap_err(),
                toml
            );
            let doc = doc.unwrap();
            assert_eq!(doc.to_string(), toml);
        }
    }
});
