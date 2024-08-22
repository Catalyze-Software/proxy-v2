use super::storage_api::EVENT_CANISTER;
use candid::Principal;
use catalyze_shared::{
    event_with_attendees::{EventFilter, EventSort, EventWithAttendees},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct EventStorageClient;

impl StorageClient<u64, EventWithAttendees, EventFilter, EventSort> for EventStorageClient {
    fn name(&self) -> String {
        "event".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &EVENT_CANISTER
    }
}

impl StorageClientInsertable<EventWithAttendees, EventFilter, EventSort> for EventStorageClient {}

pub fn events() -> EventStorageClient {
    EventStorageClient
}
