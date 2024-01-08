use std::cell::RefCell;

use ic_stable_structures::StableBTreeMap;

use crate::{
    entities::report::Report,
    stores::main_store::{MEMORY_MANAGER, REPORTS_MEMORY_ID},
};

use super::main_store::Memory;

thread_local! {
    /// The `reports` store.
    /// # Note
    /// This store is used to keep track of the reports that have been created.
    pub static REPORTS: RefCell<StableBTreeMap<u64, Report, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(REPORTS_MEMORY_ID)),
        )
    );
}

/// The `Reports` struct.
/// # Note
/// This struct is used to interact with the `reports` store.
pub struct Reports;

impl Reports {
    /// Returns the `report` associated with the given key.
    /// # Arguments
    /// * `key` - The key to get the `report` at in `u64` format.
    /// # Returns
    /// * `Some(Report)` - The `report` associated with the given key.
    /// * `None` - If no `report` is associated with the given key.
    /// # Panics
    /// Panics if the `report` associated with the given key is not a `report`.
    pub fn get(key: &u64) -> Option<Report> {
        REPORTS.with(|p| p.borrow().get(key))
    }

    /// Inserts or updates a `report` into the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to insert the `report` at in `u64` format.
    /// * `value` - The `report` to insert.
    /// # Panics
    /// Panics if the `report` associated with the given key is not a `report`.
    /// # Note
    /// This function will overwrite the `report` at the given key's position if one already exists.
    pub fn insert(key: u64, value: Report) {
        REPORTS.with(|p| p.borrow_mut().insert(key, value));
    }

    /// Removes a `report` from the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to insert the `report` at in `u64` format.
    /// # Panics
    /// Panics if the `report` associated with the given key is not a `report`.
    /// # Note
    /// This function will do nothing if no `report` is associated with the given key.
    pub fn remove(key: &u64) {
        REPORTS.with(|p| p.borrow_mut().remove(key));
    }
}
