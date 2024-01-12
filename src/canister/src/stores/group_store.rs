use std::cell::{Ref, RefCell, RefMut};

use ic_stable_structures::StableBTreeMap;

use crate::{
    entities::{group::Group, member::Member},
    stores::memory_store::{MemManager, MEMORY_MANAGER},
};

use super::memory_store::Memory;

type GroupStoreRef = RefCell<StableBTreeMap<u64, Group, Memory>>;

type GroupStore<'a> = Ref<'a, StableBTreeMap<u64, Group, Memory>>;
type MutableGroupStore<'a> = RefMut<'a, StableBTreeMap<u64, Group, Memory>>;

type MemberStore = RefCell<StableBTreeMap<String, Member, Memory>>;

thread_local! {
    /// The `groups` store.
    /// # Note
    /// This store is used to keep track of the groups that have been created.
    pub static GROUPS: GroupStoreRef = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.groups()))
    );

    /// The `members` store.
    /// # Note
    /// This store is used to keep track of the members of groups.
    pub static MEMBERS: MemberStore = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.members()))
    );
}

pub trait GroupManager {
    fn groups(&self) -> GroupStore;
    fn groups_mut(&mut self) -> MutableGroupStore;
    // fn get(&self, key: &u64) -> Option<Group>;
    // fn add(&self, value: Group);
    // fn update(&self, key: &u64, value: Group);
    // fn remove(&self, key: &u64);
}

impl GroupManager for GroupStoreRef {
    fn groups(&self) -> GroupStore {
        self.borrow()
    }

    fn groups_mut(&mut self) -> MutableGroupStore {
        self.borrow_mut()
    }
}

// //// OLD FOR REFERERNCE
// /// The `Groups` struct.
// /// # Note
// /// This struct is used to interact with the `groups` store.
// pub struct Groups;

// impl Groups {
//     /// Returns the `group` associated with the given key.
//     /// # Arguments
//     /// * `key` - The key to get the `group` at in `u64` format.
//     /// # Returns
//     /// * `Some(Group)` - The `group` associated with the given key.
//     /// * `None` - If no `group` is associated with the given key.
//     /// # Panics
//     /// Panics if the `group` associated with the given key is not a `group`.
//     pub fn get(key: &u64) -> Option<Group> {
//         GROUPS.with(|p| p.borrow().get(key))
//     }

//     /// Inserts or updates a `group` into the store at a given key's position.
//     /// # Arguments
//     /// * `key` - The key to insert the `group` at in `u64` format.
//     /// * `value` - The `group` to insert.
//     /// # Panics
//     /// Panics if the `group` associated with the given key is not a `group`.
//     /// # Note
//     /// This function will overwrite the `group` at the given key's position if one already exists.
//     pub fn insert(key: u64, value: Group) {
//         GROUPS.with(|p| p.borrow_mut().insert(key, value));
//     }

//     /// Removes a `group` from the store at a given key's position.
//     /// # Arguments
//     /// * `key` - The key to insert the `group` at in `u64` format.
//     /// # Panics
//     /// Panics if the `group` associated with the given key is not a `group`.
//     /// # Note
//     /// This function will do nothing if no `group` is associated with the given key.
//     pub fn remove(key: &u64) {
//         GROUPS.with(|p| p.borrow_mut().remove(key));
//     }
// }
