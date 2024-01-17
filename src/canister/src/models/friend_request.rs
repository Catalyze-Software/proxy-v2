use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequest {
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}

impl Storable for FriendRequest {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequestResponse {
    pub id: u64,
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}
