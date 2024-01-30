/// # Profile methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Application) role based authentication

/// # Questions
/// Check the public / private access of these calls? (anon / registered / application role)
///
use candid::Principal;
use ic_cdk::{query, update};

use crate::{
    helpers::guards::{has_access, is_not_anonymous},
    logic::profile_logic::ProfileCalls,
    models::{api_error::ApiError, relation_type::RelationType, wallet::PostWallet},
    models::{
        friend_request::FriendRequestResponse,
        profile::{PostProfile, ProfileResponse, UpdateProfile},
    },
};

/// Adds a profile to the canister - [`[update]`](update)
/// # Arguments
/// * `post_profile` - The profile to add
/// * `member_canister` - Canister ID of the member canister to add an empty member
/// # Returns
/// * `ProfileResponse` - The profile that was added
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`is_not_anonymous`](is_not_anonymous) function.
#[update(guard = "is_not_anonymous")]
pub fn add_profile(
    post_profile: PostProfile,
    member_canister: Principal, // should be deprecated
) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_profile(post_profile)
}

/// Gets a profile by the given user principal - [`[query]`](query)
/// # Arguments
/// * `principal` - The user principal to get the profile by
/// # Returns
/// * `ProfileResponse` - The profile that was found
/// # Errors
/// * `ApiError` - If something went wrong while getting the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_profile_by_user_principal(principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::get_profile(principal)
}

/// Gets a profile by the given user identifier - [`[query]`](query)
/// # Arguments
/// * `id` - The user identifier to get the profile by
/// # Returns
/// * `ProfileResponse` - The profile that was found
/// # Errors
/// * `ApiError` - If something went wrong while getting the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// # Deprecated
/// This function is deprecated and should be removed in favor of `get_profile_by_user_principal`
#[query(guard = "has_access")]
#[deprecated = "should be removed in favor of get_profile_by_user_principal"]
pub fn get_profile_by_identifier(id: Principal) -> Result<ProfileResponse, ApiError> {
    Err(ApiError::not_implemented())
}

/// Gets profiles by the given user principals - [`[query]`](query)
/// # Arguments
/// * `principals` - The user principals to get the profiles by
/// # Returns
/// * `Vec<ProfileResponse>` - The profiles that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_profiles_by_user_principal(principals: Vec<Principal>) -> Vec<ProfileResponse> {
    ProfileCalls::get_profiles(principals)
}

/// Gets profiles by the given user identifiers - [`[query]`](query)
/// # Arguments
/// * `identifiers` - The user identifiers to get the profiles by
/// # Returns
/// * `Vec<ProfileResponse>` - The profiles that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
#[deprecated = "should be removed in favor of get_profiles_by_user_principal"]
pub fn get_profiles_by_identifier(identifiers: Vec<Principal>) -> Vec<ProfileResponse> {
    vec![]
}

/// Edit the caller his a profile - [`[update]`](update)
/// # Arguments
/// * `update_profile` - The profile to update
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while updating the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_profile(update_profile: UpdateProfile) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::update_profile(update_profile)
}

/// Adds a wallet to the caller his profile - [`[update]`](update)
/// # Change
/// * was `add_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `wallet` - The wallet to add
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_wallet_to_profile(wallet: PostWallet) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_wallet_to_profile(wallet)
}

/// Sets a wallet as the primary wallet of the caller his profile - [`[update]`](update)
/// # Arguments
/// * `wallet_principal` - The wallet principal to set as the primary wallet
/// # Returns
/// * `()` - If the wallet was set as the primary wallet
/// # Errors
/// * `()` - If something went wrong while setting the wallet as the primary wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn set_wallet_as_primary(wallet_principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::set_wallet_as_primary(wallet_principal)
}

/// Removes a wallet from the caller his profile - [`[update]`](update)
/// # Change
/// * was `remove_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `wallet_principal` - The wallet to remove
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_wallet_from_profile(
    wallet_principal: Principal,
) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_wallet_from_profile(wallet_principal)
}

/// Adds a group, event or task reference to the caller his profile - [`[update]`](update)
/// # Arguments
/// * `identifier` - The group, event or task identifier to add
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_starred(identifier: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_starred(identifier)
}

/// Removes a group, event or task reference from the caller his profile - [`[update]`](update)
/// # Arguments
/// * `identifier` - The group, event or task identifier to remove
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_starred(identifier: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_starred(identifier)
}

/// Gets the starred events from the caller his profile - [`[query]`](query)
/// # Returns
/// * `Vec<Principal>` - The event identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_starred_events() -> Vec<Principal> {
    ProfileCalls::get_starred_by_kind("evt")
}

/// Gets the starred tasks from the caller his profile - [`[query]`](query)
/// # Returns
/// * `Vec<Principal>` - The task identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_starred_tasks() -> Vec<Principal> {
    ProfileCalls::get_starred_by_kind("tsk")
}

/// Gets the starred groups from the caller his profile - [`[query]`](query)
/// # Returns
/// * `Vec<Principal>` - The group identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_starred_groups() -> Vec<Principal> {
    ProfileCalls::get_starred_by_kind("grp")
}

/// Create a friend request on behalf of the caller - [`[update]`](update)
/// # Arguments
/// * `principal` - The principal to send the friend request to
/// * `message` - The message to send with the friend request
/// # Returns
/// * `FriendRequestResponse` - The friend request that was created
/// # Errors
/// * `ApiError` - If something went wrong while creating the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_friend_request(
    principal: Principal,
    message: String,
) -> Result<FriendRequestResponse, ApiError> {
    Err(ApiError::not_implemented())
}

/// Remove friend from caller profile and remove caller from friend profile - [`[update]`](update)
/// # Arguments
/// * `principal` - The friend principal to remove from the caller his profile
/// # Returns
/// * `bool` - If the friend was removed from the caller his profile
/// # Errors
/// * `String` - If something went wrong while removing the friend
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_friend(principal: Principal) -> Result<bool, String> {
    Ok(true)
}

/// Accept a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `id` - The friend request identifier to accept
/// # Returns
/// * `bool` - If the friend request was accepted
/// # Errors
/// * `String` - If something went wrong while accepting the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_friend_request(id: u64) -> Result<bool, String> {
    Ok(true)
}

/// Remove a friend request created by the caller - [`[update]`](update)
/// # Arguments
/// * `id` - The friend request identifier to remove
/// # Returns
/// * `bool` - If the friend request was removed
/// # Errors
/// * `String` - If something went wrong while removing the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: Not sure why the principal is needed here as an argument
#[update(guard = "has_access")]
pub fn remove_friend_request(principal: Principal, id: u64) -> Result<bool, String> {
    Ok(true)
}

/// Gets the friend requests that are addressed to the caller - [`[query]`](query)
/// # Returns
/// * `Vec<FriendRequestResponse>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_friend_requests() -> Vec<FriendRequestResponse> {
    vec![]
}

/// Decline a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `id` - The friend request identifier to decline
/// # Returns
/// * `bool` - If the friend request was declined
/// # Errors
/// * `String` - If something went wrong while declining the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn decline_friend_request(id: u64) -> Result<bool, String> {
    Ok(true)
}

/// Block a user on the application level - [`[update]`](update)
/// # Arguments
/// * `principal` - The principal to block
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while blocking the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: Check full implementation for this
#[update(guard = "has_access")]
pub fn block_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
    Err(ApiError::not_implemented())
}

/// Unblock a user on the application level - [`[update]`](update)
/// # Arguments
/// * `principal` - The principal to unblock
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while unblocking the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: Check full implementation for this
#[update(guard = "has_access")]
pub fn unblock_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
    Err(ApiError::not_implemented())
}

/// Get the current relation for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Option<Principal>` - The principal of the relation that was found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_relations(relation_type: RelationType) -> Vec<Principal> {
    vec![]
}

/// Get the current relation count for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation count for
/// * `relation_type` - The relation type to get the relation count for `friend` or `blocked`
/// # Returns
/// * `u64` - The relation count that was found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_relations_count(principal: Principal, relation_type: RelationType) -> u64 {
    0
}

/// Approve a code of conduct version - [`[update]`](update)
/// # Arguments
/// * `version` - The code of conduct version to approve
/// # Returns
/// * `bool` - If the code of conduct version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the code of conduct version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_code_of_conduct(version: u64) -> Result<bool, ApiError> {
    Err(ApiError::not_implemented())
}

/// Approve a privacy policy version - [`[update]`](update)
/// # Arguments
/// * `version` - The privacy policy version to approve
/// # Returns
/// * `bool` - If the privacy policy version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the privacy policy version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_privacy_policy(version: u64) -> Result<bool, ApiError> {
    Err(ApiError::not_implemented())
}

/// Approve a terms of service version - [`[update]`](update)
/// # Arguments
/// * `version` - The terms of service version to approve
/// # Returns
/// * `bool` - If the terms of service version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the terms of service version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_terms_of_service(version: u64) -> Result<bool, ApiError> {
    Err(ApiError::not_implemented())
}
