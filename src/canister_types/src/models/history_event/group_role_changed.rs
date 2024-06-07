use std::str::FromStr;

use candid::{CandidType, Decode, Encode, Principal};
use serde::Deserialize;

use super::{history_event_kind::HistoryEventKind, HistoryEvent};

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum GroupRoleChangeKind {
    Add,
    Remove,
    Replace,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct GroupRoleChanged {
    pub group_id: u64,
    pub principal: Principal,
    pub username: String,
    pub roles: Vec<String>,
    pub kind: GroupRoleChangeKind,
}

impl GroupRoleChanged {
    pub fn new(
        group_id: u64,
        principal: Principal,
        username: String,
        roles: Vec<String>,
        kind: GroupRoleChangeKind,
    ) -> Self {
        Self {
            group_id,
            principal,
            username,
            roles,
            kind,
        }
    }
}

impl TryFrom<GroupRoleChanged> for HistoryEvent {
    type Error = candid::Error;

    fn try_from(value: GroupRoleChanged) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: HistoryEventKind::GroupRoleChanged.into(),
            data: Encode!(&value)?,
        })
    }
}

impl TryFrom<HistoryEvent> for GroupRoleChanged {
    type Error = candid::Error;

    fn try_from(value: HistoryEvent) -> Result<Self, Self::Error> {
        match HistoryEventKind::from_str(&value.kind).map_err(candid::Error::msg)? {
            HistoryEventKind::GroupRoleChanged => Decode!(&value.data, GroupRoleChanged),
        }
    }
}
