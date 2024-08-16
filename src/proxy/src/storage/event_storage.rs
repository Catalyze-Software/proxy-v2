use super::storage_api::EVENT_CANISTER;
use candid::Principal;
use catalyze_shared::{
    attendee::AttendeeEntry,
    event_collection::EventCollectionEntry,
    event_with_attendees::{EventFilter, EventSort, EventWithAttendees},
    ic_call::ic_call,
    CanisterResult, StorageClient, StorageClientInsertable,
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

impl EventStorageClient {
    pub async fn get_attendee(&self, principal: Principal) -> CanisterResult<AttendeeEntry> {
        ic_call(self.canister_id()?, "get_attendee", (principal,)).await
    }
}

pub fn events() -> EventStorageClient {
    EventStorageClient
}
