use super::storage_api::{StaticStorageRef, Storage, StorageMethods, PROFILES, PROFILES_MEMORY_ID};
use candid::Principal;
use canister_types::models::{api_error::ApiError, profile::Profile};

use ic_stable_structures::memory_manager::MemoryId;

pub struct ProfileStore;

impl Storage<Principal, Profile> for ProfileStore {
    const NAME: &'static str = "profiles";

    fn storage() -> StaticStorageRef<Principal, Profile> {
        &PROFILES
    }

    fn memory_id() -> MemoryId {
        PROFILES_MEMORY_ID
    }
}

impl StorageMethods<Principal, Profile> for ProfileStore {
    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Profile) -> Result<(Principal, Profile), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single user profile by key
    /// # Arguments
    /// * `key` - The user principal as key of the profile to insert
    /// * `value` - The profile to insert
    /// # Returns
    /// * `Result<(Principal, Profile), ApiError>` - The inserted profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Profile) -> Result<(Principal, Profile), ApiError> {
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
