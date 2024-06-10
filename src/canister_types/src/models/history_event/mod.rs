use candid::{CandidType, Decode, Encode};
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(HistoryEvent);

pub mod group_role_changed;
pub mod history_event_kind;

pub use group_role_changed::*;
pub use history_event_kind::*;

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct HistoryEvent {
    pub kind: String,
    pub data: Vec<u8>,
}

pub type HistoryEventEntry = (u64, HistoryEvent);
