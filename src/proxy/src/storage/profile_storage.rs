use super::{
    storage_api::{PROFILE_CANISTER, PROFILE_CANISTER_MEMORY_ID},
    CellStorage, CellStorageRef,
};
use candid::Principal;
use catalyze_shared::{
    profile::{Profile, ProfileFilter, ProfileSort},
    CanisterResult, StorageClient, StorageClientInsertableByKey,
};

use ic_stable_structures::memory_manager::MemoryId;

pub struct ProfileCanisterStorage;

impl CellStorage<Principal> for ProfileCanisterStorage {
    const NAME: &'static str = "profile_canister";

    fn storage() -> CellStorageRef<Principal> {
        &PROFILE_CANISTER
    }

    fn memory_id() -> MemoryId {
        PROFILE_CANISTER_MEMORY_ID
    }
}

#[derive(Default)]
pub struct ProfileStorageClient;

impl StorageClient<Principal, Profile, ProfileFilter, ProfileSort> for ProfileStorageClient {
    fn canister(&self) -> CanisterResult<Principal> {
        ProfileCanisterStorage::get()
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
