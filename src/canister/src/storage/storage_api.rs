use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use crate::models::{
    attendee::Attendee, event::Event, friend_request::FriendRequest, group::Group, member::Member,
    profile::Profile, report::Report,
};

use super::profile_storage::ProfileStore;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

/// The memory IDs for the different stores.
/// # Note
/// These IDs are used to identify the different stores in the `MemoryManager`.
/// # Warning
/// These IDs should not be changed. New IDs should be added to the end of the list
static PROFILES_MEMORY_ID: MemoryId = MemoryId::new(0);

static GROUPS_MEMORY_ID: MemoryId = MemoryId::new(1);
static MEMBERS_MEMORY_ID: MemoryId = MemoryId::new(2);

static EVENTS_MEMORY_ID: MemoryId = MemoryId::new(3);
static ATTENDEES_MEMORY_ID: MemoryId = MemoryId::new(4);

static REPORTS_MEMORY_ID: MemoryId = MemoryId::new(5);

static STATIC_FILES_MEMORY_ID: MemoryId = MemoryId::new(6);

static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(7);
static BOOSTED_MEMORY_ID: MemoryId = MemoryId::new(8);

/// A reference to a `StableBTreeMap` that is wrapped in a `RefCell`.
///# Generics
/// * `K` - The key type of the `StableBTreeMap`.
/// * `V` - The value type of the `StableBTreeMap`.
pub type StorageRef<K, V> = RefCell<StableBTreeMap<K, V, Memory>>;
type MemManagerStore = RefCell<MemoryManager<DefaultMemoryImpl>>;

pub trait StorageMethods<K, V> {
    fn get(&self, id: K) -> Result<V, String>;
    // fn find<F>(&self, filter: F) -> Option<(K, V)>
    // where
    //     F: Fn(&V) -> bool;
    // fn filter<F>(&self, filter: F) -> Vec<(K, V)>
    // where
    //     F: Fn(&V) -> bool;
    fn insert(&mut self, entity: V) -> Result<V, String>;
    fn insert_by_key(&mut self, key: K, entity: V) -> Result<V, String>;
    fn update(&mut self, id: K, entity: V) -> Result<V, String>;
    fn remove(&mut self, id: K) -> bool;
}

thread_local! {
    pub static MEMORY_MANAGER: MemManagerStore =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PROFILES: StorageRef<String, Profile> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(PROFILES_MEMORY_ID)))
    );

    pub static FRIEND_REQUEST: StorageRef<u64, FriendRequest> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(FRIEND_REQUESTS_MEMORY_ID)))
    );

    pub static GROUPS: StorageRef<u64, Group> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(GROUPS_MEMORY_ID)))
    );

    pub static MEMBERS: StorageRef<String, Member> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(MEMBERS_MEMORY_ID)))
    );

    pub static EVENTS: StorageRef<u64, Event> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(EVENTS_MEMORY_ID)))
    );

    pub static ATTENDEES: StorageRef<String, Attendee> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(ATTENDEES_MEMORY_ID)))
    );

    pub static REPORTS: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(REPORTS_MEMORY_ID)))
    );

    pub static STATIC_FILES: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(STATIC_FILES_MEMORY_ID)))
    );

    pub static BOOSTED: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(BOOSTED_MEMORY_ID)))
    );
}

pub trait MemManager {
    fn get_memory(&self, id: MemoryId) -> Memory;
}

impl MemManager for MemManagerStore {
    fn get_memory(&self, id: MemoryId) -> Memory {
        self.borrow().get(id)
    }
}

pub fn profiles() -> ProfileStore {
    ProfileStore::new(PROFILES)
}
