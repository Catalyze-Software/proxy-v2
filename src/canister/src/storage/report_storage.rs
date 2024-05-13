use super::storage_api::{StaticStorageRef, Storage, StorageMethods, REPORTS, REPORTS_MEMORY_ID};
use canister_types::models::{api_error::ApiError, report::Report};
use ic_stable_structures::memory_manager::MemoryId;

pub struct ReportStore;

impl Storage<u64, Report> for ReportStore {
    const NAME: &'static str = "reports";

    fn storage() -> StaticStorageRef<u64, Report> {
        &REPORTS
    }

    fn memory_id() -> MemoryId {
        REPORTS_MEMORY_ID
    }
}

impl StorageMethods<u64, Report> for ReportStore {
    /// Insert a single report
    /// # Arguments
    /// * `value` - The report to insert
    /// # Returns
    /// * `Result<Report, ApiError>` - The inserted report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if so returns an error
    fn insert(value: Report) -> Result<(u64, Report), ApiError> {
        Self::storage().with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or_else(|| 1);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}
