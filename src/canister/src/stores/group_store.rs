use std::cell::{Ref, RefCell, RefMut};

use ic_stable_structures::StableBTreeMap;

use crate::{
    entities::{
        group::{Group, PostGroup},
        member::Member,
    },
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

pub trait GroupStoreMethods {
    fn groups(&self) -> GroupStore;
    fn groups_mut(&mut self) -> MutableGroupStore;
}

impl GroupStoreMethods for GroupStoreRef {
    fn groups(&self) -> GroupStore {
        self.borrow()
    }

    fn groups_mut(&mut self) -> MutableGroupStore {
        self.borrow_mut()
    }
}
