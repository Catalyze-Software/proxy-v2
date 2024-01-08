use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Permission {
    pub name: String,
    pub protected: bool,
    pub actions: PermissionActions,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PostPermission {
    pub name: String,
    pub actions: PermissionActions,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PermissionActions {
    pub write: bool,
    pub read: bool,
    pub edit: bool,
    pub delete: bool,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub enum PermissionActionType {
    Write,
    Read,
    Edit,
    Delete,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum PermissionType {
    // Canister
    Task(Option<PermissionActions>),
    Event(Option<PermissionActions>),
    Group(Option<PermissionActions>),
    Member(Option<PermissionActions>),
    Invite(Option<PermissionActions>),
    Attendee(Option<PermissionActions>),

    // Frontend chat
    ChatNotification(Option<PermissionActions>),
    ChatMessage(Option<PermissionActions>),
    ChatChannel(Option<PermissionActions>),
    ChatEmbed(Option<PermissionActions>),
    ChatFiles(Option<PermissionActions>),
    ChatMention(Option<PermissionActions>),
    ChatPolls(Option<PermissionActions>),
}

impl fmt::Display for PermissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PermissionType::Task(_) => write!(f, "task"),
            PermissionType::Event(_) => write!(f, "event"),
            PermissionType::Group(_) => write!(f, "group"),
            PermissionType::Member(_) => write!(f, "member"),
            PermissionType::Invite(_) => write!(f, "invite"),
            PermissionType::Attendee(_) => write!(f, "attendee"),
            PermissionType::ChatNotification(_) => write!(f, "chat_notification"),
            PermissionType::ChatMessage(_) => write!(f, "chat_message"),
            PermissionType::ChatChannel(_) => write!(f, "chat_channel"),
            PermissionType::ChatEmbed(_) => write!(f, "chat_embed"),
            PermissionType::ChatFiles(_) => write!(f, "chat_files"),
            PermissionType::ChatMention(_) => write!(f, "chat_mention"),
            PermissionType::ChatPolls(_) => write!(f, "chat_polls"),
        }
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Role {
    pub name: String,
    pub protected: bool,
    pub permissions: Vec<Permission>,
    pub color: String,
    pub index: Option<u64>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PostRole {
    pub name: String,
    pub permissions: Vec<PostPermission>,
    pub color: String,
    pub index: Option<u64>,
}
