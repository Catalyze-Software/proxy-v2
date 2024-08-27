use candid::Principal;
use catalyze_shared::{CanisterResult, CellStorage};
use ic_cdk::{query, update};

use crate::{
    helpers::guards::is_developer,
    storage::{history_canister, profile_canister, report_canister},
};

#[query(guard = "is_developer")]
fn _dev_get_history_canister() -> CanisterResult<Principal> {
    history_canister().get()
}

#[query(guard = "is_developer")]
fn _dev_get_profile_canister() -> CanisterResult<Principal> {
    profile_canister().get()
}

#[query(guard = "is_developer")]
fn _dev_get_report_canister() -> CanisterResult<Principal> {
    report_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_history_canister(principal: Principal) -> CanisterResult<Principal> {
    history_canister().set(principal)
}

#[update(guard = "is_developer")]
fn _dev_set_profile_canister(principal: Principal) -> CanisterResult<Principal> {
    profile_canister().set(principal)
}

#[update(guard = "is_developer")]
fn _dev_set_report_canister(principal: Principal) -> CanisterResult<Principal> {
    report_canister().set(principal)
}
