use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum InviteType {
    OwnerRequest,
    UserRequest,
}

impl Default for InviteType {
    fn default() -> Self {
        InviteType::UserRequest
    }
}
