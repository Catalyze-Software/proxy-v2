use super::storage_api::{StorageMethods, MEMORY_MANAGER, PROFILES, PROFILES_MEMORY_ID};
use candid::Principal;
use canister_types::models::{api_error::ApiError, profile::Profile};

use ic_stable_structures::StableBTreeMap;

pub struct ProfileStore;

pub const NAME: &str = "profiles";

impl StorageMethods<Principal, Profile> for ProfileStore {
    /// Get a single user profile by key
    /// # Arguments
    /// * `key` - The key of the profile to get
    /// # Returns
    /// * `Result<(Principal, Profile), ApiError>` - The profile if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, Profile), ApiError> {
        PROFILES.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple profiles by key
    /// # Arguments
    /// * `ids` - The keys of the profiles to get
    /// # Returns
    /// * `Vec<(Principal, Profile)>` - The reports if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, Profile)> {
        PROFILES.with(|data| {
            let mut profiles = Vec::new();
            for key in keys {
                if let Some(profile) = data.borrow().get(&key) {
                    profiles.push((key, profile));
                }
            }
            profiles
        })
    }

    /// Find a single user profile by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Profile)>` - The profile if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, Profile)>
    where
        F: Fn(&Principal, &Profile) -> bool,
    {
        PROFILES.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, profile)| filter(id, profile))
                .map(|(key, profile)| (key, profile))
        })
    }

    /// Find all user profiles by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Profile)>` - The profiles if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, Profile)>
    where
        F: Fn(&Principal, &Profile) -> bool,
    {
        PROFILES.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Profile) -> Result<(Principal, Profile), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
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
        PROFILES.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Update a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to update
    /// * `value` - The profile to update
    /// # Returns
    /// * `Result<(Principal, Profile), ApiError>` - The updated profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if not returns an error
    fn update(key: Principal, value: Profile) -> Result<(Principal, Profile), ApiError> {
        PROFILES.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to remove
    /// # Returns
    /// * `bool` - True if the profile was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        PROFILES.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        PROFILES.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(PROFILES_MEMORY_ID)),
            ))
        });
    }
}
