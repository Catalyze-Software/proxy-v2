use candid::Principal;
use catalyze_shared::CanisterResult;
use ic_cdk::{query, update};

use crate::{
    helpers::guards::is_developer,
    storage::{
        reward_canister_storage::RewardCanisterStorage, CellStorage, HistoryCanisterStorage,
        ProfileCanisterStorage, ReportCanisterStorage,
    },
};

#[query(guard = "is_developer")]
fn _dev_get_history_canister() -> CanisterResult<Principal> {
    HistoryCanisterStorage::get()
}

#[query(guard = "is_developer")]
fn _dev_get_reward_canister() -> CanisterResult<Principal> {
    RewardCanisterStorage::get()
}

#[query(guard = "is_developer")]
fn _dev_get_profile_canister() -> CanisterResult<Principal> {
    ProfileCanisterStorage::get()
}

#[query(guard = "is_developer")]
fn _dev_get_report_canister() -> CanisterResult<Principal> {
    ReportCanisterStorage::get()
}

#[update(guard = "is_developer")]
fn _dev_set_history_canister(principal: Principal) -> CanisterResult<Principal> {
    HistoryCanisterStorage::set(principal)
}

#[update(guard = "is_developer")]
fn _dev_set_reward_canister(principal: Principal) -> CanisterResult<Principal> {
    RewardCanisterStorage::set(principal)
}

#[update(guard = "is_developer")]
fn _dev_set_profile_canister(principal: Principal) -> CanisterResult<Principal> {
    ProfileCanisterStorage::set(principal)
}

#[update(guard = "is_developer")]
fn _dev_set_report_canister(principal: Principal) -> CanisterResult<Principal> {
    ReportCanisterStorage::set(principal)
}
