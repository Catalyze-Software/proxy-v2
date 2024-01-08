use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: u64,
    pub end_date: u64,
}
