use std::collections::HashMap;

use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdatable,
    GROUP_MEMBERS, GROUP_MEMBERS_MEMORY_ID,
};
use canister_types::models::member_collection::MemberCollection;
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

impl StorageQueryable<u64, MemberCollection> for GroupMemberStore {}
impl StorageUpdatable<u64, MemberCollection> for GroupMemberStore {}
impl StorageInsertableByKey<u64, MemberCollection> for GroupMemberStore {}

impl GroupMemberStore {
    /// Get all group members
    /// # Returns
    /// * `HashMap<(u64, MemberCollection)>` - All group members
    pub fn get_all() -> HashMap<u64, MemberCollection> {
        Self::storage().with(|data| data.borrow().iter().map(|(k, v)| (k, v.clone())).collect())
    }
}
