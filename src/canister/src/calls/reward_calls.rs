use crate::{
    helpers::guards::has_access,
    storage::{RewardStore, RewardTimerStore},
};
use canister_types::models::reward::RewardableActivity;
use ic_cdk::query;

#[query(guard = "has_access")]
fn reward_timer_set() -> bool {
    RewardTimerStore::reward_timer_set()
}

#[query(guard = "has_access")]
fn read_reward_buffer() -> Vec<RewardableActivity> {
    RewardStore::get_all().into_iter().map(|(_, v)| v).collect()
}
