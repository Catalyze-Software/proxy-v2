use super::storage_api::PROFILE_CANISTER;
use candid::Principal;
use catalyze_shared::{
    profile_with_refs::{ProfileFilter, ProfileSort, ProfileWithRefs},
    StorageClient, StorageClientInsertableByKey,
};

#[derive(Default)]
pub struct ProfileStorageClient;

impl StorageClient<Principal, ProfileWithRefs, ProfileFilter, ProfileSort>
    for ProfileStorageClient
{
    fn name(&self) -> String {
        "profile".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &PROFILE_CANISTER
    }
}

impl StorageClientInsertableByKey<Principal, ProfileWithRefs, ProfileFilter, ProfileSort>
    for ProfileStorageClient
{
}

pub fn profiles(
) -> impl StorageClientInsertableByKey<Principal, ProfileWithRefs, ProfileFilter, ProfileSort> {
    ProfileStorageClient
}
