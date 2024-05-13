use super::storage_api::{StaticStorageRef, Storage, StorageMethods, MEMBERS, MEMBERS_MEMORY_ID};
use candid::Principal;
use canister_types::models::{api_error::ApiError, member::Member};
use ic_stable_structures::memory_manager::MemoryId;

pub struct MemberStore;

impl Storage<Principal, Member> for MemberStore {
    const NAME: &'static str = "members";

    fn storage() -> StaticStorageRef<Principal, Member> {
        &MEMBERS
    }

    fn memory_id() -> MemoryId {
        MEMBERS_MEMORY_ID
    }
}

impl StorageMethods<Principal, Member> for MemberStore {
    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Member) -> Result<(Principal, Member), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(Self::NAME)
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
        Self::storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            // Add member to the member collection

            Ok((key, value))
        })
    }
}
