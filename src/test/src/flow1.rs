// Flow 1

use crate::{
    mock_ids::{canister_test_id, member_test_id},
    mock_models::{mock_post_group, mock_post_profile},
    ENV,
};
use candid::Principal;
use models::models::{
    api_error::ApiError,
    group::{GroupResponse, PostGroup},
    profile::{PostProfile, ProfileResponse},
};
use pocket_ic::update_candid_as;

#[test]
fn flow1() {
    let sender: Principal = member_test_id();

    /*
     * Add profile
     */
    let post_profile: PostProfile = mock_post_profile();
    // Deprecated
    let member_canister: Principal = canister_test_id();

    let profile_response: ProfileResponse =
        update_candid_as::<(PostProfile, Principal), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            sender,
            "add_profile",
            (post_profile, member_canister),
        )
        .expect("Failed to call add_profile from pocketIC")
        .0
        .expect("Failed to add profile");

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(profile_response.principal, sender);

    /*
     * Approve code of conduct
     */
    let version: u64 = 1;

    let code_of_conduct_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        sender,
        "approve_code_of_conduct",
        (version,),
    )
    .expect("Failed to call approve_code_of_conduct from pocketIC")
    .0
    .expect("Failed to approve code of conduct");

    assert_eq!(code_of_conduct_approved, true);

    /*
     * Approve privacy policy
     */
    let version: u64 = 1;

    let privacy_policy_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        sender,
        "approve_privacy_policy",
        (version,),
    )
    .expect("Failed to call approve_privacy_policy from pocketIC")
    .0
    .expect("Failed to approve privacy policy");

    assert_eq!(privacy_policy_approved, true);

    /*
     * Approve terms of service
     */
    let version: u64 = 1;

    let terms_of_service_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        sender,
        "approve_terms_of_service",
        (version,),
    )
    .expect("Failed to call approve_terms_of_service from pocketIC")
    .0
    .expect("Failed to approve terms of service");

    assert_eq!(terms_of_service_approved, true);

    /*
     * Add group
     */
    let post_group: PostGroup = mock_post_group();
    let account_identifier: Option<String> = None;

    let _group_response: GroupResponse =
        update_candid_as::<(PostGroup, Option<String>), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            sender,
            "add_group",
            (post_group, account_identifier),
        )
        .expect("Failed to call add_group from pocketIC")
        .0
        .expect("Failed to add group");
}
