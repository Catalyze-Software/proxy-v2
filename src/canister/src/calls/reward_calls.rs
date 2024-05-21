use crate::{helpers::guards::has_access, storage::RewardTimerStore};
use ic_cdk::query;

#[query(guard = "has_access")]
fn reward_timer_set() -> bool {
    RewardTimerStore::reward_timer_set()
}
