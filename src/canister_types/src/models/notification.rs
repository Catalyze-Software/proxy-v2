use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::impl_storable_for;

use super::{
    attendee::{InviteAttendeeResponse, JoinedAttendeeResponse},
    friend_request::FriendRequestResponse,
    member::{InviteMemberResponse, JoinedMemberResponse},
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::UserNotificationData,
};

impl_storable_for!(Notification);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub notification_type: NotificationType,
    // used on the frontend to determine if the notification is actionable
    // this value changes based on the action the user takes
    pub is_actionable: bool,
    pub processed_by: Option<Principal>,
    pub is_accepted: Option<bool>,
    // additional data for the notification that the frontend can utilize
    pub metadata: Option<String>,
    pub sender: Principal,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Notification {
    pub fn new(notification_type: NotificationType, is_actionable: bool) -> Self {
        Self {
            notification_type,
            is_actionable,
            is_accepted: None,
            metadata: None,
            processed_by: None,
            sender: caller(),
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn mark_as_accepted(&mut self, is_accepted: bool, notification_type: NotificationType) {
        self.is_accepted = Some(is_accepted);
        self.is_actionable = false;
        self.processed_by = Some(caller());
        self.updated_at = time();
        self.notification_type = notification_type;
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
    Multisig(MultisigNotificationType),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionNotificationType {
    SingleTransaction(TransactionData),
    TransactionsComplete(TransactionCompleteData),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum MultisigNotificationType {
    WhitelistNotice((Principal, u64)),
    NewProposal((Principal, u64, u64)),
    ProposalAccept((Principal, u64, u64)),
    ProposalDecline((Principal, u64, u64)),
    ProposalStatusUpdate((Principal, u64, u64)),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum RelationNotificationType {
    FriendRequest(FriendRequestResponse),
    FriendRequestAccept(FriendRequestResponse),
    FriendRequestDecline(FriendRequestResponse),

    FriendRequestRemove(u64),   // friend_request_id
    FriendRemove(Principal),    // user principal
    BlockUser(Principal),       // user principal
    FriendRequestReminder(u64), // friend_request_id
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum GroupNotificationType {
    // user wants to join the group
    JoinGroupUserRequest(InviteMemberResponse),
    JoinGroupUserRequestAccept(InviteMemberResponse),
    JoinGroupUserRequestDecline(InviteMemberResponse),
    // group wants a user to join
    JoinGroupOwnerRequest(InviteMemberResponse),
    JoinGroupOwnerRequestAccept(InviteMemberResponse),
    JoinGroupOwnerRequestDecline(InviteMemberResponse),

    RoleAssignByOwner(JoinedMemberResponse),
    RemoveInviteByOwner(InviteMemberResponse),
    RemoveMemberByOwner(JoinedMemberResponse),

    UserLeaveGroup(u64),
    UserJoinGroup(u64),
    GroupReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventNotificationType {
    // user wants to join the event
    JoinEventUserRequest(InviteAttendeeResponse),
    JoinEventUserRequestAccept(InviteAttendeeResponse),
    JoinEventUserRequestDecline(InviteAttendeeResponse),

    // Event wants a user to join
    JoinEventOwnerRequest(InviteAttendeeResponse),
    JoinEventOwnerRequestAccept(InviteAttendeeResponse),
    JoinEventOwnerRequestDecline(InviteAttendeeResponse),

    RoleAssignByOwner(JoinedAttendeeResponse),
    RemoveInviteByOwner(InviteAttendeeResponse),
    RemoveAttendeeByOwner(JoinedAttendeeResponse),

    UserJoinEvent((u64, u64)),
    UserLeaveEvent((u64, u64)),
    EventReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NotificationResponse {
    pub id: Option<u64>,
    pub notification: Notification,
    pub user_data: Option<UserNotificationData>,
}

impl NotificationResponse {
    pub fn new(
        id: Option<u64>,
        notification: Notification,
        user_data: Option<UserNotificationData>,
    ) -> Self {
        Self {
            id,
            notification,
            user_data,
        }
    }
}
