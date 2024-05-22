use super::storage_api::REWARD_BUFFER;
use crate::{logic::reward_buffer_logic::send_reward_data, storage::storage_api::StorageQueryable};
use canister_types::models::reward::RewardableActivity;
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

//  Reward canister principal
pub const REWARD_CANISTER_ID: &str = "zgfl7-pqaaa-aaaap-accpa-cai";

// Interval for sending reward activities to Reward Canister
const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

// thread local refcell for timer id
thread_local! {
   pub static REWARD_TIMER: std::cell::RefCell<Option<ic_cdk_timers::TimerId>> = std::cell::RefCell::new(None);
}

pub struct RewardTimerStore;

impl RewardTimerStore {
    pub fn start_reward_timer() {
        let id = set_timer_interval(INTERVAL, move || ic_cdk::spawn(send_reward_data()));

        REWARD_TIMER.with(|t| *t.borrow_mut() = Some(id));
    }

    pub fn reward_timer_set() -> bool {
        REWARD_TIMER.with(|t| t.borrow().is_some())
    }
}

// Rewardable Activities
pub const GROUP_COUNT: &str = "group_count";
pub const GROUP_ACTIVITY: &str = "group_activity";
pub const EVENT_ATTENDANCE: &str = "event_attendance";

pub struct RewardStore;

impl RewardStore {
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

    fn notify_group_count_changed(group_id: u64) {
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

    // 30 day average 'activities' per day
    fn notify_group_is_active(group_id: u64) {
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

    fn notify_event_attendance(event_id: u64) {
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
}
