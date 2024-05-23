use crate::{
    helpers::guards::is_developer,
    storage::{RewardBufferStore, RewardTimerStore},
};
use canister_types::models::reward::RewardableActivity;
use ic_cdk::{query, update};

#[query(guard = "is_developer")]
fn reward_timer_next_trigger() -> Option<u64> {
    RewardTimerStore::next_trigger()
}

#[query(guard = "is_developer")]
fn read_reward_buffer() -> Vec<RewardableActivity> {
    RewardBufferStore::get_all()
        .into_iter()
        .map(|(_, v)| v)
        .collect()
}

// testers
#[update(guard = "is_developer")]
fn fill_buffer() {
    for i in 1..=3 {
        RewardBufferStore::notify_group_count_changed(i);
    }
    for i in 1..=3 {
        RewardBufferStore::notify_group_is_active(i);
    }
    for i in 1..=3 {
        RewardBufferStore::notify_event_attendance(i);
    }
}

#[update(guard = "is_developer")]
async fn send_reward_data() {
    crate::logic::reward_buffer_logic::send_reward_data().await
}
