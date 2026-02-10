# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.7] - 2026-02-10

### Fixes

- Don't panic on integers with a radix, an underscore, and a non-digit character

## [1.0.6] - 2025-12-18

### Features

- TOML 1.1 support
  - multi-line inline tables
  - trailing commas on inline tables
  - `\e` string escape character
  - `\xHH` string escape character

## [1.0.5] - 2025-12-17

## [1.0.4] - 2025-10-09

## [1.0.3] - 2025-09-18

### Compatibility

- Update MSRV to 1.76

### Internal

- Update dependencies

## [1.0.2] - 2025-08-04

### Fixes

- Improve missing-open-quote errors
- Don't treat trailing quotes as separate items
- Conjoin more values in unquoted string errors
- Reduce float false positives
- Reduce float/bool false positives

## [1.0.1] - 2025-07-11

### Fixes

- Fix infinite loop when `)` is present outside of quotes

## [1.0.0] - 2025-07-08

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.7...HEAD
[1.0.7]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.6...toml_parser-v1.0.7
[1.0.6]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.5...toml_parser-v1.0.6
[1.0.5]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.4...toml_parser-v1.0.5
[1.0.4]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.3...toml_parser-v1.0.4
[1.0.3]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.2...toml_parser-v1.0.3
[1.0.2]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.1...toml_parser-v1.0.2
[1.0.1]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.0...toml_parser-v1.0.1
[1.0.0]: https://github.com/toml-rs/toml/compare/e5b281ad...toml_parser-v1.0.0
