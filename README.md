This repo contains:
- [`toml` crate](./crates/toml) for serde support
- [`toml_edit` crate](./crates/toml_edit) for format-preserving editing of TOML
- [`toml_datetime` crate](./crates/toml_datetime) for a common type definition between `toml` and `toml_edit`
- [`serde_spanned` crate](./crates/serde_spanned) for capturing spans when deserializing keys and values
- [`toml_parser` crate](./crates/toml_parser): a low-level format-preserving TOML lexer and parser
- [`toml_writer` crate](./crates/toml_writer): a low-level interface for writing out TOML
