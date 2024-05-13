use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, USER_NOTIFICATIONS, USER_NOTIFICATIONS_MEMORY_ID,
};
use candid::Principal;
use canister_types::models::{api_error::ApiError, user_notifications::UserNotifications};
use ic_stable_structures::memory_manager::MemoryId;

pub struct UserNotificationStore;

impl Storage<Principal, UserNotifications> for UserNotificationStore {
    const NAME: &'static str = "user_notifications";

    fn storage() -> StaticStorageRef<Principal, UserNotifications> {
        &USER_NOTIFICATIONS
    }

    fn memory_id() -> MemoryId {
        USER_NOTIFICATIONS_MEMORY_ID
    }
}

impl StorageMethods<Principal, UserNotifications> for UserNotificationStore {
    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: UserNotifications) -> Result<(Principal, UserNotifications), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
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
        Self::storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}
