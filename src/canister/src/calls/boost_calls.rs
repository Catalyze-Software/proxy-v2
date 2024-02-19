use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    boosted::{Boosted, Subject},
    identifier::Identifier,
};
use ic_cdk::{query, update};

use crate::{helpers::guards::has_access, logic::boost_logic::BoostCalls, E8S_PER_DAY_BOOST_COST};

/// Returns the boosted groups
/// # Returns
/// * `Vec<Boosted>` - the boosted groups
#[query]
fn get_boosted_groups() -> Vec<(u64, Boosted)> {
    BoostCalls::get_multiple_boost_by_subject(Subject::Group(0))
}

/// Returns the boosted events
/// # Returns
/// * `Vec<Boosted>` - the events groups
#[query]
fn get_boosted_events() -> Vec<(u64, Boosted)> {
    BoostCalls::get_multiple_boost_by_subject(Subject::Event(0))
}

/// Returns the cost of boosting per day
/// # Returns
/// * `u64` - the cost of boosting per day
#[query]
fn get_e8s_per_day_boost_cost() -> u64 {
    E8S_PER_DAY_BOOST_COST
}

/// Boosts a group or event
/// # Arguments
/// * `identifier` - the identifier of the group or event
/// * `blockheight` - the blockheight of the ICP transaction
/// # Returns
/// * `u64` - the remaining boost time in seconds
/// # Errors
/// * `String` if something went wrong boosting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// The identifier is used to determine if the group or event should be boosted.
#[update(guard = "has_access")]
async fn boost(identifier: Principal, blockheight: u64) -> Result<u64, ApiError> {
    let _identifier = Identifier::from(identifier);
    match _identifier.kind().as_str() {
        "group" => BoostCalls::boost(Subject::Group(_identifier.id()), blockheight).await,
        "event" => BoostCalls::boost(Subject::Event(_identifier.id()), blockheight).await,
        _ => Err(ApiError::bad_request().add_message("Invalid identifier")),
    }
}

/// Returns the remaining boost time in seconds for a group or event
/// # Arguments
/// * `identifier` - the identifier of the group or event
/// # Returns
/// * `u64` - the remaining boost time in seconds
#[query]
fn get_remaining_boost_time_in_seconds(identifier: Principal) -> Result<u64, ApiError> {
    let _identifier = Identifier::from(identifier);

    let (id, _) = match _identifier.kind().as_str() {
        "group" => BoostCalls::get_boosted_by_subject(Subject::Group(_identifier.id())),
        "event" => BoostCalls::get_boosted_by_subject(Subject::Event(_identifier.id())),
        _ => return Err(ApiError::bad_request().add_message("Invalid identifier")),
    }?;

    BoostCalls::get_seconds_left_for_boosted(id)
}
