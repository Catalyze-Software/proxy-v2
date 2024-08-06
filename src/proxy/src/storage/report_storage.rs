use super::{
    storage_api::{REPORT_CANISTER, REPORT_CANISTER_MEMORY_ID},
    CellStorage, CellStorageRef,
};
use candid::Principal;
use catalyze_shared::{
    report::{Report, ReportFilter, ReportSort},
    CanisterResult, StorageClient, StorageClientInsertable,
};
use ic_stable_structures::memory_manager::MemoryId;

pub struct ReportCanisterStorage;

impl CellStorage<Principal> for ReportCanisterStorage {
    const NAME: &'static str = "report_canister";

    fn storage() -> CellStorageRef<Principal> {
        &REPORT_CANISTER
    }

    fn memory_id() -> MemoryId {
        REPORT_CANISTER_MEMORY_ID
    }
}

#[derive(Default)]
pub struct ReportStorageClient;

impl StorageClient<u64, Report, ReportFilter, ReportSort> for ReportStorageClient {
    fn canister(&self) -> CanisterResult<Principal> {
        ReportCanisterStorage::get()
    }
}

impl StorageClientInsertable<Report, ReportFilter, ReportSort> for ReportStorageClient {}

pub fn reports() -> impl StorageClientInsertable<Report, ReportFilter, ReportSort> {
    ReportStorageClient
}
