use super::storage_api::REPORT_CANISTER;
use candid::Principal;
use catalyze_shared::{
    report::{Report, ReportFilter, ReportSort},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct ReportStorageClient;

impl StorageClient<u64, Report, ReportFilter, ReportSort> for ReportStorageClient {
    fn name(&self) -> String {
        "report".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &REPORT_CANISTER
    }
}

impl StorageClientInsertable<Report, ReportFilter, ReportSort> for ReportStorageClient {}

pub fn reports() -> impl StorageClientInsertable<Report, ReportFilter, ReportSort> {
    ReportStorageClient
}
