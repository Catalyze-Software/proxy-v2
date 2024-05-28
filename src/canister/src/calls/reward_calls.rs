use crate::{
    helpers::guards::{is_developer, is_monitor},
    storage::{EventStore, GroupStore, RewardBufferStore, RewardTimerStore, StorageQueryable},
};
use canister_types::models::reward::RewardableActivity;
use ic_cdk::{query, update};

#[query(guard = "is_monitor")]
fn reward_timer_next_trigger() -> Option<u64> {
    RewardTimerStore::next_trigger()
}

#[query(guard = "is_monitor")]
fn read_reward_buffer() -> Vec<RewardableActivity> {
    RewardBufferStore::get_all()
        .into_iter()
        .map(|(_, v)| v)
        .collect()
}

// testers
#[update(guard = "is_developer")]
fn fill_buffer() {
    let group_ids = GroupStore::get_all()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<u64>>();

    for i in &group_ids {
        RewardBufferStore::notify_group_count_changed(*i);
    }

    for i in &group_ids {
        RewardBufferStore::notify_group_is_active(*i);
    }

    let event_ids = EventStore::get_all()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<u64>>();

    for i in event_ids {
        RewardBufferStore::notify_event_attendance(i);
    }
}

#[update(guard = "is_developer")]
async fn send_reward_data() {
    crate::logic::reward_buffer_logic::send_reward_data().await
}
