use candid::{decode_one, encode_one, CandidType};
use serde::{Deserialize, Serialize};

use super::{api_error::ApiError, notification::Notification};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum WebsocketMessage {
    UnreadCount(u64),
    Notification(Notification),
    SilentNotification(Notification),
    Error(ApiError),
}

impl WebsocketMessage {
    pub fn serialize(&self) -> Vec<u8> {
        match encode_one(&self) {
            Ok(value) => value,
            Err(_) => {
                // WebsocketMessage::Error(ApiError::deserialize().add_info("Serialization error"))
                vec![]
            }
        }
    }

    pub fn deserialize(data: &Vec<u8>) -> Self {
        match decode_one(&data) {
            Ok(value) => value,
            Err(_) => {
                WebsocketMessage::Error(ApiError::deserialize().add_info("Deserialization error"))
            }
        }
    }
}
