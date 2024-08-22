use super::storage_api::BOOSTED_CANISTER;
use candid::Principal;
use catalyze_shared::{
    boosted::{Boost, BoostedFilter, BoostedSort},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct BoostedStorageClient;

impl StorageClient<u64, Boost, BoostedFilter, BoostedSort> for BoostedStorageClient {
    fn name(&self) -> String {
        "boosts".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &BOOSTED_CANISTER
    }
}

impl StorageClientInsertable<Boost, BoostedFilter, BoostedSort> for BoostedStorageClient {}

pub fn boosts() -> impl StorageClientInsertable<Boost, BoostedFilter, BoostedSort> {
    BoostedStorageClient
}
