use candid::{decode_one, encode_one, CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use candid::{Decode, Encode};

use crate::impl_storable_for;

impl_storable_for!(Notification);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub created_by: Principal,
    pub created_at: u64,
    pub notification_data: Option<NotificationType>,
    pub metadata: String,
    pub is_read: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NotificationDataResponse {
    pub id: u64,
    pub created_by: Principal,
    pub created_at: u64,
    pub notification_data: Option<NotificationType>,
    pub metadata: String,
    pub is_read: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventNotificationData {
    pub group_identifier: Principal,
    pub event_identifier: Principal,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct InviteNotificationData {
    pub invite_type: InviteType,
    pub accepted: Option<bool>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum InviteType {
    Group(InviteTypeRequest),
    Event(InviteTypeRequest),
    Task(InviteTypeRequest),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum InviteTypeRequest {
    OwnerRequest(Principal),
    UserRequest(Principal),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FriendRequestNotificationData {
    pub friend_request_id: u64,
    pub from: Principal,
    pub to: Principal,
    pub accepted: Option<bool>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransactionNotificationData {
    pub from: Principal,
    pub to: Principal,
    pub canister_id: Principal,
    pub amount: u64,
    pub token: String,
    pub transaction_id: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct MultisigNotificationData {
    pub canister_id: Principal,
    pub action: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum NotificationType {
    None,
    // Event
    EventInvite(),
    Event(EventNotificationData),
    Invite(InviteNotificationData),
    FriendRequest(FriendRequestNotificationData),
    FriendRemove(Principal),
    Transaction(TransactionNotificationData),
    Multisig(MultisigNotificationData),
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct SendNotificationData {
    pub data: NotificationType,
    pub receivers: Vec<Principal>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum MessageType {
    UnreadCount(u64),
    Notification(NotificationDataResponse),
    SilentNotification(SilentNotificationDataResponse),
    SendNotification(SendNotificationData),
    Error(String),
}

impl MessageType {
    pub fn serialize(&self) -> Vec<u8> {
        match encode_one(&self) {
            Ok(value) => value,
            Err(_) => {
                vec![]
            }
        }
    }

    pub fn deserialize(data: &Vec<u8>) -> Self {
        match decode_one(&data) {
            Ok(value) => value,
            Err(_) => MessageType::Error("Deserialization error".to_string()),
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AppMessageReceive {
    pub text: String,
    pub timestamp: u64,
}
