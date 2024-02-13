use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronId {
    pub id: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct ListNeurons {
    pub of_principal: Option<Principal>,
    pub limit: u32,
    pub start_page_at: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct ListNeuronsResponse {
    pub neurons: Vec<Neuron>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronPermission {
    pub principal: Option<Principal>,
    pub permission_type: Vec<i32>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub permissions: Vec<NeuronPermission>,
    pub maturity_e8s_equivalent: u64,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub source_nns_neuron_id: Option<u64>,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub dissolve_state: Option<DissolveState>,
    pub voting_power_percentage_multiplier: u64,
    pub vesting_period_seconds: Option<u64>,
    pub disburse_maturity_in_progress: Vec<DisburseMaturityInProgress>,
    pub followees: Vec<(u64, Followees)>,
    pub neuron_fees_e8s: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DisburseMaturityInProgress {
    pub timestamp_of_disbursement_seconds: u64,
    pub amount_e8s: u64,
    pub account_to_disburse_to: Option<Account>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Account {
    pub owner: Option<candid::Principal>,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Subaccount {
    pub subaccount: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}
