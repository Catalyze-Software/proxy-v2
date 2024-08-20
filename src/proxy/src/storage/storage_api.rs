use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    boosted::Boost,
    friend_request::FriendRequest,
    log::Logger,
    notification::Notification,
    reward::RewardableActivity,
    state::{init_btree, init_cell, init_memory_manager},
    user_notifications::UserNotifications,
    CellStorageRef, MemoryManagerStorage, StaticStorageRef, StorageRef,
};
use ic_stable_structures::{memory_manager::MemoryId, StableBTreeMap, Storable};

use super::IDStore;

/// The memory IDs for the different stores.
/// # Note
/// These IDs are used to identify the different stores in the `MemoryManager`.
/// # Warning
/// These IDs should not be changed. New IDs should be added to the end of the list
pub static NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static USER_NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(2);

pub static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static BOOSTED_MEMORY_ID: MemoryId = MemoryId::new(4);

pub static LOGS_MEMORY_ID: MemoryId = MemoryId::new(5);

pub static TAGS_MEMORY_ID: MemoryId = MemoryId::new(6);
pub static CATEGORIES_MEMORY_ID: MemoryId = MemoryId::new(7);
pub static SKILLS_MEMORY_ID: MemoryId = MemoryId::new(8);

pub static HISTORY_POINT_MEMORY_ID: MemoryId = MemoryId::new(9);

pub static IDS_MEMORY_ID: MemoryId = MemoryId::new(10);

pub static REWARD_BUFFER_MEMORY_ID: MemoryId = MemoryId::new(11);

pub static HISTORY_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(12);
pub static REWARD_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(13);
pub static PROFILE_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(14);
pub static REPORT_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(15);
pub static GROUP_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(16);
pub static EVENT_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(17);
pub static BOOSTED_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(18);

pub trait Storage<K: Storable + Ord + Clone, V: Storable + Clone> {
    const NAME: &'static str;

    fn memory_id() -> MemoryId;
    fn storage() -> StaticStorageRef<K, V>;
}

pub trait StorageQueryable<K: 'static + Storable + Ord + Clone, V: 'static + Storable + Clone>:
    Storage<K, V>
{
    /// Get the total number of entries
    /// # Returns
    /// * `u64` - The total number of entries
    fn size() -> u64 {
        Self::storage().with(|data| data.borrow().len())
    }

    /// Get a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to get
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The entity if found, otherwise an error
    fn get(key: K) -> Result<(K, V), ApiError> {
        Self::storage().with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(
                    ApiError::not_found()
                        .add_method_name("get")
                        .add_info(Self::NAME),
                )
                .map(|value| (key, value))
        })
    }

    /// Get multiple entities by key
    /// # Arguments
    /// * `keys` - The keys of the entities to get
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn get_many(keys: Vec<K>) -> Vec<(K, V)> {
        Self::storage().with(|data| {
            let mut entities = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    entities.push((key, value));
                }
            }
            entities
        })
    }

    /// Get all entities by key
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn get_all() -> Vec<(K, V)> {
        Self::storage().with(|data| data.borrow().iter().collect())
    }

    /// Find a single entity by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(K, V)>` - The entity if found, otherwise None
    fn find<F>(filter: F) -> Option<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        Self::storage().with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all entities by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        Self::storage().with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }
}

pub trait StorageInsertable<V: 'static + Storable + Clone>: Storage<u64, V> {
    /// Insert a single entity
    /// # Arguments
    /// * `value` - The entity to insert
    /// # Returns
    /// * `Result<(u64, V), ApiError>` - The inserted entity if successful, otherwise an error
    /// # Note
    /// Does check if a entity with the same key already exists, if so returns an error
    fn insert(value: V) -> Result<(u64, V), ApiError> {
        let key = IDStore::next(Self::NAME)?;

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
}

pub trait StorageInsertableByKey<K: 'static + Storable + Ord + Clone, V: 'static + Storable + Clone>:
    Storage<K, V>
{
    /// Insert a single entity by key
    /// # Arguments
    /// * `key` - The entity as key of the entity to insert
    /// * `value` - The entity to insert
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The inserted entity if successful, otherwise an error
    /// # Note
    /// Does check if a entity with the same key already exists, if so returns an error
    fn insert_by_key(key: K, value: V) -> Result<(K, V), ApiError> {
        Self::storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(Self::NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }
}

pub trait StorageUpdateable<K: 'static + Storable + Ord + Clone, V: 'static + Storable + Clone>:
    Storage<K, V>
{
    /// Update a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to update
    /// * `value` - The entity to update
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The updated entity if successful, otherwise an error
    /// # Note
    /// Does check if a entity with the same key already exists, if not returns an error
    fn update(key: K, value: V) -> Result<(K, V), ApiError> {
        Self::storage().with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(Self::NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to remove
    /// # Returns
    /// * `bool` - True if the entity was removed, otherwise false
    fn remove(key: K) -> bool {
        Self::storage().with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Remove a entities by keys
    /// # Arguments
    /// * `keys` - The keys of the entities to remove
    fn remove_many(keys: Vec<K>) {
        Self::storage().with(|data| {
            for key in keys {
                data.borrow_mut().remove(&key);
            }
        })
    }

    /// Clear all entities
    fn clear() {
        Self::storage().with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(Self::memory_id())),
            ))
        });
    }
}

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();

    pub static FRIEND_REQUEST: StorageRef<u64, FriendRequest> = init_btree(&MEMORY_MANAGER, FRIEND_REQUESTS_MEMORY_ID);
    pub static BOOSTED: StorageRef<u64, Boost> = init_btree(&MEMORY_MANAGER, BOOSTED_MEMORY_ID);
    pub static NOTIFICATIONS: StorageRef<u64, Notification> = init_btree(&MEMORY_MANAGER, NOTIFICATIONS_MEMORY_ID);
    pub static USER_NOTIFICATIONS: StorageRef<Principal, UserNotifications> = init_btree(&MEMORY_MANAGER, USER_NOTIFICATIONS_MEMORY_ID);
    pub static LOGS: StorageRef<u64, Logger> = init_btree(&MEMORY_MANAGER, LOGS_MEMORY_ID);
    pub static REWARD_BUFFER: StorageRef<u64, RewardableActivity> = init_btree(&MEMORY_MANAGER, REWARD_BUFFER_MEMORY_ID);

    pub static TAGS: StorageRef<u64, String> = init_btree(&MEMORY_MANAGER, TAGS_MEMORY_ID);
    pub static CATEGORIES: StorageRef<u64, String> = init_btree(&MEMORY_MANAGER, CATEGORIES_MEMORY_ID);
    pub static SKILLS: StorageRef<u64, String> = init_btree(&MEMORY_MANAGER, SKILLS_MEMORY_ID);
    pub static HISTORY_POINT: CellStorageRef<u64> = init_cell(&MEMORY_MANAGER, "history_point", HISTORY_POINT_MEMORY_ID);

    pub static IDS: StorageRef<String, u64> = init_btree(&MEMORY_MANAGER, IDS_MEMORY_ID);

    pub static HISTORY_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "history_canister_id", HISTORY_CANISTER_MEMORY_ID);
    pub static REWARD_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "reward_canister_id", REWARD_CANISTER_MEMORY_ID);
    pub static PROFILE_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "profile_canister_id", PROFILE_CANISTER_MEMORY_ID);
    pub static REPORT_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "report_canister_id", REPORT_CANISTER_MEMORY_ID);
    pub static GROUP_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "group_canister_id", GROUP_CANISTER_MEMORY_ID);
    pub static EVENT_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "event_canister_id", EVENT_CANISTER_MEMORY_ID);
    pub static BOOSTED_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "boosted_canister_id", BOOSTED_CANISTER_MEMORY_ID);
}
