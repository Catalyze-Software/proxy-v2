use super::storage_api::{StorageMethods, StorageRef};
use crate::models::event::Event;

pub type EventStore = StorageRef<u64, Event>;

impl StorageMethods<u64, Event> for EventStore {
    fn get(&self, key: u64) -> Result<Event, String> {
        Ok(self.borrow().get(&key).ok_or("Entity not found")?.clone())
    }

    fn insert(&self, value: Event) -> Result<Event, String> {
        let key = self
            .borrow()
            .last_key_value()
            .map(|(k, _)| k + 1)
            .unwrap_or(0);

        if self.borrow().contains_key(&key) {
            return Err("Key already exists".to_string());
        }

        self.borrow_mut().insert(key, value.clone());
        Ok(value)
    }

    fn insert_by_key(&self, _key: u64, _value: Event) -> Result<Event, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    fn update(&mut self, key: u64, value: Event) -> Result<Event, String> {
        if !self.borrow().contains_key(&key) {
            return Err("Key does not exists".to_string());
        }

        self.borrow_mut().insert(key, value.clone());
        Ok(value)
    }

    fn remove(&mut self, key: u64) -> bool {
        self.borrow_mut().remove(&key).is_some()
    }
}
