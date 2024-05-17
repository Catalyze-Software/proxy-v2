use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProfilePrivacy {
    Public,
    #[default]
    Private,
}
