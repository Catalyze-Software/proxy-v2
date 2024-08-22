use super::storage_api::FRIEND_REQUEST_CANISTER;
use candid::Principal;
use catalyze_shared::{
    friend_request::{FriendRequest, FriendRequestFilter, FriendRequestSort},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct FriendRequestStorageClient;

impl StorageClient<u64, FriendRequest, FriendRequestFilter, FriendRequestSort>
    for FriendRequestStorageClient
{
    fn name(&self) -> String {
        "friend_requests".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &FRIEND_REQUEST_CANISTER
    }
}

impl StorageClientInsertable<FriendRequest, FriendRequestFilter, FriendRequestSort>
    for FriendRequestStorageClient
{
}

pub fn friend_requests(
) -> impl StorageClientInsertable<FriendRequest, FriendRequestFilter, FriendRequestSort> {
    FriendRequestStorageClient
}
