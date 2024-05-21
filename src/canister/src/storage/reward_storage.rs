use super::{storage_api::REWARD_BUFFER, EventStore};
use crate::logic::reward_buffer_logic::send_reward_activities;
use crate::storage::storage_api::StorageQueryable;
use canister_types::models::reward::RewardableActivity;
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

// Interval for sending reward activities to Reward Canister
const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

// thread local refcell for timer id
thread_local! {
   pub static REWARD_TIMER: std::cell::RefCell<Option<ic_cdk_timers::TimerId>> = std::cell::RefCell::new(None);
}

pub struct RewardTimerStore;

impl RewardTimerStore {
    pub fn start_reward_timer() {
        let id = set_timer_interval(INTERVAL, move || send_reward_activities());

        REWARD_TIMER.with(|t| *t.borrow_mut() = Some(id));
    }

    pub fn reward_timer_set() -> bool {
        REWARD_TIMER.with(|t| t.borrow().is_some())
    }
}

// Rewardable Activities
pub const GROUP_COUNT: &str = "group_count";
pub const GROUP_ACTIVITY: &str = "group_activity";
// note: event attendance is checked periodically and not stored in the reward buffer

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

    // fn notify_event_attendance() {
    //     EventStore::get_all().iter().for_each(|(id, event)| {
    //         if event.date.end_date < ic_cdk::api::time() {

    //         }
    //     });
    // }
}
