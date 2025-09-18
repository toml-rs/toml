# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.1] - 2025-09-15

### Performance

- Allow more build parallelism by depending on [`serde_core`](https://crates.io/crates/serde_core)

## [1.0.0] - 2025-07-08

### Compatibility

Breaking changes

- Serde support has been broken out into the default `serde` feature
- Std support has been broken out into the default `std` feature

Other

- Loosened requirements for identifying a `Spanned` during deserialization

### Features

- Added `de::SpannedDeserializer` / `de::is_spanned` for easier integration with `Deserializer`s
- Added `impl Display for Spanned<impl Display>`
- Added `impl Borrow for Spanned<Cow<B>>`

## [0.6.9] - 2025-06-06

## [0.6.8] - 2024-09-25

### Fixes

- Loosen the order that fields can be emitted by a deserializer

## [0.6.7] - 2024-07-25

## [0.6.6] - 2024-05-15

## [0.6.5] - 2023-12-19

### Features

- Add `Spanned::new`

### Documentation

- Show how to transpose a `Spanned<Enum<T>>` into `Enum<Spanned<T>>`

## [0.6.4] - 2023-10-23

## [0.6.3] - 2023-06-24

## [0.6.2] - 2023-05-18

### Compatibility

MSRV is now 1.64.0

## [0.6.1] - 2023-01-30

### Documentation

- Show features on doc.rs

## [0.6.0] - 2023-01-20

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml/compare/serde_spanned-v1.0.1...HEAD
[1.0.1]: https://github.com/toml-rs/toml/compare/serde_spanned-v1.0.0...serde_spanned-v1.0.1
[1.0.0]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.9...serde_spanned-v1.0.0
[0.6.9]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.8...serde_spanned-v0.6.9
[0.6.8]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.7...serde_spanned-v0.6.8
[0.6.7]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.6...serde_spanned-v0.6.7
[0.6.6]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.5...serde_spanned-v0.6.6
[0.6.5]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.4...serde_spanned-v0.6.5
[0.6.4]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.3...serde_spanned-v0.6.4
[0.6.3]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.2...serde_spanned-v0.6.3
[0.6.2]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.1...serde_spanned-v0.6.2
[0.6.1]: https://github.com/toml-rs/toml/compare/serde_spanned-v0.6.0...serde_spanned-v0.6.1
[0.6.0]: https://github.com/toml-rs/toml/compare/205859ff8c88fcc351ca55abc08139a6785fd075...serde_spanned-v0.6.0
