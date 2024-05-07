use super::storage_api::{StorageMethods, ATTENDEES, ATTENDEES_MEMORY_ID, MEMORY_MANAGER};
use candid::Principal;
use canister_types::models::{api_error::ApiError, attendee::Attendee};

use ic_stable_structures::StableBTreeMap;

pub struct AttendeeStore;

pub const NAME: &str = "attendees";

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    /// Get a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to get
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The attendee if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, Attendee), ApiError> {
        ATTENDEES.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple attendees by key
    /// # Arguments
    /// * `ids` - The keys of the attendees to get
    /// # Returns
    /// * `Vec<Attendee>` - The reports if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, Attendee)> {
        ATTENDEES.with(|data| {
            let mut attendees = Vec::new();
            for key in keys {
                if let Some(attendee) = data.borrow().get(&key) {
                    attendees.push((key, attendee));
                }
            }
            attendees
        })
    }

    /// Find a single attendee by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Attendee)>` - The attendee if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        ATTENDEES.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all attendees by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Attendee)>` - The attendees if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        ATTENDEES.with(|data| {
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
    fn insert(_value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
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
        ATTENDEES.with(|data| {
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

    /// Update a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to update
    /// * `value` - The attendee to update
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The updated attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if not returns an error
    fn update(key: Principal, value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        ATTENDEES.with(|data| {
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

    /// Remove a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to remove
    /// # Returns
    /// * `bool` - True if the attendee was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        ATTENDEES.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        ATTENDEES.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(ATTENDEES_MEMORY_ID)),
            ))
        });
    }
}
