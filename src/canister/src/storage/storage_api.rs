use candid::Principal;
use canister_types::models::{
    api_error::ApiError, attendee::Attendee, boosted::Boost, event::Event,
    friend_request::FriendRequest, group::Group, member::Member, notification::Notification,
    profile::Profile, report::Report, user_notifications::UserNotifications,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

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

static NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(6);
static USER_NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(7);

static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(8);
static BOOSTED_MEMORY_ID: MemoryId = MemoryId::new(9);

// TODO:
/// The type of the key used in the user centric `StableBTreeMap` for the different stores.
/// # Note
/// This is just a `Principal` but renamed to `PrincipalIdentifier` to make it more clear.
/// Should be removed once the old data is migrated to the new data model
pub type PrincipalIdentifier = Principal;

// Temporary memory IDs for the maps which are needed for backward compatibility
// should be removed once the old data is migrated to the new data model

static PROFILES_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(109);
static GROUPS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(110);
static MEMBERS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(111);
static EVENTS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(112);
static ATTENDEES_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(113);
static REPORTS_IDENTIFIER_REF_MEMORY_ID: MemoryId = MemoryId::new(114);

/// A reference to a `StableBTreeMap` that is wrapped in a `RefCell`.
///# Generics
/// * `K` - The key type of the `StableBTreeMap`.
/// * `V` - The value type of the `StableBTreeMap`.
pub type StorageRef<K, V> = RefCell<StableBTreeMap<K, V, Memory>>;
type MemManagerStore = RefCell<MemoryManager<DefaultMemoryImpl>>;

pub trait StorageMethods<K, V> {
    fn get(id: K) -> Result<(K, V), ApiError>;
    fn get_many(ids: Vec<K>) -> Vec<(K, V)>;
    fn find<F>(filter: F) -> Option<(K, V)>
    where
        F: Fn(&K, &V) -> bool;
    fn filter<F>(filter: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool;
    fn insert(entity: V) -> Result<(K, V), ApiError>;
    fn insert_by_key(key: K, entity: V) -> Result<(K, V), ApiError>;
    fn update(id: K, entity: V) -> Result<(K, V), ApiError>;
    fn remove(id: K) -> bool;
}

/// A trait for the identifier reference maps.
/// # Generics
/// * `V` - The value type of the map. (Principal or u64)
/// # Note
/// This trait is used to define the methods that are common to the identifier reference maps.
/// Temporary trait for backward compatibility
pub trait IdentifierRefMethods<V> {
    fn new_identifier() -> PrincipalIdentifier;
    fn get_id_by_identifier(key: &PrincipalIdentifier) -> Option<V>;
    fn get_identifier_by_id(value: &V) -> Option<PrincipalIdentifier>;
    fn insert_identifier_ref(value: V) -> Result<V, ApiError>;
    fn remove_identifier_ref(key: &PrincipalIdentifier) -> bool;
}

thread_local! {
    pub static MEMORY_MANAGER: MemManagerStore =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PROFILES: StorageRef<Principal, Profile> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROFILES_MEMORY_ID)))
    );

    pub static FRIEND_REQUEST: StorageRef<u64, FriendRequest> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(FRIEND_REQUESTS_MEMORY_ID)))
    );

    pub static GROUPS: StorageRef<u64, Group> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(GROUPS_MEMORY_ID)))
    );

    pub static MEMBERS: StorageRef<Principal, Member> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(MEMBERS_MEMORY_ID)))
    );

    pub static EVENTS: StorageRef<u64, Event> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(EVENTS_MEMORY_ID)))
    );

    pub static ATTENDEES: StorageRef<Principal, Attendee> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(ATTENDEES_MEMORY_ID)))
    );

    pub static REPORTS: StorageRef<u64, Report> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(REPORTS_MEMORY_ID)))
    );

    pub static BOOSTED: StorageRef<u64, Boost> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(BOOSTED_MEMORY_ID)))
    );

    pub static NOTIFICATIONS: StorageRef<u64, Notification> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(NOTIFICATIONS_MEMORY_ID)))
    );

    pub static USER_NOTIFICATIONS: StorageRef<Principal, UserNotifications> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(USER_NOTIFICATIONS_MEMORY_ID)))
    );

    // TODO:
    // Temporary memories for the maps which are needed for backward compatibility
    // should be removed once the old data is migrated to the new data model
    pub static PROFILES_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROFILES_IDENTIFIER_REF_MEMORY_ID)))
    );

    pub static GROUPS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(GROUPS_IDENTIFIER_REF_MEMORY_ID)))
    );

    pub static MEMBERS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(MEMBERS_IDENTIFIER_REF_MEMORY_ID)))
    );

    pub static EVENTS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(EVENTS_IDENTIFIER_REF_MEMORY_ID)))
    );

    pub static ATTENDEES_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(ATTENDEES_IDENTIFIER_REF_MEMORY_ID)))
    );

    pub static REPORTS_IDENTIFIER_REF: StorageRef<PrincipalIdentifier, u64> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(REPORTS_IDENTIFIER_REF_MEMORY_ID)))
    );

}
