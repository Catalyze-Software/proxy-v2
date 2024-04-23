use super::storage_api::{StorageMethods, CLEAR_MEMORY_ID, EVENT_ATTENDEES, MEMORY_MANAGER};
use canister_types::models::{api_error::ApiError, member_collection::MemberCollection};
use ic_stable_structures::StableBTreeMap;

pub struct EventAttendeeStore;

pub const NAME: &str = "event_attendees";

impl StorageMethods<u64, MemberCollection> for EventAttendeeStore {
    /// Get event attendees by key
    /// # Arguments
    /// * `key` - The key of the event for the attendees to get
    /// # Returns
    /// * `Result<(u64, MemberCollection), ApiError>` - The event if found, otherwise an error
    fn get(key: u64) -> Result<(u64, MemberCollection), ApiError> {
        EVENT_ATTENDEES.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple event attendees by key
    /// # Arguments
    /// * `ids` - The keys of the events to get the attendees for
    /// # Returns
    /// * `Vec<(u64, MemberCollection)>` - The events if found, otherwise an empty vector
    fn get_many(keys: Vec<u64>) -> Vec<(u64, MemberCollection)> {
        EVENT_ATTENDEES.with(|data| {
            let mut events = Vec::new();
            for key in keys {
                if let Some(event) = data.borrow().get(&key) {
                    events.push((key, event));
                }
            }
            events
        })
    }

    /// Find the attendees for a single event by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, MemberCollection)>` - The event if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, MemberCollection)>
    where
        F: Fn(&u64, &MemberCollection) -> bool,
    {
        EVENT_ATTENDEES.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all event attendees by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, MemberCollection)>` - The events if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, MemberCollection)>
    where
        F: Fn(&u64, &MemberCollection) -> bool,
    {
        EVENT_ATTENDEES.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single event
    /// # Arguments
    /// * `value` - The event to insert
    /// # Returns
    /// * `Result<(u64, MemberCollection), ApiError>` - The inserted event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if so returns an error
    fn insert(_value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(
        key: u64,
        value: MemberCollection,
    ) -> Result<(u64, MemberCollection), ApiError> {
        EVENT_ATTENDEES.with(|data| {
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

    /// Update a single event by key
    /// # Arguments
    /// * `key` - The key of the event to update the attendees for
    /// * `value` - The MemberCollection to update
    /// # Returns
    /// * `Result<(u64, MemberCollection), ApiError>` - The updated event if successful, otherwise an error
    /// # Note
    /// Does check if a event with the same key already exists, if not returns an error
    fn update(key: u64, value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        EVENT_ATTENDEES.with(|data| {
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

    /// Remove a single event by key
    /// # Arguments
    /// * `key` - The key of the event to remove
    /// # Returns
    /// * `bool` - True if the event was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        EVENT_ATTENDEES.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        EVENT_ATTENDEES.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(CLEAR_MEMORY_ID)),
            ))
        });
    }
}
