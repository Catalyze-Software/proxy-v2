use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use super::storage::CanisterStorage;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Asset {
    None,
    CanisterStorage(CanisterStorage),
    Url(String),
}

impl Default for Asset {
    fn default() -> Self {
        Asset::None
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Asset::*;
        match self {
            None => write!(f, "None"),
            CanisterStorage(value) => {
                write!(f, "CanisterStorage - {:?}", serde_json::to_string(value))
            }
            Url(value) => write!(f, "NotFound - {:?}", value),
        }
    }
}
