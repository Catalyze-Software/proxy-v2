use super::storage_api::{StorageMethods, StorageRef, EVENTS};
use crate::models::event::Event;

pub type EventStore = StorageRef<u64, Event>;

impl StorageMethods<u64, Event> for EventStore {
    /// Get a single event by key
    /// # Arguments
    /// * `key` - The key of the event to get
    /// # Returns
    /// * `Result<Event, String>` - The event if found, otherwise an error
    fn get(key: u64) -> Result<Event, String> {
        EVENTS.with(|data| data.borrow().get(&key).ok_or("Event not found".to_string()))
    }

    /// Insert a single event
    /// # Arguments
    /// * `value` - The event to insert
    /// # Returns
    /// * `Result<Event, String>` - The inserted event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if so returns an error
    fn insert(value: Event) -> Result<Event, String> {
        EVENTS.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(_key: u64, _value: Event) -> Result<Event, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    /// Update a single event by key
    /// # Arguments
    /// * `key` - The key of the event to update
    /// * `value` - The event to update
    /// # Returns
    /// * `Result<Event, String>` - The updated event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if not returns an error
    fn update(key: u64, value: Event) -> Result<Event, String> {
        EVENTS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    /// Remove a single event by key
    /// # Arguments
    /// * `key` - The key of the event to remove
    /// # Returns
    /// * `bool` - True if the event was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: u64) -> bool {
        EVENTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
