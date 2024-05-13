use std::collections::HashMap;

use super::storage_api::{
    StaticStorageRef, Storage, StorageMethods, GROUP_MEMBERS, GROUP_MEMBERS_MEMORY_ID,
};
use canister_types::models::{api_error::ApiError, member_collection::MemberCollection};
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupMemberStore;

impl Storage<u64, MemberCollection> for GroupMemberStore {
    const NAME: &'static str = "group_members";

    fn storage() -> StaticStorageRef<u64, MemberCollection> {
        &GROUP_MEMBERS
    }

    fn memory_id() -> MemoryId {
        GROUP_MEMBERS_MEMORY_ID
    }
}

impl StorageMethods<u64, MemberCollection> for GroupMemberStore {
    /// Insert a single group
    /// # Arguments
    /// * `value` - The group to insert
    /// # Returns
    /// * `Result<(u64, GroupMembers), ApiError>` - The inserted group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert_by_key(
        key: u64,
        value: MemberCollection,
    ) -> Result<(u64, MemberCollection), ApiError> {
        Self::storage().with(|data| {
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

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert(_value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }
}

impl GroupMemberStore {
    /// Get all group members
    /// # Returns
    /// * `HashMap<(u64, MemberCollection)>` - All group members
    pub fn get_all() -> HashMap<u64, MemberCollection> {
        Self::storage().with(|data| data.borrow().iter().map(|(k, v)| (k, v.clone())).collect())
    }
}
