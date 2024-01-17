use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum FilterType {
    And,
    Or,
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::And
    }
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FilterType::*;
        match self {
            And => write!(f, "And"),
            Or => write!(f, "Or"),
        }
    }
}
