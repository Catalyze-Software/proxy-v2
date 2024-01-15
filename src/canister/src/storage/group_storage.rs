use super::storage_api::{StorageMethods, StorageRef, GROUPS};
use crate::models::group::Group;

pub type GroupStore = StorageRef<u64, Group>;

impl StorageMethods<u64, Group> for GroupStore {
    /// Get a single group by key
    /// # Arguments
    /// * `key` - The key of the group to get
    /// # Returns
    /// * `Result<Group, String>` - The group if found, otherwise an error
    fn get(key: u64) -> Result<Group, String> {
        GROUPS.with(|data| data.borrow().get(&key).ok_or("Group not found".to_string()))
    }

    /// Insert a single group
    /// # Arguments
    /// * `value` - The group to insert
    /// # Returns
    /// * `Result<Group, String>` - The inserted group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
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

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(_key: u64, _value: Group) -> Result<Group, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    /// Update a single group by key
    /// # Arguments
    /// * `key` - The key of the group to update
    /// * `value` - The group to update
    /// # Returns
    /// * `Result<Group, String>` - The updated group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if not returns an error
    fn update(key: u64, value: Group) -> Result<Group, String> {
        GROUPS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    /// Remove a single group by key
    /// # Arguments
    /// * `key` - The key of the group to remove
    /// # Returns
    /// * `bool` - True if the group was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: u64) -> bool {
        GROUPS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
