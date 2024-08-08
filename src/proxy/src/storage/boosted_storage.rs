use super::{
    storage_api::{BOOSTED_CANISTER, BOOSTED_CANISTER_MEMORY_ID},
    CellStorage, CellStorageRef,
};
use candid::Principal;
use catalyze_shared::{
    boosted::{Boost, BoostedFilter, BoostedSort},
    CanisterResult, StorageClient, StorageClientInsertable,
};
use ic_stable_structures::memory_manager::MemoryId;

pub struct BoostedCanisterStorage;

impl CellStorage<Principal> for BoostedCanisterStorage {
    const NAME: &'static str = "boosted_canister";

    fn storage() -> CellStorageRef<Principal> {
        &BOOSTED_CANISTER
    }

    fn memory_id() -> MemoryId {
        BOOSTED_CANISTER_MEMORY_ID
    }
}

#[derive(Default)]
pub struct BoostedStorageClient;

impl StorageClient<u64, Boost, BoostedFilter, BoostedSort> for BoostedStorageClient {
    fn canister(&self) -> CanisterResult<Principal> {
        BoostedCanisterStorage::get()
    }
}

impl StorageClientInsertable<Boost, BoostedFilter, BoostedSort> for BoostedStorageClient {}

pub fn boosteds() -> impl StorageClientInsertable<Boost, BoostedFilter, BoostedSort> {
    BoostedStorageClient
}
