use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use serde::Serialize;

use crate::impl_storable_for;

pub type GroupIdentifier = Principal;

impl_storable_for!(Member);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Member {
    principal: Principal,
    profile_identifier: Principal,
    joined: HashMap<GroupIdentifier, Join>,
    invites: HashMap<GroupIdentifier, Invite>,
}

impl Member {
    pub fn default() -> Self {
        Self {
            principal: Principal::anonymous(),
            profile_identifier: Principal::anonymous(),
            joined: Default::default(),
            invites: Default::default(),
        }
    }

    pub fn new(principal: Principal, profile_identifier: Principal) -> Self {
        Self {
            principal,
            profile_identifier,
            joined: Default::default(),
            invites: Default::default(),
        }
    }

    pub fn add_group(&mut self, group_identifier: Principal, roles: Vec<String>) {
        self.joined.insert(
            group_identifier,
            Join {
                roles,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn remove_group(&mut self, group_identifier: Principal) {
        self.joined.remove(&group_identifier);
    }

    pub fn add_invite(&mut self, group_identifier: Principal, invite_type: InviteType) {
        self.invites.insert(
            group_identifier,
            Invite {
                invite_type,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn remove_invite(&mut self, group_identifier: Principal) {
        self.invites.remove(&group_identifier);
    }

    pub fn has_group_role(&self, group_identifier: Principal, role: String) -> bool {
        if let Some(group) = self.joined.get(&group_identifier) {
            return group.roles.contains(&role);
        }
        false
    }

    pub fn is_group_owner(&self, group_identifier: Principal) -> bool {
        self.has_group_role(group_identifier, "owner".to_string())
    }

    pub fn is_group_admin(&self, group_identifier: Principal) -> bool {
        self.has_group_role(group_identifier, "admin".to_string())
    }

    pub fn is_group_moderator(&self, group_identifier: Principal) -> bool {
        self.has_group_role(group_identifier, "moderator".to_string())
    }

    pub fn is_group_member(&self, group_identifier: Principal) -> bool {
        self.has_group_role(group_identifier, "member".to_string())
    }

    pub fn is_group_invited(&self, group_identifier: Principal) -> bool {
        self.invites.contains_key(&group_identifier)
    }

    /// Check if the member has a pending join request for the group
    /// # Note
    /// Member is has requested to join the group
    pub fn has_pending_join_request(&self, group_identifier: Principal) -> bool {
        if let Some(invite) = self.invites.get(&group_identifier) {
            return invite.invite_type == InviteType::UserRequest;
        }
        false
    }

    /// Check if the member ha a pending owner request for the group
    /// # Note
    /// Member is invited by group
    pub fn has_pending_group_invite(&self, group_identifier: Principal) -> bool {
        if let Some(invite) = self.invites.get(&group_identifier) {
            return invite.invite_type == InviteType::OwnerRequest;
        }
        false
    }

    pub fn get_roles(&self, group_identifier: Principal) -> Vec<String> {
        if let Some(group) = self.joined.get(&group_identifier) {
            return group.roles.clone();
        }
        vec![]
    }
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Join {
    pub roles: Vec<String>,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Invite {
    pub invite_type: InviteType,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InviteType {
    OwnerRequest,
    UserRequest,
}

impl Default for InviteType {
    fn default() -> Self {
        InviteType::UserRequest
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct JoinedMemberResponse {
    pub group_identifier: Principal,
    pub member_identifier: Principal,
    pub principal: Principal,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct InviteMemberResponse {
    pub group_identifier: Principal,
    pub member_identifier: Principal,
    pub principal: Principal,
    pub invite: Invite,
}
