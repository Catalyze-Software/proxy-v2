use std::thread::LocalKey;

use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::{api_error::ApiError, attendee::Attendee};

pub struct AttendeeStore<'a> {
    store: &'a LocalKey<StorageRef<String, Attendee>>,
}

impl<'a> AttendeeStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<String, Attendee>>) -> Self {
        Self { store }
    }
}

pub const NAME: &str = "attendees";

impl StorageMethods<Principal, Attendee> for AttendeeStore<'static> {
    /// Get a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to get
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The attendee if found, otherwise an error
    fn get(&self, key: Principal) -> Result<Attendee, ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
        })
    }

    /// Get multiple attendees by key
    /// # Arguments
    /// * `ids` - The keys of the attendees to get
    /// # Returns
    /// * `Vec<Attendee>` - The reports if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<Principal>) -> Vec<Attendee> {
        self.store.with(|data| {
            let mut attendees = Vec::new();
            for key in keys {
                if let Some(attendee) = data.borrow().get(&key.to_string()) {
                    attendees.push(attendee.clone());
                }
            }
            attendees
        })
    }

    /// Find a single attendee by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Attendee)>` - The attendee if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(Principal, Attendee)>
    where
        F: Fn(&Attendee) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .find(|(_, value)| filter(value))
                .map(|(key, value)| (Principal::from_text(key).unwrap(), value.clone()))
        })
    }

    /// Find all attendees by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Attendee)>` - The attendees if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(Principal, Attendee)>
    where
        F: Fn(&Attendee) -> bool,
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
    fn insert(&mut self, _value: Attendee) -> Result<Attendee, ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to insert
    /// * `value` - The attendee to insert
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The inserted attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if so returns an error
    fn insert_by_key(&mut self, key: Principal, value: Attendee) -> Result<Attendee, ApiError> {
        self.store.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Update a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to update
    /// * `value` - The attendee to update
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The updated attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if not returns an error
    fn update(&mut self, key: Principal, value: Attendee) -> Result<Attendee, ApiError> {
        self.store.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    /// Remove a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to remove
    /// # Returns
    /// * `bool` - True if the attendee was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(&mut self, key: Principal) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
