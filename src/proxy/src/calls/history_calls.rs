use catalyze_shared::{CanisterResult, CellStorage};
use ic_cdk::query;

use crate::storage::history_point;

#[query]
fn get_history_point() -> CanisterResult<u64> {
    history_point().get()
}
