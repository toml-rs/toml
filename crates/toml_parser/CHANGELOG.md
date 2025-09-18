# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.2...HEAD
[1.0.2]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.1...toml_parser-v1.0.2
[1.0.1]: https://github.com/toml-rs/toml/compare/toml_parser-v1.0.0...toml_parser-v1.0.1
[1.0.0]: https://github.com/toml-rs/toml/compare/e5b281ad...toml_parser-v1.0.0
