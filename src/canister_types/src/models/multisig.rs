#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WhitelistNotice {
    pub whitelist: Vec<String>,
    pub wallet: Principal,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ProposalAccept {
    pub whitelist: Vec<String>,
    pub wallet: Principal,
}
