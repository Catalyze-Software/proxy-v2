use super::storage_api::{StorageMethods, CLEAR_MEMORY_ID, GROUP_MEMBERS, MEMORY_MANAGER};
use canister_types::models::{api_error::ApiError, member_collection::MemberCollection};
use ic_stable_structures::StableBTreeMap;

pub struct GroupMemberStore;

pub const NAME: &str = "group_members";

impl StorageMethods<u64, MemberCollection> for GroupMemberStore {
    /// Get group members by key
    /// # Arguments
    /// * `key` - The key of the group for the members to get
    /// # Returns
    /// * `Result<(u64, GroupMembers), ApiError>` - The group if found, otherwise an error
    fn get(key: u64) -> Result<(u64, MemberCollection), ApiError> {
        GROUP_MEMBERS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple group members by key
    /// # Arguments
    /// * `ids` - The keys of the groups to get the members for
    /// # Returns
    /// * `Vec<(u64, GroupMembers)>` - The groups if found, otherwise an empty vector
    fn get_many(keys: Vec<u64>) -> Vec<(u64, MemberCollection)> {
        GROUP_MEMBERS.with(|data| {
            let mut groups = Vec::new();
            for key in keys {
                if let Some(group) = data.borrow().get(&key) {
                    groups.push((key, group));
                }
            }
            groups
        })
    }

    /// Find the members for a single group by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, GroupMembers)>` - The group if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, MemberCollection)>
    where
        F: Fn(&u64, &MemberCollection) -> bool,
    {
        GROUP_MEMBERS.with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all group members by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, GroupMembers)>` - The groups if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, MemberCollection)>
    where
        F: Fn(&u64, &MemberCollection) -> bool,
    {
        GROUP_MEMBERS.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

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
        GROUP_MEMBERS.with(|data| {
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
    fn insert(_value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message(
                "This value does not require a key to be inserted, use `insert_by_key` instead",
            ))
    }

    /// Update a single group by key
    /// # Arguments
    /// * `key` - The key of the group to update the members for
    /// * `value` - The GroupMembers to update
    /// # Returns
    /// * `Result<(u64, GroupMembers), ApiError>` - The updated group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if not returns an error
    fn update(key: u64, value: MemberCollection) -> Result<(u64, MemberCollection), ApiError> {
        GROUP_MEMBERS.with(|data| {
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

    /// Remove a single group by key
    /// # Arguments
    /// * `key` - The key of the group to remove
    /// # Returns
    /// * `bool` - True if the group was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        GROUP_MEMBERS.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        GROUP_MEMBERS.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(CLEAR_MEMORY_ID)),
            ))
        });
    }
}
