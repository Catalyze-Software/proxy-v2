use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdatable,
    FRIEND_REQUEST, FRIEND_REQUESTS_MEMORY_ID,
};
use canister_types::models::friend_request::FriendRequest;
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

impl StorageQueryable<u64, FriendRequest> for FriendRequestStore {}
impl StorageUpdatable<u64, FriendRequest> for FriendRequestStore {}
impl StorageInsertable<FriendRequest> for FriendRequestStore {}
