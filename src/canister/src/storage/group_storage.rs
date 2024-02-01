use std::thread::LocalKey;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::{api_error::ApiError, group::Group};

pub struct GroupStore<'a> {
    store: &'a LocalKey<StorageRef<u64, Group>>,
}

impl<'a> GroupStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<u64, Group>>) -> Self {
        Self { store }
    }
}

pub const NAME: &str = "groups";

impl StorageMethods<u64, Group> for GroupStore<'static> {
    /// Get a single group by key
    /// # Arguments
    /// * `key` - The key of the group to get
    /// # Returns
    /// * `Result<Group, ApiError>` - The group if found, otherwise an error
    fn get(&self, key: u64) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple groups by key
    /// # Arguments
    /// * `ids` - The keys of the groups to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<u64>) -> Vec<(u64, Group)> {
        self.store.with(|data| {
            let mut groups = Vec::new();
            for key in keys {
                if let Some(group) = data.borrow().get(&key) {
                    groups.push((key, group));
                }
            }
            groups
        })
    }

    /// Find a single group by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Group)>` - The group if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(u64, Group)>
    where
        F: Fn(&Group) -> bool,
    {
        self.store
            .with(|data| data.borrow().iter().find(|(_, value)| filter(value)))
    }

    /// Find all groups by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Group)>` - The groups if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(u64, Group)>
    where
        F: Fn(&Group) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(_, value)| filter(value))
                .collect()
        })
    }

    /// Insert a single group
    /// # Arguments
    /// * `value` - The group to insert
    /// # Returns
    /// * `Result<Group, ApiError>` - The inserted group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert(&mut self, value: Group) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(&mut self, _key: u64, _value: Group) -> Result<(u64, Group), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single group by key
    /// # Arguments
    /// * `key` - The key of the group to update
    /// * `value` - The group to update
    /// # Returns
    /// * `Result<Group, ApiError>` - The updated group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if not returns an error
    fn update(&mut self, key: u64, value: Group) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single group by key
    /// # Arguments
    /// * `key` - The key of the group to remove
    /// # Returns
    /// * `bool` - True if the group was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(&mut self, key: u64) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
