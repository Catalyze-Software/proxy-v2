use candid::Principal;
use catalyze_shared::{CanisterResult, CellStorage, GenericCellStorage};

use super::storage_api::{
    HISTORY_CANISTER, HISTORY_POINT, PROFILE_CANISTER, REPORT_CANISTER, REWARD_CANISTER,
    TOPIC_CANISTER,
};

pub fn reward_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("reward_canister", &REWARD_CANISTER)
}

pub fn history_point() -> impl CellStorage<u64> {
    GenericCellStorage::new("history_point", &HISTORY_POINT)
}

pub fn get_next_history_point() -> CanisterResult<u64> {
    let current = history_point().get().unwrap_or(1);
    let next = current + 1;
    history_point().set(next)?;
    Ok(next)
}

pub fn history_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("history_canister", &HISTORY_CANISTER)
}

pub fn profile_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("profile_canister", &PROFILE_CANISTER)
}

pub fn report_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("report_canister", &REPORT_CANISTER)
}

pub fn topic_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("topic_canister", &TOPIC_CANISTER)
}
