use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[derive(Default)]
pub enum InviteType {
    OwnerRequest,
    #[default]
    UserRequest,
}


