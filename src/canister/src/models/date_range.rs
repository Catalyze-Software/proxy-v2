use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DateRange {
    start_date: u64,
    end_date: u64,
}

impl DateRange {
    pub fn new(start_date: u64, end_date: u64) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    pub fn start_date(&self) -> u64 {
        self.start_date
    }

    pub fn end_date(&self) -> u64 {
        self.end_date
    }
}
