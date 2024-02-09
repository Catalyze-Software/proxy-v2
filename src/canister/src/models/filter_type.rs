use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum FilterType<T> {
    And(T),
    Or(T),
}

impl<T> Default for FilterType<T> {
    fn default() -> Self {
        FilterType::And
    }
}
