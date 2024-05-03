use super::storage_api::{StorageMethods, BOOSTED, BOOSTED_MEMORY_ID, MEMORY_MANAGER};
use canister_types::models::{api_error::ApiError, boosted::Boost};
use ic_stable_structures::StableBTreeMap;

pub struct BoostedStore;

pub const NAME: &str = "boosted";

impl StorageMethods<u64, Boost> for BoostedStore {
    /// Get a single boosted by key
    /// # Arguments
    /// * `key` - The key of the boosted to get
    /// # Returns
    /// * `Result<Boosted, ApiError>` - The boosted if found, otherwise an error
    fn get(key: u64) -> Result<(u64, Boost), ApiError> {
        BOOSTED.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple boosted by key
    /// # Arguments
    /// * `ids` - The keys of the boosted to get
    /// # Returns
    /// * `Vec<Boosted>` - The boosted if found, otherwise an empty vector
    fn get_many(keys: Vec<u64>) -> Vec<(u64, Boost)> {
        BOOSTED.with(|data| {
            let mut boosted = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    boosted.push((key, value));
                }
            }
            boosted
        })
    }

    /// Find a single boosted by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Boosted)>` - The boosted if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, Boost)>
    where
        F: Fn(&u64, &Boost) -> bool,
    {
        BOOSTED.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all boosted by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Boosted)>` - The boosted if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, Boost)>
    where
        F: Fn(&u64, &Boost) -> bool,
    {
        BOOSTED.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single boosted
    /// # Arguments
    /// * `value` - The boosted to insert
    /// # Returns
    /// * `Result<Boosted, ApiError>` - The inserted boosted if successful, otherwise an error
    /// # Note
    /// Does check if a boosted with the same key already exists, if so returns an error
    fn insert(value: Boost) -> Result<(u64, Boost), ApiError> {
        BOOSTED.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(1);

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
    fn insert_by_key(_key: u64, _value: Boost) -> Result<(u64, Boost), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single boosted by key
    /// # Arguments
    /// * `key` - The key of the boosted to update
    /// * `value` - The boosted to update
    /// # Returns
    /// * `Result<Boosted, ApiError>` - The updated boosted if successful, otherwise an error
    /// # Note
    /// Does check if a boosted with the same key already exists, if not returns an error
    fn update(key: u64, value: Boost) -> Result<(u64, Boost), ApiError> {
        BOOSTED.with(|data| {
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

    /// Remove a single boosted by key
    /// # Arguments
    /// * `key` - The key of the boosted to remove
    /// # Returns
    /// * `bool` - True if the boosted was removed, otherwise false
    fn remove(key: u64) -> bool {
        BOOSTED.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        BOOSTED.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(BOOSTED_MEMORY_ID)),
            ))
        });
    }
}

impl BoostedStore {
    /// Get all boosted by key
    /// # Returns
    /// * `Vec<Boosted>` - The boosted if found, otherwise an empty vector
    pub fn get_all() -> Vec<(u64, Boost)> {
        BOOSTED.with(|data| {
            data.borrow()
                .iter()
                .map(|(key, value)| (key, value))
                .collect()
        })
    }
}
