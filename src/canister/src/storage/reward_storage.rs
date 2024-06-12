use super::{
    storage_api::{StaticStorageRef, Storage, REWARD_BUFFER, REWARD_BUFFER_MEMORY_ID},
    StorageInsertable, StorageQueryable, StorageUpdateable,
};
use crate::logic::reward_buffer_logic::send_reward_data;
use candid::Principal;
use canister_types::models::reward::{Activity, RewardableActivity};
use ic_cdk::{api::time, spawn};
use ic_cdk_timers::set_timer_interval;
use ic_stable_structures::memory_manager::MemoryId;
use std::{cell::RefCell, time::Duration};

// Interval for sending reward activities to Reward Canister
const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

// timer to periodically process the reward buffer
thread_local! {
   pub static REWARD_TIMER: RefCell<Option<u64>> = const { RefCell::new(None) };
}

pub struct RewardTimerStore;

impl RewardTimerStore {
    pub fn start_reward_timer() {
        let _ = set_timer_interval(INTERVAL, move || {
            spawn(send_reward_data());
        });

        let next_trigger = time() + INTERVAL.as_nanos() as u64;

        REWARD_TIMER.with(|t| *t.borrow_mut() = Some(next_trigger));
    }

    pub fn next_trigger() -> Option<u64> {
        REWARD_TIMER.with(|t| *t.borrow())
    }

    pub fn set_next_trigger() {
        REWARD_TIMER.with(|t| {
            let next_trigger = time() + INTERVAL.as_nanos() as u64;
            *t.borrow_mut() = Some(next_trigger);
        });
    }
}

pub struct RewardBufferStore;

impl Storage<u64, RewardableActivity> for RewardBufferStore {
    const NAME: &'static str = "profiles";

    fn storage() -> StaticStorageRef<u64, RewardableActivity> {
        &REWARD_BUFFER
    }

    fn memory_id() -> MemoryId {
        REWARD_BUFFER_MEMORY_ID
    }
}

impl StorageQueryable<u64, RewardableActivity> for RewardBufferStore {}
impl StorageUpdateable<u64, RewardableActivity> for RewardBufferStore {}
impl StorageInsertable<RewardableActivity> for RewardBufferStore {}

impl RewardBufferStore {
    pub fn notify_group_member_count_changed(group_id: u64) {
        let activity = RewardableActivity::new(Activity::GroupMemberCount(group_id));
        let _ = Self::insert(activity);
    }

    pub fn notify_active_user(principal: Principal) {
        let activity = RewardableActivity::new(Activity::UserActivity(principal));
        let _ = Self::insert(activity);
    }
}
