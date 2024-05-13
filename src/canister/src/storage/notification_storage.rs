use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, NOTIFICATIONS, NOTIFICATIONS_MEMORY_ID,
};
use canister_types::models::{api_error::ApiError, notification::Notification};
use ic_stable_structures::memory_manager::MemoryId;

pub struct NotificationStore;

impl Storage<u64, Notification> for NotificationStore {
    const NAME: &'static str = "notifications";

    fn storage() -> StaticStorageRef<u64, Notification> {
        &NOTIFICATIONS
    }

       fn memory_id() -> MemoryId {
        NOTIFICATIONS_MEMORY_ID
    }
}

impl StorageMethods<u64, Notification> for NotificationStore {
    /// Insert a single notification
    /// # Arguments
    /// * `value` - The notification to insert
    /// # Returns
    /// * `Result<Notification, ApiError>` - The inserted notification if successful, otherwise an error
    /// # Note
    /// Does check if a notification with the same key already exists, if so returns an error
    fn insert(value: Notification) -> Result<(u64, Notification), ApiError> {
        Self::storage().with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or_else(|| 1);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}
