use super::storage_api::{StorageMethods, REPORTS};
use canister_types::models::{api_error::ApiError, report::Report};

pub struct ReportStore;

pub const NAME: &str = "reports";

impl StorageMethods<u64, Report> for ReportStore {
    /// Get a single report by key
    /// # Arguments
    /// * `key` - The key of the report to get
    /// # Returns
    /// * `Result<Report, ApiError>` - The report if found, otherwise an error
    fn get(key: u64) -> Result<(u64, Report), ApiError> {
        REPORTS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple reports by key
    /// # Arguments
    /// * `ids` - The keys of the reports to get
    /// # Returns
    /// * `Vec<Report>` - The reports if found, otherwise an empty vector
    fn get_many(ids: Vec<u64>) -> Vec<(u64, Report)> {
        REPORTS.with(|data| {
            let mut reports = Vec::new();
            for id in ids {
                if let Some(report) = data.borrow().get(&id) {
                    reports.push((id, report));
                }
            }
            reports
        })
    }

    /// Find a single report by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Report)>` - The report if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, Report)>
    where
        F: Fn(&u64, &Report) -> bool,
    {
        REPORTS.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all reports by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Report)>` - The reports if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, Report)>
    where
        F: Fn(&u64, &Report) -> bool,
    {
        REPORTS.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single report
    /// # Arguments
    /// * `value` - The report to insert
    /// # Returns
    /// * `Result<Report, ApiError>` - The inserted report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if so returns an error
    fn insert(value: Report) -> Result<(u64, Report), ApiError> {
        REPORTS.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(_key: u64, _value: Report) -> Result<(u64, Report), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single report by key
    /// # Arguments
    /// * `key` - The key of the report to update
    /// * `value` - The report to update
    /// # Returns
    /// * `Result<Report, ApiError>` - The updated report if successful, otherwise an error
    /// # Note
    /// Does check if a report with the same key already exists, if not returns an error
    fn update(key: u64, value: Report) -> Result<(u64, Report), ApiError> {
        REPORTS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single report by key
    /// # Arguments
    /// * `key` - The key of the report to remove
    /// # Returns
    /// * `bool` - True if the report was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        REPORTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
