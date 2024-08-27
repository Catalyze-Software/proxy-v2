use catalyze_shared::CanisterResult;
use ic_cdk::query;

use crate::storage::global;

#[query]
async fn get_history_point() -> CanisterResult<u64> {
    global().get_history_point().await
}
