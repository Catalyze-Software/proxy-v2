use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, FRIEND_REQUEST, FRIEND_REQUESTS_MEMORY_ID,
};
use canister_types::models::{api_error::ApiError, friend_request::FriendRequest};
use ic_stable_structures::memory_manager::MemoryId;

pub struct FriendRequestStore;

impl Storage<u64, FriendRequest> for FriendRequestStore {
    const NAME: &'static str = "friend_requests";

    fn storage() -> StaticStorageRef<u64, FriendRequest> {
        &FRIEND_REQUEST
    }

    fn memory_id() -> MemoryId {
        FRIEND_REQUESTS_MEMORY_ID
    }
}

impl StorageMethods<u64, FriendRequest> for FriendRequestStore {
    /// Insert a single friend_request
    /// # Arguments
    /// * `value` - The friend_request to insert
    /// # Returns
    /// * `Result<FriendRequest, ApiError>` - The inserted friend_request if successful, otherwise an error
    /// # Note
    /// Does check if a friend_request with the same key already exists, if so returns an error
    fn insert(value: FriendRequest) -> Result<(u64, FriendRequest), ApiError> {
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
