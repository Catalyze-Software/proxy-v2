use std::thread::LocalKey;

use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::member::Member;

pub struct MemberStore<'a> {
    store: &'a LocalKey<StorageRef<String, Member>>,
}

impl<'a> MemberStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<String, Member>>) -> Self {
        Self { store }
    }
}

impl StorageMethods<Principal, Member> for MemberStore<'static> {
    /// Get a single member by key
    /// # Arguments
    /// * `key` - The key of the member to get
    /// # Returns
    /// * `Result<Member, String>` - The member if found, otherwise an error
    fn get(&self, key: Principal) -> Result<Member, String> {
        self.store.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Member not found".to_string())
        })
    }

    /// Find a single member by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Member)>` - The member if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(Principal, Member)>
    where
        F: Fn(&Member) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .find(|(_, value)| filter(value))
                .map(|(key, value)| (Principal::from_text(key).unwrap(), value.clone()))
        })
    }

    /// Find all members by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Member)>` - The members if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(Principal, Member)>
    where
        F: Fn(&Member) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(_, value)| filter(value))
                .map(|(key, value)| (Principal::from_text(key).unwrap(), value.clone()))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(&mut self, _value: Member) -> Result<Member, String> {
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
    fn insert_by_key(&mut self, key: Principal, value: Member) -> Result<Member, String> {
        self.store.with(|data| {
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
    fn update(&mut self, key: Principal, value: Member) -> Result<Member, String> {
        self.store.with(|data| {
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
    fn remove(&mut self, key: Principal) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
