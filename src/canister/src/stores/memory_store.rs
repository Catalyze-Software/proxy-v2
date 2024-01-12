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
static PROFILES_MEMORY_ID: MemoryId = MemoryId::new(0);

static GROUPS_MEMORY_ID: MemoryId = MemoryId::new(1);
static MEMBERS_MEMORY_ID: MemoryId = MemoryId::new(2);

static EVENTS_MEMORY_ID: MemoryId = MemoryId::new(3);
static ATTENDEES_MEMORY_ID: MemoryId = MemoryId::new(4);

static REPORTS_MEMORY_ID: MemoryId = MemoryId::new(5);

static STATIC_FILES_MEMORY_ID: MemoryId = MemoryId::new(6);

static FRIEND_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(7);
static BOOSTED_MEMORY_ID: MemoryId = MemoryId::new(8);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub struct StableMemory;

impl StableMemory {
    fn get_memory(id: MemoryId) -> Memory {
        MEMORY_MANAGER.with(|m| m.borrow().get(id))
    }

    pub fn profiles() -> Memory {
        Self::get_memory(PROFILES_MEMORY_ID)
    }
    pub fn groups() -> Memory {
        Self::get_memory(GROUPS_MEMORY_ID)
    }

    pub fn members() -> Memory {
        Self::get_memory(MEMBERS_MEMORY_ID)
    }

    pub fn events() -> Memory {
        Self::get_memory(EVENTS_MEMORY_ID)
    }

    pub fn attendees() -> Memory {
        Self::get_memory(ATTENDEES_MEMORY_ID)
    }

    pub fn reports() -> Memory {
        Self::get_memory(REPORTS_MEMORY_ID)
    }

    pub fn friend_requests() -> Memory {
        Self::get_memory(FRIEND_REQUESTS_MEMORY_ID)
    }

    pub fn static_files() -> Memory {
        Self::get_memory(STATIC_FILES_MEMORY_ID)
    }

    pub fn boosted() -> Memory {
        Self::get_memory(BOOSTED_MEMORY_ID)
    }
}
