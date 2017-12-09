# toml_edit

[![Build Status](https://img.shields.io/travis/ordian/toml_edit/master.svg?label=linux%20%26%20osx)](https://travis-ci.org/ordian/toml_edit)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/ordian/toml_edit?svg=true)](https://ci.appveyor.com/project/ordian/toml-edit/branch/master)
[![codecov](https://codecov.io/gh/ordian/toml_edit/branch/master/graph/badge.svg)](https://codecov.io/gh/ordian/toml_edit)
[![crates.io](https://img.shields.io/crates/v/toml_edit.svg)](https://crates.io/crates/toml_edit)
[![docs](https://docs.rs/toml_edit/badge.svg)](https://docs.rs/toml_edit/badge.svg)
[![Join the chat at https://gitter.im/toml_edit/Lobby](https://badges.gitter.im/a.svg)](https://gitter.im/toml_edit/Lobby)


This crate allows you to parse and modify toml
documents, while preserving comments, spaces* and
relative order* or items.

*Things it does not preserve:
1. Spaces in headers, e.g. `[ ' a ' .  b ]` will be represented as `[' a '.b]`.
2. Children tables before parent table (tables are reordered, see [test]).
3. Scattered array of tables (tables are reordered, see [test]).

`toml_edit` is primarily tailored for [cargo-edit](https://github.com/killercup/cargo-edit/) needs.

## Example

```rust
extern crate toml_edit;

use toml_edit::{Document, value};

fn main() {
    let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
    "#;
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml);
    doc["a"]["b"]["c"]["d"] = value("hello");
    // autoformat inline table a.b.c
    doc["a"]["b"]["c"].as_value_mut().and_then(|v| v.as_inline_table_mut().map(|t| t.fmt()));
    let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
    "#;
    assert_eq!(doc.to_string(), expected);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[test]: https://github.com/ordian/toml_edit/blob/f09bd5d075fdb7d2ef8d9bb3270a34506c276753/tests/test_valid.rs#L84
