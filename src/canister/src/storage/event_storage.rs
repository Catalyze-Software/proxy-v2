use super::storage_api::{StaticStorageRef, Storage, StorageMethods, EVENTS, EVENTS_MEMORY_ID};
use canister_types::models::{api_error::ApiError, event::Event};
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

impl StorageMethods<u64, Event> for EventStore {
    /// Insert a single event
    /// # Arguments
    /// * `value` - The event to insert
    /// # Returns
    /// * `Result<Event, ApiError>` - The inserted event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if so returns an error
    fn insert(value: Event) -> Result<(u64, Event), ApiError> {
        Self::storage().with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or_else(|| 1);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}
