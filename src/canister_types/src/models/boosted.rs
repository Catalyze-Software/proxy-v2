use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::impl_storable_for;

impl_storable_for!(Boosted);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Boosted {
    pub subject: Subject,
    pub seconds: u64,
    pub owner: Principal,
    pub blockheight: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Boosted {
    pub fn new(subject: Subject, seconds: u64, owner: Principal, blockheight: u64) -> Self {
        Self {
            subject,
            seconds,
            created_at: time(),
            updated_at: time(),
            owner,
            blockheight,
        }
    }

    pub fn update(&mut self, seconds: u64) {
        self.seconds = seconds;
        self.updated_at = time();
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Group(u64),
    Event(u64),
}
