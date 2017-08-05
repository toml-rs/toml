#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate toml_edit;

use toml_edit::Document;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let doc = data.parse::<Document>();
        if let Ok(doc) = doc {
            assert_eq!(doc.to_string(), data);
        }
    }
});
