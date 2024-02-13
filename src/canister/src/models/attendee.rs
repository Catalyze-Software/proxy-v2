use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use serde::Serialize;

use crate::impl_storable_for;

use super::invite::InviteType;

impl_storable_for!(Attendee);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Attendee {
    pub principal: Principal,
    pub joined: HashMap<u64, Join>,
    pub invites: HashMap<u64, Invite>,
}

impl Attendee {
    pub fn default() -> Self {
        Self {
            principal: Principal::anonymous(),
            joined: Default::default(),
            invites: Default::default(),
        }
    }

    pub fn new(principal: Principal, profile_identifier: Principal) -> Self {
        Self {
            principal,
            joined: Default::default(),
            invites: Default::default(),
        }
    }

    pub fn add_joined(&mut self, event_id: u64, group_id: u64) {
        self.joined.insert(
            event_id,
            Join {
                group_id,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn get_joined(&self) -> Vec<(u64, Join)> {
        self.joined.iter().map(|(k, v)| (*k, v.clone())).collect()
    }

    pub fn remove_joined(&mut self, group_id: u64) {
        self.joined.remove(&group_id);
    }

    pub fn add_invite(&mut self, event_id: u64, group_id: u64, invite_type: InviteType) {
        self.invites.insert(
            event_id,
            Invite {
                group_id,
                invite_type,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn get_invite(&self, event_id: u64) -> Option<Invite> {
        self.invites.get(&event_id).cloned()
    }

    pub fn remove_invite(&mut self, event_id: u64) {
        self.invites.remove(&event_id);
    }

    pub fn is_event_invited(&self, event_id: &u64) -> bool {
        self.invites.contains_key(&event_id)
    }

    pub fn is_event_joined(&self, event_id: &u64) -> bool {
        self.joined.contains_key(event_id)
    }

    pub fn has_pending_join_request(&self, event_id: u64) -> bool {
        if let Some(invite) = self.invites.get(&event_id) {
            return invite.invite_type == InviteType::UserRequest;
        }
        false
    }

    pub fn has_pending_invite(&self, event_id: u64) -> bool {
        if let Some(invite) = self.invites.get(&event_id) {
            return invite.invite_type == InviteType::OwnerRequest;
        }
        false
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Join {
    pub group_id: u64,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Invite {
    pub group_id: u64,
    pub invite_type: InviteType,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct JoinedAttendeeResponse {
    pub event_id: u64,
    pub group_id: u64,
    pub principal: Principal,
}

impl JoinedAttendeeResponse {
    pub fn new(event_id: u64, group_id: u64, principal: Principal) -> Self {
        Self {
            event_id,
            group_id,
            principal,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct InviteAttendeeResponse {
    pub event_id: u64,
    pub group_id: u64,
    pub principal: Principal,
    pub invite_type: InviteType,
}

impl InviteAttendeeResponse {
    pub fn new(
        event_id: u64,
        group_id: u64,
        principal: Principal,
        invite_type: InviteType,
    ) -> Self {
        Self {
            event_id,
            group_id,
            principal,
            invite_type,
        }
    }
}
