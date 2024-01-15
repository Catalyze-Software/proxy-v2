use super::storage_api::{StorageMethods, StorageRef, REPORTS};
use crate::models::report::Report;

pub type ReportStore = StorageRef<u64, Report>;

impl StorageMethods<u64, Report> for ReportStore {
    fn get(key: u64) -> Result<Report, String> {
        REPORTS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or("Report not found".to_string())
        })
    }

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

    fn insert_by_key(_key: u64, _value: Report) -> Result<Report, String> {
        Err("This value does not require a key to be inserted, use `insert` instead".to_string())
    }

    fn update(key: u64, value: Report) -> Result<Report, String> {
        REPORTS.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key, value.clone());
            Ok(value)
        })
    }

    fn remove(key: u64) -> bool {
        REPORTS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
