use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use crate::models::{
    api_error::ApiError, attendee::Attendee, event::Event, friend_request::FriendRequest,
    group::Group, member::Member, profile::Profile, report::Report,
};

use super::{
    attendee_storage::AttendeeStore, event_storage::EventStore,
    friend_request_storage::FriendRequestStore, group_storage::GroupStore,
    member_storage::MemberStore, profile_storage::ProfileStore, report_storage::ReportStore,
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
    fn get(&self, id: K) -> Result<(K, V), ApiError>;
    fn get_many(&self, ids: Vec<K>) -> Vec<(K, V)>;
    fn find<F>(&self, filter: F) -> Option<(K, V)>
    where
        F: Fn(&V) -> bool;
    fn filter<F>(&self, filter: F) -> Vec<(K, V)>
    where
        F: Fn(&V) -> bool;
    fn insert(&mut self, entity: V) -> Result<(K, V), ApiError>;
    fn insert_by_key(&mut self, key: K, entity: V) -> Result<(K, V), ApiError>;
    fn update(&mut self, id: K, entity: V) -> Result<(K, V), ApiError>;
    fn remove(&mut self, id: K) -> bool;
}

thread_local! {
    static MEMORY_MANAGER: MemManagerStore =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PROFILES: StorageRef<Principal, Profile> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(PROFILES_MEMORY_ID)))
    );

    static PROFILES_MAP: StorageRef<Principal, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(PROFILES_MEMORY_ID)))
    );

    static FRIEND_REQUEST: StorageRef<u64, FriendRequest> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(FRIEND_REQUESTS_MEMORY_ID)))
    );

    static GROUPS: StorageRef<u64, Group> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(GROUPS_MEMORY_ID)))
    );

    static MEMBERS: StorageRef<Principal, Member> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(MEMBERS_MEMORY_ID)))
    );

    static EVENTS: StorageRef<u64, Event> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(EVENTS_MEMORY_ID)))
    );

    static ATTENDEES: StorageRef<Principal, Attendee> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(ATTENDEES_MEMORY_ID)))
    );

    static REPORTS: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(REPORTS_MEMORY_ID)))
    );

    static STATIC_FILES: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(STATIC_FILES_MEMORY_ID)))
    );

    static BOOSTED: StorageRef<u64, Report> = RefCell::new(
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

pub fn profiles<'a>() -> ProfileStore<'a> {
    ProfileStore::new(&PROFILES)
}

pub fn friend_requests<'a>() -> FriendRequestStore<'a> {
    FriendRequestStore::new(&FRIEND_REQUEST)
}

pub fn events<'a>() -> EventStore<'a> {
    EventStore::new(&EVENTS)
}

pub fn attendees<'a>() -> AttendeeStore<'a> {
    AttendeeStore::new(&ATTENDEES)
}

pub fn groups<'a>() -> GroupStore<'a> {
    GroupStore::new(&GROUPS)
}

pub fn members<'a>() -> MemberStore<'a> {
    MemberStore::new(&MEMBERS)
}

pub fn reports<'a>() -> ReportStore<'a> {
    ReportStore::new(&REPORTS)
}
