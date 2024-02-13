use std::{cell::RefCell, thread::LocalKey};

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use models::models::{
    api_error::ApiError, attendee::Attendee, boosted::Boosted, event::Event,
    friend_request::FriendRequest, group::Group, member::Member, profile::Profile, report::Report,
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

// TODO:
/// The type of the key used in the user centric `StableBTreeMap` for the different stores.
/// # Note
/// This is just a `Principal` but renamed to `PrincipalIdentifier` to make it more clear.
/// Should be removed once the old data is migrated to the new data model
pub type PrincipalIdentifier = Principal;

// Temporary memory IDs for the maps which are needed for backward compatibility
// should be removed once the old data is migrated to the new data model

static PROFILES_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(9);
static GROUPS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(10);
static MEMBERS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(11);
static EVENTS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(12);
static ATTENDEES_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(13);
static REPORTS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(14);

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
        F: Fn(&K, &V) -> bool;
    fn filter<F>(&self, filter: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool;
    fn insert(&mut self, entity: V) -> Result<(K, V), ApiError>;
    fn insert_by_key(&mut self, key: K, entity: V) -> Result<(K, V), ApiError>;
    fn update(&mut self, id: K, entity: V) -> Result<(K, V), ApiError>;
    fn remove(&mut self, id: K) -> bool;
}

/// A trait for the identifier reference maps.
/// # Generics
/// * `V` - The value type of the map. (Principal or u64)
/// # Note
/// This trait is used to define the methods that are common to the identifier reference maps.
/// Temporary trait for backward compatibility
pub trait IdentifierRefMethods<V> {
    fn new_identifier(&self) -> PrincipalIdentifier;
    fn get_id_by_identifier(&self, key: &PrincipalIdentifier) -> Option<V>;
    fn get_identifier_by_id(&self, value: &V) -> Option<PrincipalIdentifier>;
    fn insert_identifier_ref(&mut self, value: V) -> Result<V, ApiError>;
    fn remove_identifier_ref(&mut self, key: &PrincipalIdentifier) -> bool;
}

thread_local! {
    static MEMORY_MANAGER: MemManagerStore =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PROFILES: StorageRef<Principal, Profile> = RefCell::new(
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

    static STATIC_FILES: StorageRef<u64, Vec<u8>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(STATIC_FILES_MEMORY_ID)))
    );

    static BOOSTED: StorageRef<u64, Boosted> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(BOOSTED_MEMORY_ID)))
    );

    // TODO:
    // Temporary memories for the maps which are needed for backward compatibility
    // should be removed once the old data is migrated to the new data model
    static PROFILES_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(PROFILES_IDENTIFIER_REF_MEMORY_ID)))
    );

    static GROUPS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(GROUPS_IDENTIFIER_REF_MEMORY_ID)))
    );

    static MEMBERS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(MEMBERS_IDENTIFIER_REF_MEMORY_ID)))
    );

    static EVENTS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(EVENTS_IDENTIFIER_REF_MEMORY_ID)))
    );

    static ATTENDEES_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(ATTENDEES_IDENTIFIER_REF_MEMORY_ID)))
    );

    static REPORTS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.get_memory(REPORTS_IDENTIFIER_REF_MEMORY_ID)))
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
    ProfileStore::new(&PROFILES, &PROFILES_IDENTIFIER_REF)
}

pub fn events<'a>() -> EventStore<'a> {
    EventStore::new(&EVENTS, &EVENTS_IDENTIFIER_REF)
}

pub fn attendees<'a>() -> AttendeeStore<'a> {
    AttendeeStore::new(&ATTENDEES, &ATTENDEES_IDENTIFIER_REF)
}

pub fn groups<'a>() -> GroupStore<'a> {
    GroupStore::new(&GROUPS, &GROUPS_IDENTIFIER_REF)
}

pub fn members<'a>() -> MemberStore<'a> {
    MemberStore::new(&MEMBERS, &MEMBERS_IDENTIFIER_REF)
}

pub fn reports<'a>() -> ReportStore<'a> {
    ReportStore::new(&REPORTS, &REPORTS_IDENTIFIER_REF)
}

pub fn friend_requests<'a>() -> FriendRequestStore<'a> {
    FriendRequestStore::new(&FRIEND_REQUEST)
}

pub fn static_files<'a>() -> LocalKey<StorageRef<u64, Vec<u8>>> {
    STATIC_FILES
}

pub fn boosted<'a>() -> LocalKey<StorageRef<u64, Boosted>> {
    BOOSTED
}
