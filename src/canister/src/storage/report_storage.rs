use std::thread::LocalKey;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::report::Report;

pub struct ReportStore<'a> {
    store: &'a LocalKey<StorageRef<u64, Report>>,
}

impl<'a> ReportStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<u64, Report>>) -> Self {
        Self { store }
    }
}

impl StorageMethods<u64, Report> for ReportStore<'static> {
    /// Get a single report by key
    /// # Arguments
    /// * `key` - The key of the report to get
    /// # Returns
    /// * `Result<Report, String>` - The report if found, otherwise an error
    fn get(&self, key: u64) -> Result<Report, String> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or("Report not found".to_string())
        })
    }

    /// Find a single report by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Report)>` - The report if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(u64, Report)>
    where
        F: Fn(&Report) -> bool,
    {
        self.store
            .with(|data| data.borrow().iter().find(|(_, value)| filter(value)))
    }

    /// Find all reports by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Report)>` - The reports if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(u64, Report)>
    where
        F: Fn(&Report) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(_, value)| filter(value))
                .collect()
        })
    }

    /// Insert a single report
    /// # Arguments
    /// * `value` - The report to insert
    /// # Returns
    /// * `Result<Report, String>` - The inserted report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if so returns an error
    fn insert(&mut self, value: Report) -> Result<Report, String> {
        self.store.with(|data| {
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
    fn insert_by_key(&mut self, _key: u64, _value: Report) -> Result<Report, String> {
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
    fn update(&mut self, key: u64, value: Report) -> Result<Report, String> {
        self.store.with(|data| {
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
    fn remove(&mut self, key: u64) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
