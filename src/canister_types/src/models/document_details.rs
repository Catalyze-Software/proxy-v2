use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, CandidType, Deserialize)]
pub struct DocumentDetails {
    approved_version: u64,
    approved_date: u64,
}

impl DocumentDetails {
    pub fn new(approved_version: u64, approved_date: u64) -> Self {
        Self {
            approved_version,
            approved_date,
        }
    }

    pub fn approved_version(&self) -> u64 {
        self.approved_version
    }

    pub fn approved_date(&self) -> u64 {
        self.approved_date
    }
}
