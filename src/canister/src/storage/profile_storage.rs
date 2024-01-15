use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, PROFILES};
use crate::models::profile::Profile;

pub type ProfileStore = StorageRef<String, Profile>;

impl StorageMethods<Principal, Profile> for ProfileStore {
    /// Get a single user profile by key
    /// # Arguments
    /// * `key` - The key of the profile to get
    /// # Returns
    /// * `Result<Profile, String>` - The profile if found, otherwise an error
    fn get(key: Principal) -> Result<Profile, String> {
        PROFILES.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Profile not found".to_string())
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Profile) -> Result<Profile, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    /// Insert a single user profile by key
    /// # Arguments
    /// * `key` - The user principal as key of the profile to insert
    /// * `value` - The profile to insert
    /// # Returns
    /// * `Result<Profile, String>` - The inserted profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Profile) -> Result<Profile, String> {
        PROFILES.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Update a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to update
    /// * `value` - The profile to update
    /// # Returns
    /// * `Result<Profile, String>` - The updated profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if not returns an error
    fn update(key: Principal, value: Profile) -> Result<Profile, String> {
        PROFILES.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Remove a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to remove
    /// # Returns
    /// * `bool` - True if the profile was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: Principal) -> bool {
        PROFILES.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
