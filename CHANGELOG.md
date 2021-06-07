# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [Unreleased]

## [0.2.1] - 2021-06-07
- Added `Table::decor`. [#97](https://github.com/ordian/toml_edit/pull/97)
- Added `IterMut` for `Table`. [#100](https://github.com/ordian/toml_edit/pull/100)
- Added `Table::get_mut`. [#106](https://github.com/ordian/toml_edit/pull/106)
- Updated `combine` to 4.5. [#107](https://github.com/ordian/toml_edit/pull/107)

## [0.2.0] - 2020-06-18
- Added format preserving mutation functions for `Array`. [#88](https://github.com/ordian/toml_edit/pull/88)
### Breaking
- `array.push` now returns a `Result`.

