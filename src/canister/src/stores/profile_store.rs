use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::StableBTreeMap;

use crate::{
    entities::profile::{FriendRequest, Profile},
    stores::memory_store::StableMemory,
};

use super::memory_store::Memory;

thread_local! {
    /// The `profiles` store.
    /// # Note
    /// This store is used to keep track of the profiles of users.
    static PROFILES: RefCell<StableBTreeMap<String, Profile, Memory>> = RefCell::new(
        StableBTreeMap::init(StableMemory::profiles())
    );
    /// The `friend_requests` store.
    /// # Note
    /// This store is used to keep track of friend requests that have been sent to a user.
    pub static FRIEND_REQUEST: RefCell<StableBTreeMap<u64, FriendRequest, Memory>> = RefCell::new(
        StableBTreeMap::init(StableMemory::friend_requests())
    );
}

/// The `Profiles` struct.
/// # Note
/// This struct is used to interact with the `profiles` store.
pub struct Profiles;

impl Profiles {
    /// Returns the `profile` associated with the given key.
    /// # Arguments
    /// * `key` - The key to get the `profile` at in `Principal` format.
    /// # Returns
    /// * `Some(Profile)` - The `profile` associated with the given key.
    /// * `None` - If no `profile` is associated with the given key.
    /// # Panics
    /// Panics if the `profile` associated with the given key is not a `Profile`.
    pub fn get(key: &Principal) -> Option<Profile> {
        PROFILES.with(|p| p.borrow().get(&key.to_string()))
    }

    /// Inserts or updates a `profile` into the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to insert the `profile` at in `Principal` format.
    /// * `value` - The `profile` to insert.
    /// # Panics
    /// Panics if the `profile` associated with the given key is not a `Profile`.
    /// # Note
    /// This function will overwrite the `profile` at the given key's position if one already exists.
    pub fn insert(key: &Principal, value: Profile) {
        PROFILES.with(|p| p.borrow_mut().insert(key.to_string(), value));
    }

    /// Removes a `profile` from the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to remove the `profile` at in `Principal` format.
    /// # Panics
    /// Panics if the `profile` associated with the given key is not a `Profile`.
    /// # Note
    /// This function will do nothing if no `profile` is associated with the given key.
    pub fn remove(key: &Principal) {
        PROFILES.with(|p| p.borrow_mut().remove(&key.to_string()));
    }
}
