use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, GROUP_EVENTS, GROUP_EVENTS_MEMORY_ID,
};
use canister_types::models::{api_error::ApiError, event_collection::EventCollection};
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupEventsStore;

impl Storage<u64, EventCollection> for GroupEventsStore {
    const NAME: &'static str = "group_events";

    fn storage() -> StaticStorageRef<u64, EventCollection> {
        &GROUP_EVENTS
    }

    fn memory_id() -> MemoryId {
        GROUP_EVENTS_MEMORY_ID
    }
}

impl StorageMethods<u64, EventCollection> for GroupEventsStore {
    /// Insert a single group
    /// # Arguments
    /// * `value` - The event collection to insert
    /// # Returns
    /// * `Result<(u64, EventCollection), ApiError>` - The group id and inserted event collection if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert_by_key(key: u64, value: EventCollection) -> Result<(u64, EventCollection), ApiError> {
        Self::storage().with(|data| {
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

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert(_value: EventCollection) -> Result<(u64, EventCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }
}
