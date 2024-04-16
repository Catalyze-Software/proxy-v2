use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::impl_storable_for;

use super::{friend_request::FriendRequest, user_notifications::UserNotificationData};

impl_storable_for!(Notification);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub notification_type: NotificationType,
    // used on the frontend to determine if the notification is actionable
    // this value changes based on the action the user takes
    pub is_actionable: bool,
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
            sender: caller(),
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn mark_as_accepted(&mut self, is_accepted: bool, notification_type: NotificationType) {
        self.is_accepted = Some(is_accepted);
        self.is_actionable = false;
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
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum RelationNotificationType {
    FriendRequest(FriendRequest),
    FriendRequestAccept(u64),   // friend_request_id
    FriendRequestDecline(u64),  // friend_request_id
    FriendRequestRemove(u64),   // friend_request_id
    FriendRemove(Principal),    // user principal
    BlockUser(Principal),       // user principal
    FriendRequestReminder(u64), // friend_request_id
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum GroupNotificationType {
    // user wants to join the group
    JoinGroupUserRequest(u64),
    JoinGroupUserRequestAccept(u64),
    JoinGroupUserRequestDecline(u64),
    // group wants a user to join
    JoinGroupOwnerRequest(u64),
    JoinGroupOwnerRequestAccept(u64),
    JoinGroupOwnerRequestDecline(u64),
    UserJoinGroup(u64),
    UserLeaveGroup(u64),
    GroupReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventNotificationType {
    // user wants to join the event
    JoinEventUserRequest(u64),
    JoinEventUserRequestAccept(u64),
    JoinEventUserRequestDecline(u64),

    // Event wants a user to join
    JoinEventOwnerRequest(u64),
    JoinEventOwnerRequestAccept(u64),
    JoinEventOwnerRequestDecline(u64),
    UserJoinEvent(u64),
    UserLeaveEvent(u64),
    EventReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionNotificationType {
    SingleTransaction(u64),
    MultipleTransaction(Vec<u64>),
    Airdrop,
    MultisigTransaction(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NotificationResponse {
    pub id: u64,
    pub notification: Notification,
    pub user_data: Option<UserNotificationData>,
}

impl NotificationResponse {
    pub fn new(
        id: u64,
        notification: Notification,
        user_data: Option<UserNotificationData>,
    ) -> Self {
        Self {
            id,
            notification,
            user_data: None,
        }
    }
}
