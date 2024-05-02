use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::transfer::Memo;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransactionData {
    pub sender: Principal,
    pub receiver: Principal,
    pub canister: Principal,
    pub memo: Option<Memo>,
    pub fee: Nat,
    pub block_height: Nat,
    pub amount: Nat,
}
