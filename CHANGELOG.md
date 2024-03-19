# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Removed

### Fixed

## [0.1.1]

### Added

- add `get_type` implementation for `Subject` struct

### Changed

- Change method argument order on `*_calls.rs` files
- Replace `identifier` method argument with `u64` on `event_calls.ts`
- Replace `identifier` method argument with `u64` on `group_calls.ts`
- Replace `identifier` method argument with `u64` on `report_calls.tsx`
- Deprecate `identifiers` for `Boost` and replace it with `Subject`
- Change `starred: HashMap<Identifier, String>` to `Vec<Subject>`
- change `get_starred_by_kind` to `get_starred_by_subject`

## [0.1.0] - 2024-03-15

### Added

- Changelog with versioning
- Canister http call to get current changelog `/changelog`
- Canister http call to get current version `/version`
- `#[allow(unused)]` to deprecated migration methods

## Fixed

- Missing `notification_id` on migration models

[0.1.1]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.1
[0.1.0]: https://github.com/Catalyze-Software/proxy/releases/tag/0.1.0
