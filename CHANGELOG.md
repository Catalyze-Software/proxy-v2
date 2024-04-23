# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

### Changed

### Fixed

### Removed

## [0.1.4]

### Added

- Added `get_group_by_name` query call
- Added `get_event_attendees_profiles_and_roles` query call
- Added `get_event_invites_with_profiles` query call
- Added `get_group_members_with_profiles` query call
- Added `get_group_member_with_profile` query call
- Added `get_group_invites_with_profiles` query call
- Added `get_incoming_friend_requests_with_profile` query call
- Added `get_outgoing_friend_requests_with_profile` query call
- Added `get_relations_with_profiles` query call
- Added `SubjectResponse` to pass back reported objects
- Added `clear` methods to the stores
- added `_dev_clear_notifications` to clear the notifications from the frontend

### Changed

- Changed `get_boosted_groups` response from `Vec<(u64, Boost)>` to `Vec<GroupResponse>`
- Changed `get_boosted_events` response from `Vec<(u64, Boost)>` to `Vec<EventResponse>`
- changed `get_boosts_by_subject` parameter from `subject: Subject` to `subject: SubjectType`
- Change `get_notifications` response to `Vec<NotificationResponse>`
- Change `get_unread_notifications` response to `Vec<NotificationResponse>`
- Changed `Subject` on `ReportResponse` to `SubjectResponse` which passes back the reported object
- changed `get_pinned_by_subject_type` response to `Vec<SubjectResponse>`
- Changed `GroupNotificationType` `JoinGroupUserRequestAccept` and `JoinGroupUserRequestDecline` to return `InviteMemberResponse`
- Changed `GroupNotificationType` `JoinGroupOwnerRequestAccept` and `JoinGroupUserRequestDecline` to return `InviteMemberResponse`
- Changed `EventNotificationType` `JoinEventUserRequestAccept` and `JoinEventUserRequestDecline` to return `InviteAttendeeResponse`
- Changed `EventNotificationType` `JoinEventOwnerRequestAccept` and `JoinEventUserRequestDecline` to return `InviteAttendeeResponse`

### Fixed

- Fixed `get_unread_notifications` which returned all notifications

### Removed

## [0.1.3]

### Added

- Added `GroupMemberStore` to improve lookup performance
- Added `GroupMemberStore` initialization when creating a new group
- Added `GroupEventStore` initialization when creating a new group
- Added `UserNotifications` initialization when creating a new profile
-
- Added `EventAttendeeStore` to improve loopup performance
- Added `EventAttendeeStore` initialization when creating a new event
-
- Added `MemberCollection` struct for usage in `GroupMemberStore` and `EventAttendeeStore`

- Added `GroupEventStore` to improve lookup performance
- Added `EventCollection` struct for usage in `GroupEventStore`
- Added migration logic to fill the stores
-

### Changed

- Notification now return a `NotificationResponse` which hold additional metadata instead of a `Notification`
- Change `id: u64` to `id: Option<u64>` in `NotificationResponse` where a non-set `id` means a silent notification
- change `FriendRequest` enum value from `FriendRequest` to `FriendRequestResponse`
- change `JoinEventUserRequest` enum value from `u64` to `InviteAttendeeResponse`
- change `JoinEventOwnerRequest` enum value from `u64` to `InviteAttendeeResponse`
- change `JoinGroupUserRequest` enum value from `u64` to `InviteMemberResponse`
- change `JoinGroupOwnerRequest` enum value from `u64` to `InviteMemberResponse`
- change `FriendRequestAccept` enum value from `u64` to `FriendRequestResponse`
- change `FriendRequestDecline` enum value from `u64` to `FriendRequestResponse`
- change `remove_notifications` to `remove_user_notifications` from `NotificationCalls`

### Fixed

- Fixed `event_count` and `member_count` by usage of the new stores

### Removed

- Removed `SilentNotification` and `SendSilentNotification` from `NotificationType` enum
- Removed `get_user_notifications` query call

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
- Changed `member_count` and `event_count` to always return `0` for the `get_groups` call because of the message execution limit

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

[0.1.3]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.3
[0.1.2]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.2
[0.1.1]: https://github.com/Catalyze-Software/proxy/compare/master...0.1.1
[0.1.0]: https://github.com/Catalyze-Software/proxy/releases/tag/0.1.0
