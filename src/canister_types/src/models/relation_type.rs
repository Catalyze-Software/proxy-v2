use std::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationType {
    Friend,
    Blocked,
}

impl fmt::Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RelationType::*;
        match self {
            Friend => write!(f, "friend"),
            Blocked => write!(f, "blocked"),
        }
    }
}
