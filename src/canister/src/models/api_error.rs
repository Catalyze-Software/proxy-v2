use std::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::validation::ValidationResponse;

#[derive(Clone, CandidType, Debug, Serialize, Deserialize)]
pub struct ApiError {
    tag: Option<String>,
    message: Option<String>,
    method_name: Option<String>,
    error_type: ApiErrorType,
    info: Option<Vec<String>>,
}

impl ApiError {
    pub fn new(error_type: ApiErrorType, message: Option<String>) -> Self {
        ApiError {
            tag: None,
            message,
            method_name: None,
            error_type,
            info: None,
        }
    }

    pub fn validation_response(validation_response: Vec<ValidationResponse>) -> Self {
        ApiError {
            tag: None,
            message: Some("Validation error".to_string()),
            method_name: None,
            error_type: ApiErrorType::ValidationError(validation_response),
            info: None,
        }
    }

    pub fn serialize(message: String) -> Self {
        ApiError {
            tag: None,
            message: Some(message),
            method_name: None,
            error_type: ApiErrorType::SerializeError,
            info: None,
        }
    }

    pub fn deserialize(message: String) -> Self {
        ApiError {
            tag: None,
            message: Some(message),
            method_name: None,
            error_type: ApiErrorType::DeserializeError,
            info: None,
        }
    }

    pub fn unexpected(message: String) -> Self {
        ApiError {
            tag: None,
            message: Some(message),
            method_name: None,
            error_type: ApiErrorType::Unexpected,
            info: None,
        }
    }

    pub fn not_implemented() -> Self {
        ApiError {
            tag: None,
            message: Some("Not implemented".to_string()),
            method_name: None,
            error_type: ApiErrorType::NotImplemented,
            info: None,
        }
    }

    pub fn unauthorized() -> Self {
        ApiError {
            tag: None,
            message: Some("Unauthorized".to_string()),
            method_name: None,
            error_type: ApiErrorType::Unauthorized,
            info: None,
        }
    }

    pub fn not_found() -> Self {
        ApiError {
            tag: None,
            message: Some("Not found".to_string()),
            method_name: None,
            error_type: ApiErrorType::NotFound,
            info: None,
        }
    }

    pub fn bad_request() -> Self {
        ApiError {
            tag: None,
            message: Some("Bad request".to_string()),
            method_name: None,
            error_type: ApiErrorType::BadRequest,
            info: None,
        }
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn add_info(mut self, info: Vec<String>) -> Self {
        self.info = Some(info);
        self
    }

    pub fn add_method_name(mut self, method_name: String) -> Self {
        self.method_name = Some(method_name);
        self
    }
}

#[derive(Clone, CandidType, Debug, Deserialize, Serialize)]
pub enum ApiErrorType {
    NotImplemented,
    Unexpected,
    Unauthorized,
    NotFound,
    BadRequest,
    ValidationError(Vec<ValidationResponse>),
    SerializeError,
    DeserializeError,
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiErrorType::*;
        match self {
            NotImplemented => write!(f, "NotImplemented"),
            Unexpected => write!(f, "Unexpected"),
            Unauthorized => write!(f, "Unauthorized"),
            NotFound => write!(f, "NotFound"),
            BadRequest => write!(f, "BadRequest"),
            ValidationError(_) => write!(f, "ValidationError"),
            SerializeError => write!(f, "SerializeError"),
            DeserializeError => write!(f, "DeserializeError"),
        }
    }
}
