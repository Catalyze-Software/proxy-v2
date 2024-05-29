use super::storage_api::REWARD_BUFFER;
use crate::logic::reward_buffer_logic::send_reward_data;
use canister_types::models::reward::RewardableActivity;
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
        let _ = set_timer_interval(INTERVAL, move || ic_cdk::spawn(send_reward_data()));

        let next_trigger = ic_cdk::api::time() + INTERVAL.as_nanos() as u64;

        REWARD_TIMER.with(|t| *t.borrow_mut() = Some(next_trigger));
    }

    pub fn next_trigger() -> Option<u64> {
        REWARD_TIMER.with(|t| *t.borrow())
    }

    pub fn set_next_trigger() {
        REWARD_TIMER.with(|t| {
            let next_trigger = ic_cdk::api::time() + INTERVAL.as_nanos() as u64;
            *t.borrow_mut() = Some(next_trigger);
        });
    }
}

// Rewardable Activities
pub const GROUP_COUNT: &str = "group_count";
pub const GROUP_ACTIVITY: &str = "group_activity";
pub const EVENT_ATTENDANCE: &str = "event_attendance";

pub struct RewardBufferStore;

impl RewardBufferStore {
    fn new_index() -> u64 {
        REWARD_BUFFER.with(|tree| {
            let index = tree
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);
            index
        })
    }

    pub fn notify_group_count_changed(group_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity {
            timestamp: ic_cdk::api::time(),
            id: group_id,
            activity: GROUP_COUNT.to_string(),
        };
        REWARD_BUFFER.with(|tree| {
            tree.borrow_mut().insert(index, activity);
        });
    }

    pub fn notify_group_is_active(group_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity {
            timestamp: ic_cdk::api::time(),
            id: group_id,
            activity: GROUP_ACTIVITY.to_string(),
        };
        REWARD_BUFFER.with(|tree| {
            tree.borrow_mut().insert(index, activity);
        });
    }

    pub fn notify_event_attendance(event_id: u64) {
        let index = Self::new_index();
        let activity = RewardableActivity {
            timestamp: ic_cdk::api::time(),
            id: event_id,
            activity: EVENT_ATTENDANCE.to_string(),
        };
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
