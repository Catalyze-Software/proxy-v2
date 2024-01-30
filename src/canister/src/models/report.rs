use candid::{CandidType, Deserialize, Principal};

use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{date_range::DateRange, sort_direction::SortDirection},
};

impl_storable_for!(Report);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Report {
    pub reported_by: Principal,
    pub subject: Principal,
    pub group_identifier: Principal,
    pub message: String,
    pub created_on: u64,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            reported_by: Principal::anonymous(),
            subject: Principal::anonymous(),
            group_identifier: Principal::anonymous(),
            message: Default::default(),
            created_on: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PostReport {
    pub subject: Principal,
    pub message: String,
    pub group_identifier: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ReportResponse {
    pub identifier: Principal,
    pub reported_by: Principal,
    pub group_identifier: Principal,
    pub subject: Principal,
    pub subject_kind: String,
    pub message: String,
    pub created_on: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ReportSort {
    Id(SortDirection),
    Kind(SortDirection),
    CreatedOn(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ReportFilter {
    Kind(String),
    CreatedOn(DateRange),
    ReportedBy(Principal),
}
