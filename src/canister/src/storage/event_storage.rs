use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdatable, EVENTS,
    EVENTS_MEMORY_ID,
};
use canister_types::models::event::Event;
use ic_stable_structures::memory_manager::MemoryId;

pub struct EventStore;

impl Storage<u64, Event> for EventStore {
    const NAME: &'static str = "events";

    fn storage() -> StaticStorageRef<u64, Event> {
        &EVENTS
    }

    fn memory_id() -> MemoryId {
        EVENTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, Event> for EventStore {}
impl StorageUpdatable<u64, Event> for EventStore {}
impl StorageInsertable<Event> for EventStore {}
