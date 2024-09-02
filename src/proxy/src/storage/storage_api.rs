use candid::Principal;
use catalyze_shared::{
    state::{init_btree, init_cell, init_memory_manager},
    CellStorageRef, MemoryManagerStorage, StorageRef,
};
use ic_stable_structures::memory_manager::MemoryId;

/// The memory IDs for the different stores.
/// # Note
/// These IDs are used to identify the different stores in the `MemoryManager`.
/// # Warning
/// These IDs should not be changed. New IDs should be added to the end of the list
pub static NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static USER_NOTIFICATIONS_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static IDS_MEMORY_ID: MemoryId = MemoryId::new(4);

pub static HISTORY_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(5);
pub static PROFILE_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(6);
pub static REPORT_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(7);
pub static GROUP_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(8);
pub static EVENT_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(9);
pub static BOOSTED_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(10);
pub static TOPIC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(11);
pub static FRIEND_REQUEST_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(12);
pub static GLOBAL_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(13);
pub static NOTIFICATION_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(14);
pub static TRANSACTION_HANDLER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(15);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();

    pub static IDS: StorageRef<String, u64> = init_btree(&MEMORY_MANAGER, IDS_MEMORY_ID);

    pub static HISTORY_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "history_canister_id", HISTORY_CANISTER_MEMORY_ID);
    pub static PROFILE_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "profile_canister_id", PROFILE_CANISTER_MEMORY_ID);
    pub static REPORT_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "report_canister_id", REPORT_CANISTER_MEMORY_ID);
    pub static GROUP_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "group_canister_id", GROUP_CANISTER_MEMORY_ID);
    pub static EVENT_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "event_canister_id", EVENT_CANISTER_MEMORY_ID);
    pub static BOOSTED_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "boosted_canister_id", BOOSTED_CANISTER_MEMORY_ID);
    pub static NOTIFICATION_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "notification_canister_id", NOTIFICATION_CANISTER_MEMORY_ID);
    pub static TOPIC_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "topic_canister_id", TOPIC_CANISTER_MEMORY_ID);
    pub static FRIEND_REQUEST_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "friend_request_canister_id", FRIEND_REQUEST_CANISTER_MEMORY_ID);
    pub static GLOBAL_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "global_canister_id", GLOBAL_CANISTER_MEMORY_ID);
    pub static TRANSACTION_HANDLER_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "transaction_handler_canister_id", TRANSACTION_HANDLER_CANISTER_MEMORY_ID);
}
