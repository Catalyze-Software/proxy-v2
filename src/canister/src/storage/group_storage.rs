use super::storage_api::{StaticStorageRef, Storage, StorageMethods, GROUPS, GROUPS_MEMORY_ID};
use canister_types::models::{api_error::ApiError, group::Group};
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupStore;

impl Storage<u64, Group> for GroupStore {
    const NAME: &'static str = "groups";

    fn storage() -> StaticStorageRef<u64, Group> {
        &GROUPS
    }

    fn memory_id() -> MemoryId {
        GROUPS_MEMORY_ID
    }
}

impl StorageMethods<u64, Group> for GroupStore {
    /// Insert a single group
    /// # Arguments
    /// * `value` - The group to insert
    /// # Returns
    /// * `Result<Group, ApiError>` - The inserted group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert(value: Group) -> Result<(u64, Group), ApiError> {
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
