use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(HistoryEvent);

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct HistoryEvent {
    pub history_point: u64,
    pub payload: HistoryEventPayload,
}

impl HistoryEvent {
    pub fn new(history_point: u64, payload: HistoryEventPayload) -> Self {
        Self {
            history_point,
            payload,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum HistoryEventPayload {
    GroupRoleChanged(GroupRoleChanged),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct GroupRoleChanged {
    pub group_id: u64,
    pub principal: Principal,
    pub username: String,
    pub roles: Vec<String>,
    pub kind: GroupRoleChangeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum GroupRoleChangeKind {
    Add,
    Remove,
    Replace,
}
