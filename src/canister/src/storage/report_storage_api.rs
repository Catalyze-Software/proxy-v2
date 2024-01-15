use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::report::Report;

pub type ReportStore = StorageRef<u64, Report>;

impl StorageMethods<u64, Report> for ReportStore {
    fn get(&self, id: u64) -> Result<Report, String> {
        Ok(self.borrow().get(&id).ok_or("Report not found")?.clone())
    }

    fn insert(&self, entity: Report) -> Result<Report, String> {
        let id = self
            .borrow()
            .last_key_value()
            .map(|(k, _)| k + 1)
            .unwrap_or(0);
        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn insert_by_key(&self, id: u64, entity: Report) -> Result<Report, String> {
        panic!("This entity does not require a key to be inserted, use `insert` instead")
    }

    fn update(&mut self, id: u64, entity: Report) -> Result<Report, String> {
        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: u64) -> bool {
        self.borrow_mut().remove(&id).is_some()
    }
}
