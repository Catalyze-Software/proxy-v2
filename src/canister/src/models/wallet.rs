use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, CandidType, Deserialize)]
pub struct Wallet {
    pub provider: String,
    pub is_primary: bool,
}

#[derive(Clone, Debug, Serialize, CandidType, Deserialize)]
pub struct PostWallet {
    pub provider: String,
    pub principal: Principal,
}

#[derive(Clone, Debug, Serialize, CandidType, Deserialize)]
pub struct WalletResponse {
    pub provider: String,
    pub principal: Principal,
    pub is_primary: bool,
}

impl Default for Wallet {
    fn default() -> Self {
        Self {
            provider: Default::default(),
            is_primary: Default::default(),
        }
    }
}
