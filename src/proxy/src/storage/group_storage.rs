use super::storage_api::GROUP_CANISTER;
use candid::Principal;
use catalyze_shared::{
    group_with_members::{GroupFilter, GroupSort, GroupWithMembers},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct GroupStorageClient;

impl StorageClient<u64, GroupWithMembers, GroupFilter, GroupSort> for GroupStorageClient {
    fn name(&self) -> String {
        "group".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &GROUP_CANISTER
    }
}

impl StorageClientInsertable<GroupWithMembers, GroupFilter, GroupSort> for GroupStorageClient {}

pub fn groups() -> GroupStorageClient {
    GroupStorageClient
}
