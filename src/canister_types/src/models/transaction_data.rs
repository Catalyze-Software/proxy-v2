use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::transfer::Memo};
use serde::{Deserialize, Serialize};

// These structs is used on the transaction_handler cansister
// changes to these structs should be reflected on the transaction_handler canister
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransactionData {
    pub sender: Principal,
    pub receiver: Principal,
    pub canister: Principal,
    pub memo: Option<Memo>,
    pub fee: Nat,
    pub block_height: Nat,
    pub amount: Nat,
    pub metadata: Vec<(String, MetadataValue)>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransactionCompleteData {
    pub sender: Principal,
    pub receiver_count: u64,
    pub total_amount_distributed: Nat,
    pub canister: Principal,
    pub metadata: Vec<(String, MetadataValue)>,
}
