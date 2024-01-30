use candid::{CandidType, Decode, Encode, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(FriendRequest);

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequest {
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequestResponse {
    pub id: u64,
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}
