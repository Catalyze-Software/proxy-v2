use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use crate::entities::{
    attendee::Attendee,
    event::Event,
    group::Group,
    member::Member,
    profile::{FriendRequest, Profile},
    report::Report,
};

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
    fn insert(&self, entity: V) -> Result<V, String>;
    fn insert_by_key(&self, key: K, entity: V) -> Result<V, String>;
    fn update(&mut self, id: K, entity: V) -> Result<V, String>;
    fn remove(&mut self, id: K) -> bool;
}

thread_local! {
    pub static MEMORY_MANAGER: MemManagerStore =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PROFILES: StorageRef<String, Profile> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.profiles()))
    );

    static FRIEND_REQUEST: StorageRef<u64, FriendRequest> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.friend_requests()))
    );

    static GROUPS: StorageRef<u64, Group> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.groups()))
    );

    static MEMBERS: StorageRef<String, Member> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.members()))
    );

    static EVENTS: StorageRef<u64, Event> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.events()))
    );

    static ATTENDEES: StorageRef<String, Attendee> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.attendees()))
    );

    static REPORTS: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.reports()))
    );

    static STATIC_FILES: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.static_files()))
    );

    static BOOSTED: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.boosted()))
    );
}

pub trait MemManager {
    fn get_memory(&self, id: MemoryId) -> Memory;
    fn profiles(&self) -> Memory;
    fn friend_requests(&self) -> Memory;
    fn groups(&self) -> Memory;
    fn members(&self) -> Memory;
    fn events(&self) -> Memory;
    fn attendees(&self) -> Memory;
    fn reports(&self) -> Memory;
    fn static_files(&self) -> Memory;
    fn boosted(&self) -> Memory;
}

impl MemManager for MemManagerStore {
    fn get_memory(&self, id: MemoryId) -> Memory {
        self.borrow().get(id)
    }

    fn profiles(&self) -> Memory {
        self.get_memory(PROFILES_MEMORY_ID)
    }

    fn friend_requests(&self) -> Memory {
        self.get_memory(FRIEND_REQUESTS_MEMORY_ID)
    }

    fn groups(&self) -> Memory {
        self.get_memory(GROUPS_MEMORY_ID)
    }

    fn members(&self) -> Memory {
        self.get_memory(MEMBERS_MEMORY_ID)
    }

    fn events(&self) -> Memory {
        self.get_memory(EVENTS_MEMORY_ID)
    }

    fn attendees(&self) -> Memory {
        self.get_memory(ATTENDEES_MEMORY_ID)
    }

    fn reports(&self) -> Memory {
        self.get_memory(REPORTS_MEMORY_ID)
    }

    fn static_files(&self) -> Memory {
        self.get_memory(STATIC_FILES_MEMORY_ID)
    }

    fn boosted(&self) -> Memory {
        self.get_memory(BOOSTED_MEMORY_ID)
    }
}
