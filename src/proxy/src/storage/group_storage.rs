use super::storage_api::GROUP_CANISTER;
use catalyze_shared::{
    group_with_members::{GroupFilter, GroupSort, GroupWithMembers},
    storage_clients, StorageClientInsertable,
};

pub fn groups() -> impl StorageClientInsertable<GroupWithMembers, GroupFilter, GroupSort> {
    storage_clients::groups(&GROUP_CANISTER)
}
