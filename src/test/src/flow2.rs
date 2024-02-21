// Flow 2

use candid::Principal;
use models::models::{
    profile::{PostProfile, ProfileResponse, UpdateProfile},
    wallet::PostWallet,
};

use crate::{
    calls::profile_calls::{
        add_profile, add_starred, add_wallet_to_profile, edit_profile, remove_starred,
        remove_wallet_from_profile,
    },
    mocks::{
        models::{mock_post_profile, mock_post_wallet, mock_update_profile},
        principals::{canister_test_id, canister_test_id2, member_test_id, member_test_id2},
    },
    SENDER,
};

pub fn flow2() {
    // Set sender principal
    SENDER.with(|s| *s.borrow_mut() = member_test_id2());
    // Deprecated
    let member_canister: Principal = canister_test_id2();

    /*
     * Add profile
     */
    let post_profile: PostProfile = mock_post_profile();

    let profile_response: ProfileResponse = add_profile(post_profile, member_canister);

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(profile_response.principal, SENDER.with(|s| *s.borrow()));

    /*
     * Edit profile
     */
    let update_profile: UpdateProfile = mock_update_profile();

    let profile_response: ProfileResponse = edit_profile(update_profile.clone());

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(profile_response.principal, SENDER.with(|s| *s.borrow()));
    // The `username` field of the response should be the same as the `username` of the update profile.
    assert_eq!(profile_response.display_name, update_profile.display_name);

    /*
     * Add starred
     */
    let principal_to_star: Principal = member_test_id();

    let _: ProfileResponse = add_starred(principal_to_star);

    /*
     * Remove starred
     */
    let _: ProfileResponse = remove_starred(principal_to_star);

    /*
     * Add wallet
     */
    let post_wallet: PostWallet = mock_post_wallet();

    let _: ProfileResponse = add_wallet_to_profile(post_wallet);

    /*
     * Remove wallet
     */
    let wallet_principal: Principal = canister_test_id();

    let _: ProfileResponse = remove_wallet_from_profile(wallet_principal);
}
