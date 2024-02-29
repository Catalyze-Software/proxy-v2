mod attendee_storage;
mod boosted_storage;
mod event_storage;
mod friend_request_storage;
mod group_storage;
mod member_storage;
mod profile_storage;
mod report_storage;
mod storage_api;

// Re-export stores
pub use attendee_storage::AttendeeStore;
pub use boosted_storage::BoostedStore;
pub use event_storage::EventStore;
pub use friend_request_storage::FriendRequestStore;
pub use group_storage::GroupStore;
pub use member_storage::MemberStore;
pub use profile_storage::ProfileStore;
pub use report_storage::ReportStore;

pub use storage_api::IdentifierRefMethods;
pub use storage_api::StorageMethods;
