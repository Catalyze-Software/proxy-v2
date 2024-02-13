use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfilePrivacy {
    Public,
    Private,
}

impl Default for ProfilePrivacy {
    fn default() -> Self {
        ProfilePrivacy::Private
    }
}
