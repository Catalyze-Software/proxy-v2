use crate::{
    helpers::guards::is_developer,
    logic::reward_buffer_logic::send_reward_data,
    storage::{RewardBufferStore, RewardTimerStore, StorageQueryable},
};
use catalyze_shared::reward::RewardableActivityResponse;
use ic_cdk::{query, update};

#[query]
fn reward_timer_next_trigger() -> Option<u64> {
    RewardTimerStore::next_trigger()
}

#[query]
fn read_reward_buffer() -> Vec<RewardableActivityResponse> {
    RewardBufferStore::get_all()
        .into_iter()
        .map(|(_, v)| v.into())
        .collect()
}

#[update(guard = "is_developer")]
async fn _dev_send_reward_data() {
    send_reward_data().await;
}
