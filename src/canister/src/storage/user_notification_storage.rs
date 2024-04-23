use super::storage_api::{
    StorageMethods, MEMORY_MANAGER, USER_NOTIFICATIONS, USER_NOTIFICATIONS_MEMORY_ID,
};
use candid::Principal;
use canister_types::models::{api_error::ApiError, user_notifications::UserNotifications};
use ic_stable_structures::StableBTreeMap;

pub struct UsernotificationStore;

pub const NAME: &str = "user_notifications";

impl StorageMethods<Principal, UserNotifications> for UsernotificationStore {
    /// Get a single unread_notifications by key
    /// # Arguments
    /// * `key` - The key of the unread_notifications to get
    /// # Returns
    /// * `Result<(Principal, UnreadNotifications), ApiError>` - The unread_notifications if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, UserNotifications), ApiError> {
        USER_NOTIFICATIONS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple multi_unread_notifications by key
    /// # Arguments
    /// * `ids` - The keys of the multi_unread_notifications to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, UserNotifications)> {
        USER_NOTIFICATIONS.with(|data| {
            let mut multi_unread_notifications: Vec<(Principal, UserNotifications)> = Vec::new();
            for key in keys {
                if let Some(unread_notifications) = data.borrow().get(&key) {
                    multi_unread_notifications.push((key, unread_notifications));
                }
            }
            multi_unread_notifications
        })
    }

    /// Find a single unread_notifications by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, UnreadNotifications)>` - The unread_notifications if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, UserNotifications)>
    where
        F: Fn(&Principal, &UserNotifications) -> bool,
    {
        USER_NOTIFICATIONS.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all multi_unread_notifications by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, UnreadNotifications)>` - The multi_unread_notifications if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, UserNotifications)>
    where
        F: Fn(&Principal, &UserNotifications) -> bool,
    {
        USER_NOTIFICATIONS.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: UserNotifications) -> Result<(Principal, UserNotifications), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single unread_notifications by key
    /// # Arguments
    /// * `key` - The user principal as key of the unread_notifications to insert
    /// * `value` - The unread_notifications to insert
    /// # Returns
    /// * `Result<(Principal, UnreadNotifications), ApiError>` - The inserted unread_notifications if successful, otherwise an error
    /// # Note
    /// Does check if a unread_notifications with the same key already exists, if so returns an error
    fn insert_by_key(
        key: Principal,
        value: UserNotifications,
    ) -> Result<(Principal, UserNotifications), ApiError> {
        USER_NOTIFICATIONS.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Update a single unread_notifications by key
    /// # Arguments
    /// * `key` - The user principal key of the unread_notifications to update
    /// * `value` - The unread_notifications to update
    /// # Returns
    /// * `Result<(Principal, UnreadNotifications), ApiError>` - The updated unread_notifications if successful, otherwise an error
    /// # Note
    /// Does check if a unread_notifications with the same key already exists, if not returns an error
    fn update(
        key: Principal,
        value: UserNotifications,
    ) -> Result<(Principal, UserNotifications), ApiError> {
        USER_NOTIFICATIONS.with(|data| {
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

    /// Remove a single unread_notifications by key
    /// # Arguments
    /// * `key` - The user principal key of the unread_notifications to remove
    /// # Returns
    /// * `bool` - True if the unread_notifications was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        USER_NOTIFICATIONS.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        USER_NOTIFICATIONS.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(USER_NOTIFICATIONS_MEMORY_ID)),
            ))
        });
    }
}
