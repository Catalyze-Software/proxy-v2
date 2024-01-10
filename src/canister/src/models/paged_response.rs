use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub page: usize,
    pub limit: usize,
    pub total: usize,
    pub number_of_pages: usize,
    pub data: Vec<T>,
}
