use candid::Principal;
use canister_types::models::boosted::Boosted;
use ic_cdk::{query, update};

use crate::{helpers::guards::has_access, E8S_PER_DAY_BOOST_COST};

/// Returns the boosted groups
/// # Returns
/// * `Vec<Boosted>` - the boosted groups
#[query]
fn get_boosted_groups() -> Vec<Boosted> {
    vec![]
}

/// Returns the boosted events
/// # Returns
/// * `Vec<Boosted>` - the events groups
#[query]
fn get_boosted_events() -> Vec<Boosted> {
    vec![]
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
async fn boost(identifier: Principal, blockheight: u64) -> Result<u64, String> {
    Err("Not implemented".to_string())
}

/// Returns the remaining boost time in seconds for a group or event
/// # Arguments
/// * `identifier` - the identifier of the group or event
/// # Returns
/// * `u64` - the remaining boost time in nanoseconds
#[query]
fn get_remaining_boost_time_in_seconds(identifier: Principal) -> u64 {
    0
}
