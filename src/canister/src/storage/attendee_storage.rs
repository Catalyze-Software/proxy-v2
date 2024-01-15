use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, ATTENDEES};
use crate::models::attendee::Attendee;

pub type AttendeeStore = StorageRef<String, Attendee>;

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    fn get(key: Principal) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Attendee not found".to_string())
        })
    }

    fn insert(_value: Attendee) -> Result<Attendee, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    fn insert_by_key(key: Principal, value: Attendee) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn update(key: Principal, value: Attendee) -> Result<Attendee, String> {
        ATTENDEES.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn remove(key: Principal) -> bool {
        ATTENDEES.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
