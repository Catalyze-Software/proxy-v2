use super::storage_api::{StorageMethods, StorageRef, REPORTS};
use crate::models::report::Report;

pub type ReportStore = StorageRef<u64, Report>;

impl StorageMethods<u64, Report> for ReportStore {
    /// Get a single report by key
    /// # Arguments
    /// * `key` - The key of the report to get
    /// # Returns
    /// * `Result<Report, String>` - The report if found, otherwise an error
    fn get(key: u64) -> Result<Report, String> {
        REPORTS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or("Report not found".to_string())
        })
    }

    /// Insert a single report
    /// # Arguments
    /// * `value` - The report to insert
    /// # Returns
    /// * `Result<Report, String>` - The inserted report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if so returns an error
    fn insert(value: Report) -> Result<Report, String> {
        REPORTS.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(_key: u64, _value: Report) -> Result<Report, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    /// Update a single report by key
    /// # Arguments
    /// * `key` - The key of the report to update
    /// * `value` - The report to update
    /// # Returns
    /// * `Result<Report, String>` - The updated report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if not returns an error
    fn update(key: u64, value: Report) -> Result<Report, String> {
        REPORTS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    /// Remove a single report by key
    /// # Arguments
    /// * `key` - The key of the report to remove
    /// # Returns
    /// * `bool` - True if the report was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(key: u64) -> bool {
        REPORTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
