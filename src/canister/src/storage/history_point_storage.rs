use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{HISTORY_POINT, HISTORY_POINT_MEMORY_ID},
    CellStorage, CellStorageRef,
};

pub struct HistoryPointStorage;

impl CellStorage<u64> for HistoryPointStorage {
    const NAME: &'static str = "history_point";

    fn storage() -> CellStorageRef<u64> {
        &HISTORY_POINT
    }

    fn memory_id() -> MemoryId {
        HISTORY_POINT_MEMORY_ID
    }
}
