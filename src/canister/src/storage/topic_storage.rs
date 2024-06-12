use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, CATEGORIES,
        CATEGORIES_MEMORY_ID, SKILLS, SKILLS_MEMORY_ID, TAGS, TAGS_MEMORY_ID,
    },
    StorageInsertable,
};

pub struct TagStore;
pub struct CategoryStore;
pub struct SkillStore;

impl Storage<u64, String> for TagStore {
    const NAME: &'static str = "tags";

    fn storage() -> StaticStorageRef<u64, String> {
        &TAGS
    }

    fn memory_id() -> MemoryId {
        TAGS_MEMORY_ID
    }
}

impl Storage<u64, String> for CategoryStore {
    const NAME: &'static str = "categories";

    fn storage() -> StaticStorageRef<u64, String> {
        &CATEGORIES
    }

    fn memory_id() -> MemoryId {
        CATEGORIES_MEMORY_ID
    }
}

impl Storage<u64, String> for SkillStore {
    const NAME: &'static str = "skills";

    fn storage() -> StaticStorageRef<u64, String> {
        &SKILLS
    }

    fn memory_id() -> MemoryId {
        SKILLS_MEMORY_ID
    }
}

impl<T: Storage<u64, String>> StorageQueryable<u64, String> for T {}
impl<T: Storage<u64, String>> StorageUpdateable<u64, String> for T {}
impl<T: Storage<u64, String>> StorageInsertable<String> for T {}
