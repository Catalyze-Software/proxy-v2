use super::storage_api::GROUP_CANISTER;
use candid::Principal;
use catalyze_shared::{
    group_with_members::{GroupFilter, GroupSort, GroupWithMembers},
    ic_call::ic_call,
    old_member::MemberEntry,
    CanisterResult, StorageClient, StorageClientInsertable,
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

impl GroupStorageClient {
    pub async fn get_member(&self, member: Principal) -> CanisterResult<MemberEntry> {
        ic_call(self.canister_id()?, "get_member", (member,)).await
    }

    pub async fn get_many_members(
        &self,
        members: Vec<Principal>,
    ) -> CanisterResult<Vec<MemberEntry>> {
        ic_call(self.canister_id()?, "get_many_members", (members,)).await
    }
}

pub fn groups() -> GroupStorageClient {
    GroupStorageClient
}
