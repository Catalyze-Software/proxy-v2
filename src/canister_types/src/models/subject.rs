use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subject {
    None,
    Group(u64),
    Event(u64),
    Task(u64),
    Profile(Principal),
    Member(Principal),
    Attendee(Principal),
}

impl Subject {
    pub fn get_type(&self) -> SubjectType {
        match self {
            Subject::None => SubjectType::None,
            Subject::Group(_) => SubjectType::Group,
            Subject::Event(_) => SubjectType::Event,
            Subject::Task(_) => SubjectType::Task,
            Subject::Profile(_) => SubjectType::Profile,
            Subject::Member(_) => SubjectType::Member,
            Subject::Attendee(_) => SubjectType::Attendee,
        }
    }

    pub fn get_id(&self) -> &u64 {
        match self {
            Subject::Group(id) => id,
            Subject::Event(id) => id,
            Subject::Task(id) => id,
            _ => &0,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubjectType {
    None,
    Group,
    Event,
    Task,
    Profile,
    Member,
    Attendee,
}

impl Default for Subject {
    fn default() -> Self {
        Subject::None
    }
}
