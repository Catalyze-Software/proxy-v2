use candid::Principal;
use catalyze_shared::{
    api_error::ApiError, application_role::ApplicationRole, CanisterResult, StorageClient,
};
use ic_cdk::caller;

use crate::storage::profiles;

/// Checks if the caller has a profile and is not blocked or banned on the application level
/// # Returns
/// * `()` if the caller has a profile and is not blocked or banned
/// # Errors
/// * `ApiError` if the caller has no profile or is blocked or banned
pub async fn has_access() -> CanisterResult<()> {
    let (_, profile) = profiles().get(caller()).await?;

    if ![ApplicationRole::Blocked, ApplicationRole::Banned].contains(&profile.application_role) {
        return Ok(());
    }

    Err(ApiError::unauthorized().add_message("Blocked or banned"))
}

/// Checks if the caller is the monitor principal
pub fn is_monitor() -> Result<(), String> {
    // monitor principal
    let monitor_principal =
        Principal::from_text("6or45-oyaaa-aaaap-absua-cai").expect("Invalid principal");
    if caller() == monitor_principal {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

pub fn is_prod_developer() -> Result<(), String> {
    let developers = [
        // production
        "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe",
    ];

    if developers.contains(&caller().to_text().as_str()) {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

// Check if the caller is the Catalyze developer principal
pub fn is_developer() -> Result<(), String> {
    let developers = [
        // production
        "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe",
        // staging
        "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae",
        // Olek
        "bgykr-qmmrw-bynrn-ffwva-j6th7-juxki-het4d-5sac4-7v4t2-re73t-bqe",
        // Monitor
        "6or45-oyaaa-aaaap-absua-cai",
    ];

    if developers.contains(&caller().to_text().as_str()) {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

// TODO: add guards for group role based access
// https://forum.dfinity.org/t/rust-guard-access-arguments/22229?u=rmcs
// https://docs.rs/ic-cdk/latest/ic_cdk/api/call/fn.arg_data.html
