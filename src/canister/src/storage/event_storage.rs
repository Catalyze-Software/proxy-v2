use std::thread::LocalKey;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::event::Event;

pub struct EventStore<'a> {
    store: &'a LocalKey<StorageRef<u64, Event>>,
}

impl<'a> EventStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<u64, Event>>) -> Self {
        Self { store }
    }
}

impl StorageMethods<u64, Event> for EventStore<'static> {
    /// Get a single event by key
    /// # Arguments
    /// * `key` - The key of the event to get
    /// # Returns
    /// * `Result<Event, String>` - The event if found, otherwise an error
    fn get(&self, key: u64) -> Result<Event, String> {
        self.store
            .with(|data| data.borrow().get(&key).ok_or("Event not found".to_string()))
    }

    /// Find a single event by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Event)>` - The event if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(u64, Event)>
    where
        F: Fn(&Event) -> bool,
    {
        self.store
            .with(|data| data.borrow().iter().find(|(_, value)| filter(value)))
    }

    /// Find all events by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Event)>` - The events if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(u64, Event)>
    where
        F: Fn(&Event) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(_, value)| filter(value))
                .collect()
        })
    }

    /// Insert a single event
    /// # Arguments
    /// * `value` - The event to insert
    /// # Returns
    /// * `Result<Event, String>` - The inserted event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if so returns an error
    fn insert(&mut self, value: Event) -> Result<Event, String> {
        self.store.with(|data| {
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
    fn insert_by_key(&mut self, _key: u64, _value: Event) -> Result<Event, String> {
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
    fn update(&mut self, key: u64, value: Event) -> Result<Event, String> {
        self.store.with(|data| {
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
    fn remove(&mut self, key: u64) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
