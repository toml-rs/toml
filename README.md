This repo contains:
- [`toml` crate](./crates/toml) for serde support
- [`toml_edit` crate](./crates/toml_edit) for format-preserving editing of TOML
- [`toml_datetime` crate](./crates/toml_datetime) for a common type definition between `toml` and `toml_edit`
- [`toml_parser` crate](./crates/toml_parser) a dependency of `toml` and `toml_edit` crates, which does zero-copy parsing
