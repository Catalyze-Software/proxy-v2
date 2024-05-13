use canister_types::models::api_error::ApiError;
use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{
        StaticStorageRef, Storage, INTERESTS, INTERESTS_MEMORY_ID, SKILLS, SKILLS_MEMORY_ID, TAGS,
        TAGS_MEMORY_ID,
    },
    StorageMethods,
};

pub type Topic = (u64, String);

const TAGS_NAME: &str = "tags";
const INTERESTS_NAME: &str = "interests";
const SKILLS_NAME: &str = "skills";

pub struct TagsStore;
pub struct InterestsStore;
pub struct SkillsStore;

impl Storage<u64, String> for TagsStore {
    const NAME: &'static str = "tags";

    fn storage() -> StaticStorageRef<u64, String> {
        &TAGS
    }

    fn memory_id() -> MemoryId {
        TAGS_MEMORY_ID
    }
}

impl Storage<u64, String> for InterestsStore {
    const NAME: &'static str = "interests";

    fn storage() -> StaticStorageRef<u64, String> {
        &INTERESTS
    }

    fn memory_id() -> MemoryId {
        INTERESTS_MEMORY_ID
    }
}

impl Storage<u64, String> for SkillsStore {
    const NAME: &'static str = "skills";

    fn storage() -> StaticStorageRef<u64, String> {
        &SKILLS
    }

    fn memory_id() -> MemoryId {
        SKILLS_MEMORY_ID
    }
}

impl<T: Storage<u64, String>> StorageMethods<u64, String> for T {
    /// Insert a single topic
    /// # Arguments
    /// * `value` - The topic content to insert
    /// # Returns
    /// * `Result<Topic, ApiError>` - The inserted topic if successful, otherwise an error
    /// # Note
    /// Does check if a topic with the same key already exists, if so returns an error
    fn insert(value: String) -> Result<Topic, ApiError> {
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
