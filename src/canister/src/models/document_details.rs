use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, CandidType, Deserialize)]
pub struct DocumentDetails {
    pub approved_version: u64,
    pub approved_date: u64,
}
