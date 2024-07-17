#![no_main]

use toml_edit::DocumentMut;

libfuzzer_sys::fuzz_target!(|data: &[u8]| -> libfuzzer_sys::Corpus {
    let Ok(data) = std::str::from_utf8(data) else {
        return libfuzzer_sys::Corpus::Reject;
    };

    let doc = match data.parse::<DocumentMut>() {
        Ok(doc) => doc,
        Err(err) => {
            println!(
                "parse error: {err}

data:
```toml
{data}
```
"
            );
            return libfuzzer_sys::Corpus::Keep;
        }
    };
    let toml = doc.to_string();
    let doc = toml.parse::<DocumentMut>();
    assert!(
        doc.is_ok(),
        "parse error: {}

data:
```toml
{data}
```

doc.to_string():
```toml
{}
```",
        doc.unwrap_err(),
        toml
    );
    let doc = doc.unwrap();
    assert_eq!(
        doc.to_string(),
        toml,
        "data:
```toml
{data}
```
"
    );
    libfuzzer_sys::Corpus::Keep
});
