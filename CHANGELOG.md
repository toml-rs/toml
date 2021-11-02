# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.8.0] - 2021-11-02

### Breaking Changes

- Disallow the direct creation of `toml_edit::ser::Serializer` so we can change it in the future.

#### Fixes

- Decouple serde support from `easy` feature
- Make core types impl `Deserializer`, making it easier to use them
- Make core types impl `Display` so its easier to print errors to users

## [0.7.0] - 2021-11-02

### Breaking Changes

- `Document::root` is now private

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

- Added support for TOML 1.0, with [one functional caveat](https://github.com/ordian/toml_edit/issues/128) and [one format-preserving caveat](https://github.com/ordian/toml_edit/issues/163)
- Added `Item::into_value`
- Changed `Table` and `InlineTable` to be more Map-like
- Expanded support in `TableLike`
- Added [toml-rs](https://docs.rs/toml)-compatible API via the `toml_edit::easy` module for when developers want to ensure consistency between format-preserving and general TOML work, with [one caveat](https://github.com/ordian/toml_edit/issues/192).
- Exposed more control over formatting, with ability to modify any key or value whitespace.
- Fixed it so we preserve formatting on dotted keys in standard table headers
- Dropped `chrono` dependency

This release was sponsored by Futurewei

## [0.2.1] - 2021-06-07
- Added `Table::decor`. [#97](https://github.com/ordian/toml_edit/pull/97)
- Added `IterMut` for `Table`. [#100](https://github.com/ordian/toml_edit/pull/100)
- Added `Table::get_mut`. [#106](https://github.com/ordian/toml_edit/pull/106)
- Updated `combine` to 4.5. [#107](https://github.com/ordian/toml_edit/pull/107)

## 0.2.0 - 2020-06-18
- Added format preserving mutation functions for `Array`. [#88](https://github.com/ordian/toml_edit/pull/88)
### Breaking
- `array.push` now returns a `Result`.

<!-- next-url -->
[Unreleased]: https://github.com/ordian/toml_edit/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/ordian/toml_edit/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/ordian/toml_edit/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ordian/toml_edit/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ordian/toml_edit/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ordian/toml_edit/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/ordian/toml_edit/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/ordian/toml_edit/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/ordian/toml_edit/compare/v0.2.0...v0.2.1
