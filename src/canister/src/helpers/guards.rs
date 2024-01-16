use candid::Principal;
use ic_cdk::caller;

use crate::{
    models::application_role::ApplicationRole,
    storage::storage_api::{profiles, StorageMethods},
};

/// Checks if the caller is an anonymous principal
/// # Returns
/// * `()` if the caller is not anonymous
/// # Errors
/// * `String` if the caller is anonymous
/// # Note
/// `Result<(), String>` type is required because of the usage as a guard in the `candid` attribute macro
pub fn is_not_anonymous() -> Result<(), String> {
    match caller() == Principal::anonymous() {
        true => Err("Unauthorized".to_string()),
        false => Ok(()),
    }
}

/// Checks if the caller is anonymous, has a profile and is not blocked or banned on the application level
/// # Returns
/// * `()` if the caller is not anonymous, has a profile and is not blocked or banned
/// # Errors
/// * `String` if the caller is anonymous, has no profile or is blocked or banned
/// # Note
/// `Result<(), String>` type is required because of the usage as a guard in the `candid` attribute macro
pub fn has_access() -> Result<(), String> {
    // Check if the caller is anonymous
    if let Err(err) = is_not_anonymous() {
        return Err(err);
    }

    // Get the caller's profile
    let profile = profiles().get(caller())?;

    // Check if the caller is blocked or banned on the application level
    if vec![ApplicationRole::Blocked, ApplicationRole::Banned].contains(&profile.application_role) {
        Err("Blocked or banned profile".to_string())
    } else {
        Ok(())
    }
}
// TODO: add guards for group role based access
// https://forum.dfinity.org/t/rust-guard-access-arguments/22229?u=rmcs
// https://docs.rs/ic-cdk/latest/ic_cdk/api/call/fn.arg_data.html
