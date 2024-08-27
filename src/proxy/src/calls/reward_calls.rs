use crate::storage::global;
use catalyze_shared::{reward::RewardableActivityResponse, CanisterResult};
use ic_cdk::query;

#[query]
async fn reward_timer_next_trigger() -> CanisterResult<u64> {
    global().reward_timer_next_trigger().await
}

#[query]
async fn read_reward_buffer() -> CanisterResult<Vec<RewardableActivityResponse>> {
    global().read_reward_buffer().await
}
