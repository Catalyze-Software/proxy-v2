use candid::{CandidType, Deserialize};

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub struct Invite {
    pub updated_at: u64,
    pub invite_type: InviteType,
}



#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum InviteType {
    OwnerRequest,
    #[default]
    UserRequest,
}


