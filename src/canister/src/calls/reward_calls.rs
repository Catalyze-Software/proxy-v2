use crate::{
    helpers::guards::{is_developer, is_monitor},
    logic::reward_buffer_logic::send_reward_data,
    storage::{
        reward_canister_storage::RewardCanisterStorage, CellStorage, GroupStore, RewardBufferStore,
        RewardTimerStore, StorageQueryable,
    },
};
use candid::Principal;
use canister_types::models::{api_error::ApiError, reward::RewardableActivity};
use ic_cdk::{query, update};

#[update(guard = "is_developer")]
fn set_reward_canister(principal: Principal) -> Result<Principal, ApiError> {
    RewardCanisterStorage::set(principal)
}

#[query(guard = "is_developer")]
fn get_reward_canister() -> Result<Principal, ApiError> {
    RewardCanisterStorage::get()
}

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
        RewardBufferStore::notify_group_member_count_changed(*i);
    }
}

#[update(guard = "is_developer")]
async fn _dev_send_reward_data() {
    send_reward_data().await
}
