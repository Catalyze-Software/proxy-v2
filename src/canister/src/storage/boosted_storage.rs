use super::storage_api::{StaticStorageRef, Storage, StorageMethods, BOOSTED, BOOSTED_MEMORY_ID};
use canister_types::models::{api_error::ApiError, boosted::Boost};
use ic_stable_structures::memory_manager::MemoryId;

pub struct BoostedStore;

impl Storage<u64, Boost> for BoostedStore {
    const NAME: &'static str = "boosted";

    fn storage() -> StaticStorageRef<u64, Boost> {
        &BOOSTED
    }

    fn memory_id() -> MemoryId {
        BOOSTED_MEMORY_ID
    }
}

impl StorageMethods<u64, Boost> for BoostedStore {
    /// Insert a single boosted
    /// # Arguments
    /// * `value` - The boosted to insert
    /// # Returns
    /// * `Result<Boosted, ApiError>` - The inserted boosted if successful, otherwise an error
    /// # Note
    /// Does check if a boosted with the same key already exists, if so returns an error
    fn insert(value: Boost) -> Result<(u64, Boost), ApiError> {
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
