# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Compatibility

- Bumped MSRV to 1.60.0
- Deprecated  `Deserializer::set_require_newline_after_table`
- Deprecated  `Deserializer::set_allow_duplicate_after_longer_table`

## [0.5.9]

Changes:

- #373: Allow empty table keys
- #426: Fix serialization of -0.0
- #439: Make datetime structs and fields public

## [0.5.8]

Minor doc fix (#409)

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml_edit/compare/70caf40...HEAD
[0.5.9]: https://github.com/toml-rs/toml_edit/compare/94b319f...70caf40
[0.5.8]: https://github.com/toml-rs/toml_edit/compare/9a94610...94b319f
