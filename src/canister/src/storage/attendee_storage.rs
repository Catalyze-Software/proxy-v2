use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, ATTENDEES};
use crate::models::attendee::Attendee;

pub type AttendeeStore = StorageRef<String, Attendee>;

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    /// Get a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to get
    /// # Returns
    /// * `Result<Attendee, String>` - The attendee if found, otherwise an error
    fn get(key: Principal) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Attendee not found".to_string())
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Attendee) -> Result<Attendee, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    /// Insert a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to insert
    /// * `value` - The attendee to insert
    /// # Returns
    /// * `Result<Attendee, String>` - The inserted attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Attendee) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Update a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to update
    /// * `value` - The attendee to update
    /// # Returns
    /// * `Result<Attendee, String>` - The updated attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if not returns an error
    fn update(key: Principal, value: Attendee) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Remove a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to remove
    /// # Returns
    /// * `bool` - True if the attendee was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: Principal) -> bool {
        ATTENDEES.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
