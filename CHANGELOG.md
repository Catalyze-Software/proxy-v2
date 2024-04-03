# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

### Changed

### Fixed

### Removed

## [0.1.2]

### Added

- Add query call `get_self_groups` that return all the groups that the user is part of
- Add query call `get_self_events` that return all the events that the user is part of
- Added `pinned: Vec<Subject>` to `Profile`
- Added `is_pinned` method to `Profile` implementation
- Added `is_pinned` variable to `GroupCallerData` implementation
- Added `add_pinned` query call + method
- Added `remove_pinned` query call + method
- Added `get_pinned_by_subject` query call + method
- Added `add_pinned` query call + method
- Added `remove_pinned` query call + method
- Added `hours_to_nanoseconds` method for date comparison
- Added `members_count` to `GroupResponse`
- Added `events_count` to `GroupResponse`
- Added `get_groups_count` to give back all different filtering counts

### Changed

- Changed query call `get_self_group` to `get_self_member` because this call returns member specific data
- Changed query call `get_self_event` to `get_self_attendee` because this call returns attendee specific data
- Changed `get_event_count` to give back all different filtering counts

### Removed

- Removed not used empty variable from `TransactionNotificationType::Airdrop`
- Removed `get_starred_groups` query in favor of `get_starred_by_subject`
- Removed `get_starred_events` query in favor of `get_starred_by_subject`
- Removed `group_id` param from `get_event` query method
- Removed `group_id` param from `join_event` update method

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

[0.1.2]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.2
[0.1.1]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.1
[0.1.0]: https://github.com/Catalyze-Software/proxy/releases/tag/0.1.0
