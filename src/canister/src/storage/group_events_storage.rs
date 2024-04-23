use super::storage_api::{StorageMethods, GROUP_EVENTS, GROUP_EVENTS_MEMORY_ID, MEMORY_MANAGER};
use canister_types::models::{api_error::ApiError, event_collection::EventCollection};
use ic_stable_structures::StableBTreeMap;

pub struct GroupEventsStore;

pub const NAME: &str = "group_events";

impl StorageMethods<u64, EventCollection> for GroupEventsStore {
    /// Get event collection by key
    /// # Arguments
    /// * `key` - The key of the group for the members to get
    /// # Returns
    /// * `Result<(u64, EventCollection), ApiError>` - The group if found, otherwise an error
    fn get(key: u64) -> Result<(u64, EventCollection), ApiError> {
        GROUP_EVENTS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple group event collections by key
    /// # Arguments
    /// * `ids` - The keys of the groups to get the members for
    /// # Returns
    /// * `Vec<(u64, EventCollection)>` - The groups if found, otherwise an empty vector
    fn get_many(keys: Vec<u64>) -> Vec<(u64, EventCollection)> {
        GROUP_EVENTS.with(|data| {
            let mut groups = Vec::new();
            for key in keys {
                if let Some(group) = data.borrow().get(&key) {
                    groups.push((key, group));
                }
            }
            groups
        })
    }

    /// Find the event collection for a single group by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, EventCollection)>` - The event collection if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, EventCollection)>
    where
        F: Fn(&u64, &EventCollection) -> bool,
    {
        GROUP_EVENTS.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all group members by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, EventCollection)>` - The event collection if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, EventCollection)>
    where
        F: Fn(&u64, &EventCollection) -> bool,
    {
        GROUP_EVENTS.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single group
    /// # Arguments
    /// * `value` - The event collection to insert
    /// # Returns
    /// * `Result<(u64, EventCollection), ApiError>` - The group id and inserted event collection if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert_by_key(key: u64, value: EventCollection) -> Result<(u64, EventCollection), ApiError> {
        GROUP_EVENTS.with(|data| {
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
    fn insert(_value: EventCollection) -> Result<(u64, EventCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }

    /// Update a single event collection by key
    /// # Arguments
    /// * `key` - The key of the group to update the members for
    /// * `value` - The EventCollection to update
    /// # Returns
    /// * `Result<(u64, EventCollection), ApiError>` - The updated group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if not returns an error
    fn update(key: u64, value: EventCollection) -> Result<(u64, EventCollection), ApiError> {
        GROUP_EVENTS.with(|data| {
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

    /// Remove a event collection by key
    /// # Arguments
    /// * `key` - The key of the group to remove
    /// # Returns
    /// * `bool` - True if the group was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        GROUP_EVENTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        GROUP_EVENTS.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(GROUP_EVENTS_MEMORY_ID)),
            ))
        });
    }
}
