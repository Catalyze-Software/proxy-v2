use candid::Principal;
use catalyze_shared::CanisterResult;
use ic_cdk::{query, update};

use crate::{
    helpers::guards::is_developer,
    storage::{CellStorage, HistoryCanisterStorage, HistoryPointStorage},
};

#[query]
fn get_history_point() -> CanisterResult<u64> {
    HistoryPointStorage::get()
}

#[update(guard = "is_developer")]
fn _dev_set_history_canister(principal: Principal) -> CanisterResult<Principal> {
    HistoryCanisterStorage::set(principal)
}

#[query(guard = "is_developer")]
fn _dev_get_history_canister() -> CanisterResult<Principal> {
    HistoryCanisterStorage::get()
}
