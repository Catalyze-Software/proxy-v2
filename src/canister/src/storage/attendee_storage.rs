use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, ATTENDEES, ATTENDEES_MEMORY_ID,
};
use candid::Principal;
use canister_types::models::{api_error::ApiError, attendee::Attendee};

use ic_stable_structures::memory_manager::MemoryId;

pub struct AttendeeStore;

impl Storage<Principal, Attendee> for AttendeeStore {
    const NAME: &'static str = "attendees";

    fn storage() -> StaticStorageRef<Principal, Attendee> {
        &ATTENDEES
    }

    fn memory_id() -> MemoryId {
        ATTENDEES_MEMORY_ID
    }
}

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to insert
    /// * `value` - The attendee to insert
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The inserted attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        Self::storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}
