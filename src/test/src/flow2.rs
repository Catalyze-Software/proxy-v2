// Flow 2

use candid::Principal;
use canister_types::models::{
    profile::{PostProfile, ProfileResponse, UpdateProfile},
    wallet::PostWallet,
};

use crate::{
    calls::profile_calls::{
        add_profile, add_starred, add_wallet_to_profile, edit_profile, remove_starred,
        remove_wallet_from_profile, set_wallet_as_primary,
    },
    mocks::{
        models::{mock_post_profile, mock_post_wallet, mock_post_wallet2, mock_update_profile},
        principals::{
            canister_test_id2, member_test_id2, wallet_test_id,
            wallet_test_id2,
        },
    },
    GROUP_ID, SENDER,
};

pub fn flow2() {
    // Set sender principal
    SENDER.with(|s| *s.borrow_mut() = Some(member_test_id2()));
    // Deprecated
    let member_canister: Principal = canister_test_id2();

    /*
     * Add profile
     */
    let post_profile: PostProfile = mock_post_profile();

    let profile_response: ProfileResponse = add_profile(post_profile, member_canister);

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(
        profile_response.principal,
        SENDER.with(|s| s.borrow().unwrap())
    );

    /*
     * Edit profile
     */
    let update_profile: UpdateProfile = mock_update_profile();

    let profile_response: ProfileResponse = edit_profile(update_profile.clone());

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(
        profile_response.principal,
        SENDER.with(|s| s.borrow().unwrap())
    );
    // The `username` field of the response should be the same as the `username` of the update profile.
    assert_eq!(profile_response.display_name, update_profile.display_name);

    /*
     * Add starred
     */
    let principal_to_star: Principal = GROUP_ID.with(|g| g.borrow().unwrap());

    let _: ProfileResponse = add_starred(principal_to_star);

    /*
     * Remove starred
     */
    let _: ProfileResponse = remove_starred(principal_to_star);

    /*
     * Add wallets
     */
    let post_wallet: PostWallet = mock_post_wallet();
    let post_wallet2: PostWallet = mock_post_wallet2();

    let _: ProfileResponse = add_wallet_to_profile(post_wallet);
    let _: ProfileResponse = add_wallet_to_profile(post_wallet2);

    /*
     * Set primary wallet
     */
    let _: ProfileResponse = set_wallet_as_primary(wallet_test_id2());

    /*
     * Remove wallet
     */
    let _: ProfileResponse = remove_wallet_from_profile(wallet_test_id());
}
