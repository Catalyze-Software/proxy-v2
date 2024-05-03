use super::storage_api::{StorageMethods, MEMBERS, MEMBERS_MEMORY_ID, MEMORY_MANAGER};
use candid::Principal;
use canister_types::models::{api_error::ApiError, member::Member};
use ic_stable_structures::StableBTreeMap;

pub struct MemberStore;

pub const NAME: &str = "members";

impl StorageMethods<Principal, Member> for MemberStore {
    /// Get a single member by key
    /// # Arguments
    /// * `key` - The key of the member to get
    /// # Returns
    /// * `Result<(Principal, Member), ApiError>` - The member if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple members by key
    /// # Arguments
    /// * `ids` - The keys of the members to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, Member)> {
        MEMBERS.with(|data| {
            let mut members = Vec::new();
            for key in keys {
                if let Some(member) = data.borrow().get(&key) {
                    members.push((key, member));
                }
            }
            members
        })
    }

    /// Find a single member by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Member)>` - The member if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, Member)>
    where
        F: Fn(&Principal, &Member) -> bool,
    {
        MEMBERS.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all members by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Member)>` - The members if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, Member)>
    where
        F: Fn(&Principal, &Member) -> bool,
    {
        MEMBERS.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Member) -> Result<(Principal, Member), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single member by key
    /// # Arguments
    /// * `key` - The user principal as key of the member to insert
    /// * `value` - The member to insert
    /// # Returns
    /// * `Result<(Principal, Member), ApiError>` - The inserted member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Member) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            // Add member to the member collection

            Ok((key, value))
        })
    }

    /// Update a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to update
    /// * `value` - The member to update
    /// # Returns
    /// * `Result<(Principal, Member), ApiError>` - The updated member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if not returns an error
    fn update(key: Principal, value: Member) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
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

    /// Remove a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to remove
    /// # Returns
    /// * `bool` - True if the member was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        MEMBERS.with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all attendees
    fn clear() -> () {
        MEMBERS.with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(MEMBERS_MEMORY_ID)),
            ))
        });
    }
}
