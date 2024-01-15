use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::report::Report;

pub type ReportStore = StorageRef<u64, Report>;

impl StorageMethods<u64, Report> for ReportStore {
    fn get(&self, key: u64) -> Result<Report, String> {
        Ok(self.borrow().get(&key).ok_or("Report not found")?.clone())
    }

    fn insert(&self, value: Report) -> Result<Report, String> {
        let key = self
            .borrow()
            .last_key_value()
            .map(|(k, _)| k + 1)
            .unwrap_or(0);

        if self.borrow().contains_key(&key) {
            return Err("Key already exists".to_string());
        }

        self.borrow_mut().insert(key, value.clone());
        Ok(value)
    }

    fn insert_by_key(&self, key: u64, value: Report) -> Result<Report, String> {
        panic!("This value does not require a key to be inserted, use `insert` instead")
    }

    fn update(&mut self, key: u64, value: Report) -> Result<Report, String> {
        self.borrow_mut().insert(key, value.clone());
        Ok(value)
    }

    fn remove(&mut self, key: u64) -> bool {
        self.borrow_mut().remove(&key).is_some()
    }
}
