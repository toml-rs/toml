# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Fixes

- Prevent a stack overflow when parsing very large files

## [0.8.17] - 2024-07-30

### Performance

- Speed up whitespace parsing
- Speed up empty array parsing
- Speed up general array parsing
- Speed up general value parsing

### Features

- impl Serialize/Deserialize for Date/Time

### Fixes

- When recursion depth is reached, be sure to show that error rather than something else

## [0.8.16] - 2024-07-25

## [0.8.15] - 2024-07-17

### Fixes

- Correctly encode TOML keys with mixed quotes

## [0.8.14] - 2024-06-03

### Fixes

- Allow inferring keys as string literals
- Prefer string literals if it avoids escaping double-quotes

## [0.8.13] - 2024-05-15

## [0.8.12] - 2024-03-18

### Fixes

- Drop recursion limit from 128 to 100 to work on `opt-level = 0` builds

## [0.8.11] - 2024-03-11

### Performance

- *(de)* Remove an allocation when parsing

### Compatibility

MSRV is now 1.70

## [0.8.10] - 2024-02-05

### Internal

- Update `toml_edit` dependency

## [0.8.9] - 2024-01-31

### Fixes

- *(de)* Improve error span for empty tables

### Compatibility

MSRV is now 1.69

## [0.8.8] - 2023-11-06

### Compatibility

- If you relied on `toml` to enable `toml_edit` `parse` or `display` features, it will no longer work

## [0.8.7] - 2023-11-06

## [0.8.6] - 2023-10-27

### Fixes

- *(ser)* Make sign of `nan` deterministic by always being positive

## [0.8.5] - 2023-10-26

### Fixes

- *(parser)* Ensure the sign of NAN is preserved
- *(serde)* Ensure the sign of NAN is preserved

## [0.8.4] - 2023-10-23

### Fixes

- *(parser)* Error on invalid days of month, accounting for leap years

## [0.8.3] - 2023-10-23

### Compatibility

MSRV is now 1.67

## [0.8.2] - 2023-10-03

### Fixes

- *(parser)* Correctly error when mixing inline tables with inline dotted keys

## [0.8.1] - 2023-09-26

### Fixes

- *(de)* Allow parsing keys into newtypes

## [0.8.0] - 2023-09-13

### Compatibility

- Serialization and deserialization of tuple variants has changed from being an array to being a table with the key being the variant name and the value being the array

### Fixes

- Consistently serialize and deserialize struct and tuple variants, matching serde_json's behavior

## [0.7.8] - 2023-09-09

### Fixes

- *(ser)* Don't lose data when inline tables are nested deeply under arrays

## [0.7.7] - 2023-09-08

### Fixes

- *(ser)* Error rather than drop whole arrays when a single element is `None`

### Compatibility

MSRV is now 1.66.0

## [0.7.6] - 2023-07-05

### Features

- Add `Map::retain`

## [0.7.5] - 2023-06-24

### Internal

- Update `indexmap`

## [0.7.4] - 2023-05-18

### Features

- *(ser)* Newtype variant support

### Compatibility

MSRV is now 1.64.0

## [0.7.3] - 2023-03-08

### Fixes

- Don't skip writing standard tables that are "underneath" dotted keys

## [0.7.2] - 2023-02-07

### Fixes

- *(ser)* Error on i64 overflow

## [0.7.1] - 2023-01-30

### Documentation

- Show features on doc.rs

## [0.7.0] - 2023-01-27

### Breaking Change

- `Offset::Custom` changed from tracking hours+minutes to minutes
- `Offset::Custom`s parser now enforces a range of minutes
- Removed deprecated `Error::line_col` infavor of `Error::span`
- Removed deprecated `ser::tables_last` as it isn't needed anymore
- Removed deprecagted `Serializer::pretty_*` functions as `toml_edit` is for greater customization

### Fixes

- Allow negative minute `Offset`s

## [0.6.0] - 2023-01-23

### Compatibility

Breaking Behavior Changes
- `FromStr` impl for `Value` now only parses toml values, not documents.  See instead `Table`
- `Display` impl for `Value` now only renders toml values, not documents.  See instead `Table`
- `from_str` now only parses toml documents, not values.  See instead `de::ValueDeserializer`
- `to_string` / `to_string_pretty` now only renders toml documents, not values.  See instead `ser::ValueSerializer`

Breaking API Changes
- `de::from_slice` and `ser::to_vec` were removed, instead use `from_str` and `to_string` and convert with bytes manually
- `toml!` returns a `Table`, rather than a `Value`
- `toml::de` can  no longer deserialize to borrowed types as everything becomes owned through the parsing process
- `serde::de::Deserializer` is now implemented for `toml::Deserializer`, rather than `&mut toml::Deserializer`
- `serde::ser::Serializer` is now implemented for `toml::Serializer`, rather than `&mut toml::Serializer`
- `value` no longer re-exports `Map` and `Entry`
- `Spanned::span`s return type changed to `std::ops::Range<usize>`
- `Spanned::start` removed in favor of `Spanned::span().start`
- `Spanned::wnd` removed in favor of `Spanned::span().end`
- `parse` and `display` default features were added
- `toml::ser::Error`s variants are private.  For `toml::ser::Error::Custom`, you can use `serde::ser::Error::custom`
- Deprecated items from 0.5 were removed

Deprecations
- Deprecated `Deserializer::tables_last`, it is no longer needed
- Deprecated `Deserializer::pretty_string`, this is all bundled up with `pretty`
- Deprecated `Deserializer::pretty_string_literal`, this is all bundled up with `pretty`
- Deprecated `Deserializer::pretty_array`, this is all bundled up with `pretty`
- Deprecated `Deserializer::pretty_array_indent`, this is all bundled up with `pretty`
- Deprecated `Deserializer::pretty_array_trailing_comma`, this is all bundled up with `pretty`
- Deprecated `de::Error::line_col`, replaced with `de::Error::span`

MSRV is now 1.60.0

### Features

- `Table` now impls `Display`, `FromStr`, `Deserialize`
- `Table` now has `try_from` and `try_into` methods
- Added `de::ValueDeserializer` and `ser::ValueSerializer` for exclusively working with toml values
- Provide `de::Error::span` and `de::Error::message`, allowing custom error rendering

### Fixes

- Bring the parser into full TOML 1.0 compliance
- *(parse)* Improve the quality of spans in `Spanned` / errors
- *(display)* Automatically handle root key-value pairs vs tables, removing the need for `tables_last`
- *(display)* Fix problems with stray `,` being inserted when using array of tables
- *(error)* Show the error locatation in the source for parse and deserialize errors

## [0.5.11] - 2023-01-20

### Compatibility

- Deprecated  `Deserializer::set_require_newline_after_table`
- Deprecated  `Deserializer::set_allow_duplicate_after_longer_table`
- Deprecated  `Deserializer::end`

## [0.5.10] - 2022-12-14

## [0.5.9]

Changes:

- #373: Allow empty table keys
- #426: Fix serialization of -0.0
- #439: Make datetime structs and fields public

## [0.5.8]

Minor doc fix (#409)

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml/compare/toml-v0.8.17...HEAD
[0.8.17]: https://github.com/toml-rs/toml/compare/toml-v0.8.16...toml-v0.8.17
[0.8.16]: https://github.com/toml-rs/toml/compare/toml-v0.8.15...toml-v0.8.16
[0.8.15]: https://github.com/toml-rs/toml/compare/toml-v0.8.14...toml-v0.8.15
[0.8.14]: https://github.com/toml-rs/toml/compare/toml-v0.8.13...toml-v0.8.14
[0.8.13]: https://github.com/toml-rs/toml/compare/toml-v0.8.12...toml-v0.8.13
[0.8.12]: https://github.com/toml-rs/toml/compare/toml-v0.8.11...toml-v0.8.12
[0.8.11]: https://github.com/toml-rs/toml/compare/toml-v0.8.10...toml-v0.8.11
[0.8.10]: https://github.com/toml-rs/toml/compare/toml-v0.8.9...toml-v0.8.10
[0.8.9]: https://github.com/toml-rs/toml/compare/toml-v0.8.8...toml-v0.8.9
[0.8.8]: https://github.com/toml-rs/toml/compare/toml-v0.8.7...toml-v0.8.8
[0.8.7]: https://github.com/toml-rs/toml/compare/toml-v0.8.6...toml-v0.8.7
[0.8.6]: https://github.com/toml-rs/toml/compare/toml-v0.8.5...toml-v0.8.6
[0.8.5]: https://github.com/toml-rs/toml/compare/toml-v0.8.4...toml-v0.8.5
[0.8.4]: https://github.com/toml-rs/toml/compare/toml-v0.8.3...toml-v0.8.4
[0.8.3]: https://github.com/toml-rs/toml/compare/toml-v0.8.2...toml-v0.8.3
[0.8.2]: https://github.com/toml-rs/toml/compare/toml-v0.8.1...toml-v0.8.2
[0.8.1]: https://github.com/toml-rs/toml/compare/toml-v0.8.0...toml-v0.8.1
[0.8.0]: https://github.com/toml-rs/toml/compare/toml-v0.7.8...toml-v0.8.0
[0.7.8]: https://github.com/toml-rs/toml/compare/toml-v0.7.7...toml-v0.7.8
[0.7.7]: https://github.com/toml-rs/toml/compare/toml-v0.7.6...toml-v0.7.7
[0.7.6]: https://github.com/toml-rs/toml/compare/toml-v0.7.5...toml-v0.7.6
[0.7.5]: https://github.com/toml-rs/toml/compare/toml-v0.7.4...toml-v0.7.5
[0.7.4]: https://github.com/toml-rs/toml/compare/toml-v0.7.3...toml-v0.7.4
[0.7.3]: https://github.com/toml-rs/toml/compare/toml-v0.7.2...toml-v0.7.3
[0.7.2]: https://github.com/toml-rs/toml/compare/toml-v0.7.1...toml-v0.7.2
[0.7.1]: https://github.com/toml-rs/toml/compare/toml-v0.7.0...toml-v0.7.1
[0.7.0]: https://github.com/toml-rs/toml/compare/toml-v0.6.0...toml-v0.7.0
[0.6.0]: https://github.com/toml-rs/toml/compare/70caf40...toml-v0.6.0
[0.5.11]: https://github.com/toml-rs/toml_edit/compare/toml-v0.5.10...toml-v0.5.11
[0.5.10]: https://github.com/toml-rs/toml_edit/compare/70caf40...toml-v0.5.10
[0.5.9]: https://github.com/toml-rs/toml_edit/compare/94b319f...70caf40
[0.5.8]: https://github.com/toml-rs/toml_edit/compare/9a94610...94b319f
