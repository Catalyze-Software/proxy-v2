use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subject {
    None,
    Group(u64),
    Event(u64),
    Profile(Principal),
    Member(Principal),
    Attendee(Principal),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubjectType {
    None,
    Group,
    Event,
    Profile,
    Member,
    Attendee,
}

impl Default for Subject {
    fn default() -> Self {
        Subject::None
    }
}
