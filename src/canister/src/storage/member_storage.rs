use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, MEMBERS};
use crate::models::member::Member;

pub type MemberStore = StorageRef<String, Member>;

impl StorageMethods<Principal, Member> for MemberStore {
    /// Get a single member by key
    /// # Arguments
    /// * `key` - The key of the member to get
    /// # Returns
    /// * `Result<Member, String>` - The member if found, otherwise an error
    fn get(key: Principal) -> Result<Member, String> {
        MEMBERS.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Entity not found".to_string())
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Member) -> Result<Member, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    /// Insert a single member by key
    /// # Arguments
    /// * `key` - The user principal as key of the member to insert
    /// * `value` - The member to insert
    /// # Returns
    /// * `Result<Member, String>` - The inserted member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Member) -> Result<Member, String> {
        MEMBERS.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Update a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to update
    /// * `value` - The member to update
    /// # Returns
    /// * `Result<Member, String>` - The updated member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if not returns an error
    fn update(key: Principal, value: Member) -> Result<Member, String> {
        MEMBERS.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Remove a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to remove
    /// # Returns
    /// * `bool` - True if the member was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: Principal) -> bool {
        MEMBERS.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
