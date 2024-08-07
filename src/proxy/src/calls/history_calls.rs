use catalyze_shared::CanisterResult;
use ic_cdk::query;

use crate::storage::{CellStorage, HistoryPointStorage};

#[query]
fn get_history_point() -> CanisterResult<u64> {
    HistoryPointStorage::get()
}
