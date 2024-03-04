use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::impl_storable_for;

use super::{
    attendee::InviteAttendeeResponse, friend_request::FriendRequestResponse,
    member::InviteMemberResponse,
};

impl_storable_for!(Notification);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub receivers: Vec<Principal>,
    pub notification_type: NotificationType,
    // used on the frontend to determine if the notification is actionable
    // this value changes based on the action the user takes
    pub is_actionable: bool,
    pub is_accepted: Option<bool>,
    // additional data for the notification that the frontend can utilize
    pub metadata: Option<String>,
    pub created_by: Principal,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Notification {
    pub fn new(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Self {
        Self {
            receivers,
            notification_type,
            is_actionable,
            is_accepted: None,
            metadata: None,
            created_by: caller(),
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn mark_as_accepted(&mut self, is_accepted: bool) {
        self.is_accepted = Some(is_accepted);
        self.is_actionable = false;
        self.updated_at = time();
    }

    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = Some(metadata);
        self.updated_at = time();
    }

    pub fn set_is_actionable(&mut self, is_actionable: bool) {
        self.is_actionable = is_actionable;
        self.updated_at = time();
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum NotificationType {
    Relation(RelationNotificationType),
    Group(GroupNotificationType),
    Event(EventNotificationType),
    Transaction(TransactionNotificationType),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum RelationNotificationType {
    FriendRequest(FriendRequestResponse),
    FriendRequestAccept(u64),  // friend_request_id
    FriendRequestDecline(u64), // friend_request_id
    FriendRemove(Principal),   // user principal
    BlockUser(Principal),      // user principal
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum GroupNotificationType {
    // user wants to join the group
    JoinGroupUserRequest(InviteMemberResponse),
    JoinGroupUserRequestAccept(u64),
    JoinGroupUserRequestDecline(u64),
    // group wants a user to join
    JoinGroupOwnerRequest(InviteMemberResponse),
    JoinGroupOwnerRequestAccept(u64),
    JoinGroupOwnerRequestDecline(u64),
    UserLeaveGroup(Principal),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventNotificationType {
    // user wants to join the event
    JoinEventUserRequest(InviteAttendeeResponse),
    JoinEventUserRequestAccept(u64),
    JoinEventUserRequestDecline(u64),

    // Event wants a user to join
    JoinEventOwnerRequest(InviteAttendeeResponse),
    JoinEventOwnerRequestAccept(u64),
    JoinEventOwnerRequestDecline(u64),
    UserLeaveEvent(Principal),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionNotificationType {
    SingleTransaction(u64),
    MultipleTransaction(Vec<u64>),
    Airdrop(),
}
