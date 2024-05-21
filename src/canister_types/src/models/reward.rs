use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(CandidType, Deserialize)]
pub struct RewardableActivity {
    pub timestamp: u64,
    // group or event id
    pub id: u64,
    pub activity: String,
}

impl Storable for RewardableActivity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode RewardableActivity"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, RewardableActivity).expect("Failed to decode RewardableActivity")
    }

    const BOUND: Bound = Bound::Unbounded;
}
