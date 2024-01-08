use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

// The memory IDs for the different stores.
// # Note
// These IDs are used to identify the different stores in the `MemoryManager`.
// # Warning
// These IDs should not be changed. New IDs should be added to the end of the list
pub static PROFILES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static GROUPS_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static ATTENDEES_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static EVENTS_MEMORY_ID: MemoryId = MemoryId::new(4);
pub static MEMBERS_MEMORY_ID: MemoryId = MemoryId::new(5);
pub static REPORTS_MEMORY_ID: MemoryId = MemoryId::new(6);

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}
