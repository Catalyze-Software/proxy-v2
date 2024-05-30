use super::storage_api::REWARD_BUFFER;
use crate::logic::reward_buffer_logic::send_reward_data;
use canister_types::models::reward::{Activity, RewardableActivity};
use ic_cdk::{api::time, spawn};
use ic_cdk_timers::set_timer_interval;
use std::{cell::RefCell, time::Duration};

//  Reward canister principal
pub const REWARD_CANISTER_ID: &str = "zgfl7-pqaaa-aaaap-accpa-cai";

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

impl RewardBufferStore {
    fn new_index() -> u64 {
        REWARD_BUFFER.with(|tree| {
            tree.borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0)
        })
    }

    pub fn notify_group_count_changed(group_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity::new(Activity::GroupCount(group_id));
        REWARD_BUFFER.with(|tree| {
            tree.borrow_mut().insert(index, activity);
        });
    }

    pub fn notify_group_is_active(group_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity::new(Activity::GroupActivity(group_id));
        REWARD_BUFFER.with(|tree| {
            tree.borrow_mut().insert(index, activity);
        });
    }

    pub fn notify_event_attendance(event_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity::new(Activity::EventAttendance(event_id));
        REWARD_BUFFER.with(|tree| {
            tree.borrow_mut().insert(index, activity);
        });
    }

    pub fn get_all() -> Vec<(u64, RewardableActivity)> {
        REWARD_BUFFER.with(|tree| tree.borrow().iter().collect())
    }

    pub fn clear() {
        REWARD_BUFFER.with(|tree| tree.borrow_mut().clear_new());
    }
}
