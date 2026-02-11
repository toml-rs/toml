# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.0] - 2026-02-11

### Fixes

- Wrap `Time::second` and `Time::nanosecond` in `Option`, preserving whether they are present or not

## [0.7.5] - 2025-12-18

### Features

- TOML 1.1 support
  - Make seconds optional when parsing (sets to `0`)

## [0.7.4] - 2025-12-17

## [0.7.3] - 2025-10-09

## [0.7.2] - 2025-09-18

### Compatibility

- Update MSRV to 1.76

### Internal

- Update dependencies

## [0.7.1] - 2025-09-15

### Performance

- Allow more build parallelism by depending on [`serde_core`](https://crates.io/crates/serde_core)

## [0.7.0] - 2025-07-08

### Compatibility

Breaking changes

- Std support has been broken out into the default `std` feature

### Features

- Added `de::DatetimeDeserializer` / `de::is_datetime` for easier integration with `Deserializer`s
- Added `ser::DatetimeSerializer` / `ser::is_datetime` for easier integration with `Serializer`s
- Added `de::VisitMap` for easier manual impls of untagged enums

## [0.6.11] - 2025-06-06

### Fixes

- Remove trailing space in `FromStr` error
- Use 2-digiti values for bounds in `FromStr` errors

## [0.6.10] - 2025-06-06

### Fixes

- Fix bounds checks in `FromStr`
- Improve `FromStr` error messages

## [0.6.9] - 2025-04-25

## [0.6.8] - 2024-07-30

### Features

- impl Serialize/Deserialize for Date/Time

## [0.6.7] - 2024-07-25

## [0.6.6] - 2024-05-15

## [0.6.5] - 2023-10-23

### Fixes

- Allow leapseconds in `FromStr`
- Error on invalid days of month, accounting for leap years, in `FromStr`

## [0.6.4] - 2023-10-23

## [0.6.3] - 2023-06-24

## [0.6.2] - 2023-05-18

### Compatibility

MSRV is now 1.64.0

## [0.6.1] - 2023-01-30

### Documentation

- Show features on doc.rs

## [0.6.0] - 2023-01-27

### Breaking Change

- `Offset::Custom` changed from tracking hours+minutes to minutes
- `Offset::Custom`s parser now enforces a range of minutes

### Fixes

- Allow negative minute `Offset`s

## [0.5.1] - 2023-01-20

## [0.5.0] - 2022-10-21

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml/compare/toml_datetime-v1.0.0...HEAD
[1.0.0]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.5...toml_datetime-v1.0.0
[0.7.5]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.4...toml_datetime-v0.7.5
[0.7.4]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.3...toml_datetime-v0.7.4
[0.7.3]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.2...toml_datetime-v0.7.3
[0.7.2]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.1...toml_datetime-v0.7.2
[0.7.1]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.7.0...toml_datetime-v0.7.1
[0.7.0]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.11...toml_datetime-v0.7.0
[0.6.11]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.10...toml_datetime-v0.6.11
[0.6.10]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.9...toml_datetime-v0.6.10
[0.6.9]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.8...toml_datetime-v0.6.9
[0.6.8]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.7...toml_datetime-v0.6.8
[0.6.7]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.6...toml_datetime-v0.6.7
[0.6.6]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.5...toml_datetime-v0.6.6
[0.6.5]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.4...toml_datetime-v0.6.5
[0.6.4]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.3...toml_datetime-v0.6.4
[0.6.3]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.2...toml_datetime-v0.6.3
[0.6.2]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.1...toml_datetime-v0.6.2
[0.6.1]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.6.0...toml_datetime-v0.6.1
[0.6.0]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.5.1...toml_datetime-v0.6.0
[0.5.1]: https://github.com/toml-rs/toml/compare/toml_datetime-v0.5.0...toml_datetime-v0.5.1
[0.5.0]: https://github.com/toml-rs/toml/compare/87741642c0f1a5217fd125e99fb52181869f74fa...toml_datetime-v0.5.0
