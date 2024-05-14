use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdatable,
    EVENT_ATTENDEES, EVENT_ATTENDEES_MEMORY_ID,
};
use canister_types::models::member_collection::MemberCollection;
use ic_stable_structures::memory_manager::MemoryId;

pub struct EventAttendeeStore;

impl Storage<u64, MemberCollection> for EventAttendeeStore {
    const NAME: &'static str = "event_attendees";

    fn storage() -> StaticStorageRef<u64, MemberCollection> {
        &EVENT_ATTENDEES
    }

    fn memory_id() -> MemoryId {
        EVENT_ATTENDEES_MEMORY_ID
    }
}

impl StorageQueryable<u64, MemberCollection> for EventAttendeeStore {}
impl StorageUpdatable<u64, MemberCollection> for EventAttendeeStore {}
impl StorageInsertableByKey<u64, MemberCollection> for EventAttendeeStore {}
