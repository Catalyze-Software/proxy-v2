use super::storage_api::{StorageMethods, FRIEND_REQUEST};
use canister_types::models::{api_error::ApiError, friend_request::FriendRequest};

pub struct FriendRequestStore;

pub const NAME: &str = "friend_requests";

impl StorageMethods<u64, FriendRequest> for FriendRequestStore {
    /// Get a single friend request by key
    /// # Arguments
    /// * `key` - The key of the friend_request to get
    /// # Returns
    /// * `Result<FriendRequest, ApiError>` - The friend_request if found, otherwise an error
    fn get(key: u64) -> Result<(u64, FriendRequest), ApiError> {
        FRIEND_REQUEST.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple friend_requests by key
    /// # Arguments
    /// * `ids` - The keys of the friend_requests to get
    /// # Returns
    /// * `Vec<FriendRequest>` - The friend_requests if found, otherwise an empty vector
    fn get_many(keys: Vec<u64>) -> Vec<(u64, FriendRequest)> {
        FRIEND_REQUEST.with(|data| {
            let mut friend_requests = Vec::new();
            for key in keys {
                if let Some(friend_request) = data.borrow().get(&key) {
                    friend_requests.push((key, friend_request));
                }
            }
            friend_requests
        })
    }

    /// Find a single friend_request by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, FriendRequest)>` - The friend_request if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, FriendRequest)>
    where
        F: Fn(&u64, &FriendRequest) -> bool,
    {
        FRIEND_REQUEST.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all friend_requests by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, FriendRequest)>` - The friend_requests if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, FriendRequest)>
    where
        F: Fn(&u64, &FriendRequest) -> bool,
    {
        FRIEND_REQUEST.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single friend_request
    /// # Arguments
    /// * `value` - The friend_request to insert
    /// # Returns
    /// * `Result<FriendRequest, ApiError>` - The inserted friend_request if successful, otherwise an error
    /// # Note
    /// Does check if a friend_request with the same key already exists, if so returns an error
    fn insert(value: FriendRequest) -> Result<(u64, FriendRequest), ApiError> {
        FRIEND_REQUEST.with(|data| {
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
    fn insert_by_key(_key: u64, _value: FriendRequest) -> Result<(u64, FriendRequest), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single friend_request by key
    /// # Arguments
    /// * `key` - The key of the friend_request to update
    /// * `value` - The friend_request to update
    /// # Returns
    /// * `Result<FriendRequest, ApiError>` - The updated friend_request if successful, otherwise an error
    /// # Note
    /// Does check if a friend_request with the same key already exists, if not returns an error
    fn update(key: u64, value: FriendRequest) -> Result<(u64, FriendRequest), ApiError> {
        FRIEND_REQUEST.with(|data| {
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

    /// Remove a single friend_request by key
    /// # Arguments
    /// * `key` - The key of the friend_request to remove
    /// # Returns
    /// * `bool` - True if the friend_request was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        FRIEND_REQUEST.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
