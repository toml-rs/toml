#![recursion_limit = "256"]
#![allow(clippy::dbg_macro)]
#![cfg(all(feature = "parse", feature = "display"))]

macro_rules! parse_value {
    ($s:expr) => {{
        use serde::Deserialize;
        let v = toml::Value::deserialize(toml::de::ValueDeserializer::new($s));
        assert!(
            v.is_ok(),
            "Failed with `{}` when parsing:
```
{}
```
",
            v.unwrap_err(),
            $s
        );
        v.unwrap()
    }};
}

mod invalid;
mod parse;

use toml::Table as RustDocument;
use toml::Value as RustValue;
