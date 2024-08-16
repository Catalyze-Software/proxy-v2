use super::storage_api::PROFILE_CANISTER;
use candid::Principal;
use catalyze_shared::{
    profile::{Profile, ProfileFilter, ProfileSort},
    StorageClient, StorageClientInsertableByKey,
};

#[derive(Default)]
pub struct ProfileStorageClient;

impl StorageClient<Principal, Profile, ProfileFilter, ProfileSort> for ProfileStorageClient {
    fn name(&self) -> String {
        "profile".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &PROFILE_CANISTER
    }
}

impl StorageClientInsertableByKey<Principal, Profile, ProfileFilter, ProfileSort>
    for ProfileStorageClient
{
}

pub fn profiles(
) -> impl StorageClientInsertableByKey<Principal, Profile, ProfileFilter, ProfileSort> {
    ProfileStorageClient
}
