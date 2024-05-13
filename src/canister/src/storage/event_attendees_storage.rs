use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, EVENT_ATTENDEES, EVENT_ATTENDEES_MEMORY_ID,
};
use canister_types::models::{api_error::ApiError, member_collection::MemberCollection};
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

impl StorageMethods<u64, MemberCollection> for EventAttendeeStore {
    /// Insert a single event
    /// # Arguments
    /// * `value` - The event to insert
    /// # Returns
    /// * `Result<(u64, MemberCollection), ApiError>` - The inserted event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if so returns an error
    fn insert(_value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(
        key: u64,
        value: MemberCollection,
    ) -> Result<(u64, MemberCollection), ApiError> {
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
}
