use std::thread::LocalKey;

use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::{api_error::ApiError, member::Member};

pub struct MemberStore<'a> {
    store: &'a LocalKey<StorageRef<String, Member>>,
}

impl<'a> MemberStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<String, Member>>) -> Self {
        Self { store }
    }
}

pub const NAME: &str = "members";

impl StorageMethods<Principal, Member> for MemberStore<'static> {
    /// Get a single member by key
    /// # Arguments
    /// * `key` - The key of the member to get
    /// # Returns
    /// * `Result<Member, ApiError>` - The member if found, otherwise an error
    fn get(&self, key: Principal) -> Result<(Principal, Member), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple members by key
    /// # Arguments
    /// * `ids` - The keys of the members to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<Principal>) -> Vec<(Principal, Member)> {
        self.store.with(|data| {
            let mut members = Vec::new();
            for key in keys {
                if let Some(member) = data.borrow().get(&key.to_string()) {
                    members.push((key, member));
                }
            }
            members
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
                .map(|(key, value)| (Principal::from_text(key).unwrap(), value))
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
                .map(|(key, value)| (Principal::from_text(key).unwrap(), value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(&mut self, _value: Member) -> Result<(Principal, Member), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single member by key
    /// # Arguments
    /// * `key` - The user principal as key of the member to insert
    /// * `value` - The member to insert
    /// # Returns
    /// * `Result<Member, ApiError>` - The inserted member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if so returns an error
    fn insert_by_key(
        &mut self,
        key: Principal,
        value: Member,
    ) -> Result<(Principal, Member), ApiError> {
        self.store.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok((key, value))
        })
    }

    /// Update a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to update
    /// * `value` - The member to update
    /// # Returns
    /// * `Result<Member, ApiError>` - The updated member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if not returns an error
    fn update(&mut self, key: Principal, value: Member) -> Result<(Principal, Member), ApiError> {
        self.store.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok((key, value))
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
