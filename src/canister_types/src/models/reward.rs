use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(RewardableActivity);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct RewardableActivity {
    id: u64,
    activity: String,
    timestamp: u64,
}

impl RewardableActivity {
    pub fn new(activity: Activity) -> Self {
        Self {
            id: activity.id(),
            activity: activity.as_string(),
            timestamp: time(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_activity(&self) -> Activity {
        Activity::from_string(&self.activity, self.id)
    }

    pub fn before(&self, days: u64) -> bool {
        self.timestamp < time() - days * 24 * 60 * 60
    }
}

#[derive(Clone, Debug, CandidType)]
pub enum Activity {
    GroupCount(u64),
    GroupActivity(u64),
    EventAttendance(u64),
}

impl Activity {
    pub fn as_string(&self) -> String {
        match self {
            Activity::GroupCount(_) => "group_count".to_string(),
            Activity::GroupActivity(_) => "group_activity".to_string(),
            Activity::EventAttendance(_) => "event_attendance".to_string(),
        }
    }

    pub fn from_string(s: &str, id: u64) -> Self {
        match s {
            "group_count" => Activity::GroupCount(id),
            "group_activity" => Activity::GroupActivity(id),
            "event_attendance" => Activity::EventAttendance(id),
            _ => panic!("Unknown activity: {}", s),
        }
    }

    pub fn id(&self) -> u64 {
        match self {
            Activity::GroupCount(v) => *v,
            Activity::GroupActivity(v) => *v,
            Activity::EventAttendance(v) => *v,
        }
    }
}

#[derive(Deserialize, CandidType, Clone)]
pub struct RewardData {
    // group or event owner
    pub owner: Principal,
    // group or event id
    pub id: u64,
    // count, activity score, or attendance count
    pub count: u64,
}

impl RewardData {
    pub fn new(owner: Principal, id: u64, count: u64) -> Self {
        Self { owner, id, count }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct RewardDataPackage {
    pub group_member_counts: Vec<RewardData>,
    pub group_activity_counts: Vec<RewardData>,
    pub event_attendee_counts: Vec<RewardData>,
}
