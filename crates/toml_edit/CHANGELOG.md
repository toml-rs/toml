# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.15.0] - 2022-10-21

### Breaking Changes

- `Datetime`'s `FromStr::Err` changed from `toml_edit::Error` to `toml_edit::DatetimeParseError`
- `Datetime` no longer implements `TryFrom`
- `Time` no longer implements `FromStr`, `TryFrom`, `Serialize`, or `Deserialize`
- `Date` no longer implements `FromStr`, `TryFrom`, `Serialize`, or `Deserialize`

### Fixes

- Remove leading newlines in the document with default table decor

## [0.14.4] - 2022-05-09

### Fixes

- Allow enum variants as table keys

## [0.14.3] - 2022-04-26

### Features

Tables
- Added `OccupiedEntry::key_mut` and `InlineOccupiedEntry::key_mut`
- Added `TableLike::entry` and `TableLike::entry`
- Added `get_key_value()` and `get_key_value_mut()` to `TableLike`, `Table`, and `InlineTable`

## [0.14.2] - 2022-03-30

### Fixes

- Make `perf` opt-in

## [0.14.1] - 2022-03-30

### Features

- make performance-specific dependencies optional with the `perf` feature (default)

## [0.14.0] - 2022-03-29

### Performance

- Upgrade to faster `kstring`

## [0.13.4] - 2022-01-31

### Fixes

- Have `toml_edit::easy::to_string_pretty` render empty tables

## [0.13.3] - 2022-01-28

### Fixes

- `toml_edit::value` now uses default decor

## [0.13.2] - 2022-01-27

### Features

- Allowing clearing table-likes

## [0.13.1] - 2022-01-26

### Features

- Programmatically expose line and column for some errors

### Performance

- Slight performance improvement with comments

## [0.13.0] - 2022-01-13

### Breaking Changes

- `iter`, `get`, and `contains_key` functions were made consistent across `Table` and `InlineTable`, ignoring `Item::None`.

### Fixes

- Reduce places users need to handle both `None` and `Item::None`, making the APIs more consistent across the board
- Remove a quote around a non-literal value in an error

## [0.12.6] - 2022-01-12

### Fixes

- Expose `Document::as_item`

## [0.12.5] - 2022-01-12

### Fixes

- Errors now only quote literals

## [0.12.4] - 2022-01-07

### Fixes

- Fix `Table::set_position` documentation so its clear it always applies

## [0.12.3] - 2021-12-31

### Fixes

- `to_string_pretty` now hides empty tables

## [0.12.2] - 2021-12-30

### Fixes

- Cleaned up several error messages
- `toml_edit::ser::to_string_pretty` is now pretty

## [0.12.1] - 2021-12-28

### Features

- Added `Table::sort_values_by` and `InlineTable::sort_values_by`

### Fixes

- Clarified error message when accidentally using bare words for values

## [0.12.0] - 2021-12-14

### Fixes

- Serde trait bounds switched from `serde::Deserialize<'static>` to `serde::de::DeserializeOwned`

### Breaking Changes

- Serde trait bounds switched from `serde::Deserialize<'static>` to `serde::de::DeserializeOwned`

## [0.11.0] - 2021-12-14

### Features

- Use `Key::parse` to parse a string of dotted keys into a `Vec<Key>`

### Fixes

- `Key::from_str` now strictly parses TOML syntax

### Breaking Changes

- `Key::from_str` now strictly parses TOML syntax

## [0.10.1] - 2021-12-01

### Fixes

- Allow trailing whitespace after dates
- Truncate overflowing fractional seconds rather than error (we will still roundtrip the original time)

## [0.10.0] - 2021-11-25

### Breaking Changes

- `TableLike::fmt` now resets the decor to default (`None`) rather than assigning the default decor
- Converting between table types clears formatting

### Features

- `Key` now derefs to the key's value
- Allow modifying key formatting with `iter_mut()`
- New `visit` and `visit_mut` APIs

### Fixes

- `Value::try_from` and `Value::try_into` to work with all types
- Don't fail on UTF-8 BOM
- Ensure there is a trailing space for default-formatted inline tables
- Converting between table types clears formatting
- `Array::fmt` removes trailing comma and whitespace

## [0.9.1] - 2021-11-15

### Features

- Allow indexing on `InlineTable`

### Fixes

- serde support for newtypes
- Don't error on `easy::Value::to_string`

## [0.9.0] - 2021-11-15

### Breaking Changes

- Some types in `toml_edit::ser` got shuffled around.

### Features

- Added `toml_edit::ser::to_item` for converting any serializable state to a `toml_edit::Item`
- Added `toml_edit::InlineTable::into_table`
- Added `toml_edit::Document` now has a `From<Table>` impl.

### Fixes

- `toml_edit::Item::into_table` now includes `InlineTable`

## [0.8.0] - 2021-11-02

### Breaking Changes

- Disallow the direct creation of `toml_edit::ser::Serializer` so we can change it in the future.

### Fixes

- Decouple serde support from `easy` feature
- Make core types impl `Deserializer`, making it easier to use them
- Make core types impl `Display` so its easier to print errors to users

## [0.7.0] - 2021-11-02

### Breaking Changes

- `Document::root` is now private
- The `Index` implementation for `Item` now panics when the index is not found
  - Use `Item::get` and `Item::get_mut` instead

#### Features

- `Document` now derefs to `Table` for easier access
- `Item` now has `get` / `get_mut` like `easy::Value`

#### Fixes

- Clarified role of `toml_edit::easy`

## [0.6.0] - 2021-10-14

#### Features

- Add `TableLike::set_dotted` so you can make a table dotted, independent of its type

#### Fixes

- Allow dotted inline-tables in standard tables

#### Breaking Changes

- `toml_edit::TableLike` is now sealed, disallowing others to implement it

## [0.5.0] - 2021-09-30

#### Performance

| toml_edit | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|-----------|-------------------------|--------------------|
| HEAD      | 4.0 us                  | 149 us             |

| toml_edit::easy | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|-----------------|-------------------------|--------------------|
| v0.4.0          | 16.9 us                 | 602 us             |
| HEAD            | 5.0 us                  | 179 us             |

| toml    | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|---------|-------------------------|--------------------|
| v0.5.8  | 4.7 us                  | 121 us             |

- Removed ambiguity between `String` and `Datetime` when deserializing
- Hand implemented `Deserialize` for `toml_edit::easy::Value` to dispatch on type, rather than trying every variant.

#### Breaking Changes

- `Datetime` is no longer a string in `serde`s data model but a proprietary type.

## [0.4.0] - 2021-09-29

#### Breaking Changes

- Changed some strings callers generally don't interact with (e.g.
  `Into<String>`) to an opaque type, allowing us to change how we allocate most
  strings without requiring breaking changes in the future.
  - This impacts `Key`, `Repr`, and `Decor`
  - This does not impact `Value`, assuming people want a familiar type over performance

#### Fixes

- Support trailing quotes in strings

#### Performance

| toml_edit | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|-----------|-------------------------|--------------------|
| v0.3.1    | 8.7 us                  | 271 us             |
| HEAD      | 4.1 us                  | 150 us             |

| toml_edit::easy | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|-----------------|-------------------------|--------------------|
| v0.3.1          | 21.2 us                 | 661 us             |
| HEAD            | 18.6 us                 | 630 us             |

| toml    | `cargo init` Cargo.toml | Cargo's Cargo.toml |
|---------|-------------------------|--------------------|
| v0.5.8  | 4.8 us                  | 125 us             |

Changes include:
- Batch create strings
- Small-string optimization
- Removed superfluous allocations
- Switched from recursion to looping
- Avoid decoding bytes to `char`
- Optimized grammar selection rules which also reduced allocations further

## [0.3.1] - 2021-09-14

#### Fixes

- Sane default formatting for arrays

## [0.3.0] - 2021-09-13

- Added support for TOML 1.0, with [one functional caveat](https://github.com/toml-rs/toml_edit/issues/128) and [one format-preserving caveat](https://github.com/toml-rs/toml_edit/issues/163)
- Added `Item::into_value`
- Changed `Table` and `InlineTable` to be more Map-like
- Expanded support in `TableLike`
- Added [toml-rs](https://docs.rs/toml)-compatible API via the `toml_edit::easy` module for when developers want to ensure consistency between format-preserving and general TOML work, with [one caveat](https://github.com/toml-rs/toml_edit/issues/192).
- Exposed more control over formatting, with ability to modify any key or value whitespace.
- Fixed it so we preserve formatting on dotted keys in standard table headers
- Dropped `chrono` dependency

This release was sponsored by Futurewei

## [0.2.1] - 2021-06-07
- Added `Table::decor`. [#97](https://github.com/toml-rs/toml_edit/pull/97)
- Added `IterMut` for `Table`. [#100](https://github.com/toml-rs/toml_edit/pull/100)
- Added `Table::get_mut`. [#106](https://github.com/toml-rs/toml_edit/pull/106)
- Updated `combine` to 4.5. [#107](https://github.com/toml-rs/toml_edit/pull/107)

## 0.2.0 - 2020-06-18
- Added format preserving mutation functions for `Array`. [#88](https://github.com/toml-rs/toml_edit/pull/88)
### Breaking
- `array.push` now returns a `Result`.

<!-- next-url -->
[Unreleased]: https://github.com/toml-rs/toml_edit/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/toml-rs/toml_edit/compare/v0.14.4...v0.15.0
[0.14.4]: https://github.com/toml-rs/toml_edit/compare/v0.14.3...v0.14.4
[0.14.3]: https://github.com/toml-rs/toml_edit/compare/v0.14.2...v0.14.3
[0.14.2]: https://github.com/toml-rs/toml_edit/compare/v0.14.1...v0.14.2
[0.14.1]: https://github.com/toml-rs/toml_edit/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/toml-rs/toml_edit/compare/v0.13.4...v0.14.0
[0.13.4]: https://github.com/toml-rs/toml_edit/compare/v0.13.3...v0.13.4
[0.13.3]: https://github.com/toml-rs/toml_edit/compare/v0.13.2...v0.13.3
[0.13.2]: https://github.com/toml-rs/toml_edit/compare/v0.13.1...v0.13.2
[0.13.1]: https://github.com/toml-rs/toml_edit/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/toml-rs/toml_edit/compare/v0.12.6...v0.13.0
[0.12.6]: https://github.com/toml-rs/toml_edit/compare/v0.12.5...v0.12.6
[0.12.5]: https://github.com/toml-rs/toml_edit/compare/v0.12.4...v0.12.5
[0.12.4]: https://github.com/toml-rs/toml_edit/compare/v0.12.3...v0.12.4
[0.12.3]: https://github.com/toml-rs/toml_edit/compare/v0.12.2...v0.12.3
[0.12.2]: https://github.com/toml-rs/toml_edit/compare/v0.12.1...v0.12.2
[0.12.1]: https://github.com/toml-rs/toml_edit/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/toml-rs/toml_edit/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/toml-rs/toml_edit/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/toml-rs/toml_edit/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/toml-rs/toml_edit/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/toml-rs/toml_edit/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/toml-rs/toml_edit/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/toml-rs/toml_edit/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/toml-rs/toml_edit/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/toml-rs/toml_edit/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/toml-rs/toml_edit/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/toml-rs/toml_edit/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/toml-rs/toml_edit/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/toml-rs/toml_edit/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/toml-rs/toml_edit/compare/v0.2.0...v0.2.1
