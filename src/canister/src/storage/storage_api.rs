use candid::Principal;
use canister_types::models::{
    api_error::ApiError, attendee::Attendee, boosted::Boost, event::Event,
    event_collection::EventCollection, friend_request::FriendRequest, group::Group, log::Logger,
    member::Member, member_collection::MemberCollection, notification::Notification,
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
pub static PROFILES_MEMORY_ID: MemoryId = MemoryId::new(0);

pub static GROUPS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static MEMBERS_MEMORY_ID: MemoryId = MemoryId::new(2);

pub static EVENTS_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static ATTENDEES_MEMORY_ID: MemoryId = MemoryId::new(4);

pub static REPORTS_MEMORY_ID: MemoryId = MemoryId::new(5);

pub static NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(6);
pub static USER_NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(7);

pub static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(8);
pub static BOOSTED_MEMORY_ID: MemoryId = MemoryId::new(9);

pub static GROUP_MEMBERS_MEMORY_ID: MemoryId = MemoryId::new(10);
pub static EVENT_ATTENDEES_MEMORY_ID: MemoryId = MemoryId::new(11);
pub static GROUP_EVENTS_MEMORY_ID: MemoryId = MemoryId::new(12);

pub static LOGS_MEMORY_ID: MemoryId = MemoryId::new(13);

pub static TAGS_MEMORY_ID: MemoryId = MemoryId::new(14);
pub static INTERESTS_MEMORY_ID: MemoryId = MemoryId::new(15);
pub static SKILLS_MEMORY_ID: MemoryId = MemoryId::new(16);

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
    fn clear();
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

    pub static LOGS: StorageRef<u64, Logger> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(LOGS_MEMORY_ID)))
    );

    // Collections for more performant lookup
    pub static GROUP_MEMBERS: StorageRef<u64, MemberCollection> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(GROUP_MEMBERS_MEMORY_ID)))
    );

    pub static GROUP_EVENTS: StorageRef<u64, EventCollection> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(GROUP_EVENTS_MEMORY_ID)))
    );

    pub static EVENT_ATTENDEES: StorageRef<u64, MemberCollection> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(EVENT_ATTENDEES_MEMORY_ID)))
    );

    pub static TAGS: StorageRef<u64, String> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(TAGS_MEMORY_ID)))
    );

    pub static INTERESTS: StorageRef<u64, String> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(INTERESTS_MEMORY_ID)))
    );

    pub static SKILLS: StorageRef<u64, String> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(SKILLS_MEMORY_ID)))
    );

}
