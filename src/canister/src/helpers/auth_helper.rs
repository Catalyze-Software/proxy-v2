use candid::Principal;
use ic_cdk::caller;

use crate::{models::application_role::ApplicationRole, stores::profile_store::Profiles};

/// Checks if the caller is anonymous
pub fn is_not_anonymous() -> Result<(), String> {
    match caller() == Principal::anonymous() {
        true => Err("Unauthorized".to_string()),
        false => Ok(()),
    }
}

/// Checks if the caller is anonymous, has a profile and is not blocked or banned on the application level
pub fn has_access() -> Result<(), String> {
    // Check if the caller is anonymous
    if let Err(err) = is_not_anonymous() {
        return Err(err);
    }

    // Check if the caller has a profile
    match Profiles::get(&caller()) {
        Some(profile) => {
            // Check if the caller is blocked or banned on the application level
            if vec![ApplicationRole::Blocked, ApplicationRole::Banned]
                .contains(&profile.application_role)
            {
                Err("Blocked or banned profile".to_string())
            } else {
                Ok(())
            }
        }
        None => Err("Profile not found".to_string()),
    }
}
