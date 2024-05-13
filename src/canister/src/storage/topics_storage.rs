use std::thread::LocalKey;

use canister_types::models::api_error::ApiError;
use ic_stable_structures::{memory_manager::MemoryId, StableBTreeMap};

use super::{
    storage_api::{
        StorageRef, INTERESTS, INTERESTS_MEMORY_ID, MEMORY_MANAGER, SKILLS, SKILLS_MEMORY_ID, TAGS,
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

trait StorageMethodsExt {
    fn store() -> &'static LocalKey<StorageRef<u64, String>>;
    fn store_name() -> String;
    fn memory_id() -> MemoryId;
}

impl StorageMethodsExt for TagsStore {
    fn store() -> &'static LocalKey<StorageRef<u64, String>> {
        &TAGS
    }

    fn store_name() -> String {
        TAGS_NAME.to_string()
    }

    fn memory_id() -> MemoryId {
        TAGS_MEMORY_ID
    }
}

impl StorageMethodsExt for InterestsStore {
    fn store() -> &'static LocalKey<StorageRef<u64, String>> {
        &INTERESTS
    }

    fn store_name() -> String {
        INTERESTS_NAME.to_string()
    }

    fn memory_id() -> MemoryId {
        INTERESTS_MEMORY_ID
    }
}

impl StorageMethodsExt for SkillsStore {
    fn store() -> &'static LocalKey<StorageRef<u64, String>> {
        &SKILLS
    }

    fn store_name() -> String {
        SKILLS_NAME.to_string()
    }

    fn memory_id() -> MemoryId {
        SKILLS_MEMORY_ID
    }
}

impl<T: StorageMethodsExt> StorageMethods<u64, String> for T {
    /// Get a single topic by id
    /// # Arguments
    /// * `id` - The id of the topic to get
    /// # Returns
    /// * `Result<Topic, ApiError>` - The topic if found, otherwise an error?
    fn get(id: u64) -> Result<(u64, String), ApiError> {
        Self::store().with(|data| {
            data.borrow()
                .get(&id)
                .ok_or(
                    ApiError::not_found()
                        .add_method_name("get")
                        .add_info(Self::store_name().as_str()),
                )
                .map(|value| (id, value))
        })
    }

    /// Get multiple topics by key
    /// # Arguments
    /// * `ids` - The keys of the topics to get
    /// # Returns
    /// * `Vec<Topic>` - The topics if found, otherwise an empty vector
    fn get_many(ids: Vec<u64>) -> Vec<Topic> {
        Self::store().with(|data| {
            let mut topics = Vec::new();
            for id in ids {
                if let Some(topic) = data.borrow().get(&id) {
                    topics.push((id, topic));
                }
            }
            topics
        })
    }

    /// Find a single topics by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<Topic>` - The topics if found, otherwise None
    fn find<F>(filter: F) -> Option<Topic>
    where
        F: Fn(&u64, &String) -> bool,
    {
        Self::store().with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all topics by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<Topic>` - The topics if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<Topic>
    where
        F: Fn(&u64, &String) -> bool,
    {
        Self::store().with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single topic
    /// # Arguments
    /// * `value` - The topic content to insert
    /// # Returns
    /// * `Result<Topic, ApiError>` - The inserted topic if successful, otherwise an error
    /// # Note
    /// Does check if a topic with the same key already exists, if so returns an error
    fn insert(value: String) -> Result<Topic, ApiError> {
        Self::store().with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(1);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(Self::store_name().as_str())
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
    fn insert_by_key(_key: u64, _value: String) -> Result<Topic, ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(Self::store_name().as_str())
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single topic by key
    /// # Arguments
    /// * `key` - The key of the topic to update
    /// * `value` - The topic to update
    /// # Returns
    /// * `Result<Topic, ApiError>` - The updated topic if successful, otherwise an error
    /// # Note
    /// Does check if a topic with the same key already exists, if not returns an error
    fn update(key: u64, value: String) -> Result<Topic, ApiError> {
        Self::store().with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(Self::store_name().as_str())
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single topic by key
    /// # Arguments
    /// * `key` - The key of the topic to remove
    /// # Returns
    /// * `bool` - True if the topic was removed, otherwise false
    /// # Note
    fn remove(key: u64) -> bool {
        Self::store().with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Clear all topics
    fn clear() {
        Self::store().with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(Self::memory_id())),
            ))
        });
    }
}
