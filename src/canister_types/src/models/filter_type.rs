use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum FilterType<T: Default> {
    And(T),
    Or(T),
}

impl<T: Default> FilterType<T> {
    pub fn inner(&self) -> &T {
        match self {
            FilterType::And(inner) => inner,
            FilterType::Or(inner) => inner,
        }
    }
}

impl<T: Default> Default for FilterType<T> {
    fn default() -> Self {
        FilterType::And(Default::default())
    }
}
