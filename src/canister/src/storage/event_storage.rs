use super::storage_api::{StorageMethods, StorageRef, EVENTS};
use crate::models::event::Event;

pub type EventStore = StorageRef<u64, Event>;

impl StorageMethods<u64, Event> for EventStore {
    fn get(key: u64) -> Result<Event, String> {
        EVENTS.with(|data| data.borrow().get(&key).ok_or("Event not found".to_string()))
    }

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

    fn insert_by_key(_key: u64, _value: Event) -> Result<Event, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    fn update(key: u64, value: Event) -> Result<Event, String> {
        EVENTS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    fn remove(key: u64) -> bool {
        EVENTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
