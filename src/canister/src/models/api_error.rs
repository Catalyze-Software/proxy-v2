use std::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_json;

use super::validation::ValidationResponse;

#[derive(Clone, CandidType, Debug, Deserialize, Serialize)]
pub enum ApiErrorType {
    Unexpected,
    Unauthorized,
    NotFound,
    BadRequest,
    ValidationError,
    SerializeError,
    DeserializeError,
    UpdateRequired,
    CanisterAtCapacity,
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiErrorType::*;
        match self {
            Unexpected => write!(f, "Unexpected"),
            Unauthorized => write!(f, "Unauthorized"),
            NotFound => write!(f, "NotFound"),
            BadRequest => write!(f, "BadRequest"),
            ValidationError => write!(f, "ValidationError"),
            SerializeError => write!(f, "SerializeError"),
            DeserializeError => write!(f, "DeserializeError"),
            UpdateRequired => write!(f, "UpdateRequired"),
            CanisterAtCapacity => write!(f, "CanisterAtCapacity"),
        }
    }
}

#[derive(Clone, CandidType, Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub tag: String,
    pub message: String,
    pub location: String,
    pub inputs: Option<Vec<String>>,
}

#[derive(Clone, CandidType, Debug, Serialize, Deserialize)]
pub enum ApiError {
    NotImplemented,
    Unexpected(ErrorMessage),
    Unauthorized(ErrorMessage),
    NotFound(ErrorMessage),
    BadRequest(ErrorMessage),
    ValidationError(Vec<ValidationResponse>),
    SerializeError(ErrorMessage),
    DeserializeError(ErrorMessage),
    // UpdateRequired(UpdateMessage),
    CanisterAtCapacity(ErrorMessage),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiError::*;
        match self {
            NotImplemented => write!(f, "Generic"),
            Unexpected(value) => write!(f, "Unexpected - {:?}", serde_json::to_string(value)),
            Unauthorized(value) => write!(f, "Unauthorized - {:?}", serde_json::to_string(value)),
            NotFound(value) => write!(f, "NotFound - {:?}", serde_json::to_string(value)),
            BadRequest(value) => write!(f, "BadRequest - {:?}", serde_json::to_string(value)),
            ValidationError(value) => {
                write!(f, "ValidationError - {:?}", serde_json::to_string(value))
            }
            SerializeError(value) => {
                write!(f, "SerializeError - {:?}", serde_json::to_string(value))
            }
            DeserializeError(value) => {
                write!(f, "DeserializeError - {:?}", serde_json::to_string(value))
            }
            CanisterAtCapacity(value) => {
                write!(f, "CanisterAtCapacity - {:?}", serde_json::to_string(value))
            }
        }
    }
}
