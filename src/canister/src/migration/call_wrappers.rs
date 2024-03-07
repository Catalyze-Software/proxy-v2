use candid::Principal;
use ic_cdk::trap;

use super::old_models::profile_models::Profile as OldProfile;

// child profiles
#[ic_cdk::update]
async fn read_profiles_entries(env: String) -> Vec<(String, OldProfile)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_PROFILES,
        "development" => super::principals::DEVELOPMENT_CHILD_PROFILES,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldProfile)>,)>(canister_id, "read_profiles_entries", ())
        .await
        .expect("Failed to call profile canister")
        .0
}
