use super::storage_api::{StorageMethods, StorageRef, GROUPS};
use crate::models::group::Group;

pub type GroupStore = StorageRef<u64, Group>;

impl StorageMethods<u64, Group> for GroupStore {
    fn get(key: u64) -> Result<Group, String> {
        GROUPS.with(|data| data.borrow().get(&key).ok_or("Group not found".to_string()))
    }

    fn insert(value: Group) -> Result<Group, String> {
        GROUPS.with(|data| {
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

    fn insert_by_key(_key: u64, _value: Group) -> Result<Group, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    fn update(key: u64, value: Group) -> Result<Group, String> {
        GROUPS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    fn remove(key: u64) -> bool {
        GROUPS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
