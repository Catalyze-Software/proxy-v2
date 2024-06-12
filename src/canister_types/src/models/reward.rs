use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(RewardableActivity);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct RewardableActivity {
    activity: Vec<u8>,
    timestamp: u64,
}

impl RewardableActivity {
    pub fn new(activity: Activity) -> Self {
        Self {
            activity: activity.encode(),
            timestamp: time(),
        }
    }

    pub fn get_activity(&self) -> Activity {
        Activity::decode(self.activity.clone())
    }

    pub fn before(&self, days: u64) -> bool {
        self.timestamp < time() - days * 24 * 60 * 60
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum Activity {
    GroupMemberCount(u64),
    UserActivity(Principal),
}

impl Activity {
    pub fn encode(&self) -> Vec<u8> {
        Encode!(self).unwrap()
    }

    pub fn decode(bytes: Vec<u8>) -> Self {
        Decode!(&bytes, Activity).unwrap()
    }

    pub fn as_string(&self) -> String {
        match self {
            Activity::GroupMemberCount(_) => "group_count".to_string(),
            Activity::UserActivity(_) => "user_activity".to_string(),
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
    pub user_activity: Vec<RewardData>,
}
