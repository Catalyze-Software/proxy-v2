use crate::{
    helpers::guards::{_raw_has_access, has_access},
    logic::{friend_request_logic::FriendRequestCalls, profile_logic::ProfileCalls},
};
/// # Profile methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Application) role based authentication

/// # Questions
/// Check the public / private access of these calls? (anon / registered / application role)
///
use candid::Principal;
use catalyze_shared::{
    friend_request::FriendRequestResponse,
    helpers::guards::is_not_anonymous,
    profile_with_refs::{PostProfile, ProfileResponse, UpdateProfile},
    relation_type::RelationType,
    subject::{Subject, SubjectResponse, SubjectType},
    wallet::PostWallet,
    CanisterResult,
};
use ic_cdk::{caller, query, update};

/// Adds a profile to the canister - [`[update]`](update)
/// # Arguments
/// * `post_profile` - The profile to add
/// # Returns
/// * `ProfileResponse` - The profile that was added
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`is_not_anonymous`](is_not_anonymous) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_profile(post_profile: PostProfile) -> CanisterResult<ProfileResponse> {
    ProfileCalls::add_profile(post_profile).await
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
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_profile(principal: Principal) -> CanisterResult<ProfileResponse> {
    has_access().await?;
    ProfileCalls::get_profile(principal).await
}

/// Gets profiles by the given user principals - [`[query]`](query)
/// # Arguments
/// * `principals` - The user principals to get the profiles by
/// # Returns
/// * `Vec<ProfileResponse>` - The profiles that were found
/// # Errors
/// * `ApiError` - If something went wrong getting the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_profiles(principals: Vec<Principal>) -> CanisterResult<Vec<ProfileResponse>> {
    has_access().await?;
    ProfileCalls::get_profiles(principals).await
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
#[update(guard = "is_not_anonymous")]
pub async fn edit_profile(update_profile: UpdateProfile) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::update_profile(update_profile).await
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
#[update(guard = "is_not_anonymous")]
pub async fn add_wallet_to_profile(wallet: PostWallet) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::add_wallet_to_profile(wallet).await
}

/// Sets a wallet as the primary wallet of the caller his profile - [`[update]`](update)
/// # Arguments
/// * `wallet_principal` - The wallet principal to set as the primary wallet
/// # Returns
/// * `ProfileResponse` - Profile response with the updated wallet
/// # Errors
/// * `ApiError` - If something went wrong while setting the wallet as the primary wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn set_wallet_as_primary(wallet_principal: Principal) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::set_wallet_as_primary(wallet_principal).await
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
#[update(guard = "is_not_anonymous")]
pub async fn remove_wallet_from_profile(
    wallet_principal: Principal,
) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::remove_wallet_from_profile(wallet_principal).await
}

/// Adds a starred subject to the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to add to starred
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_starred(subject: Subject) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::add_starred(subject).await
}

/// Removes a starred subject from the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to remove from starred
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_starred(subject: Subject) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::remove_starred(subject).await
}

/// Gets the starred subjects from the caller his profile - [`[query]`](query)
/// # Arguments
/// * `subject_type` - The starred subjects type to fetch
/// # Returns
/// * `Vec<Principal>` - The group identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_starred_by_subject_type(subject_type: SubjectType) -> CanisterResult<Vec<u64>> {
    has_access().await?;
    Ok(ProfileCalls::get_starred_by_subject(subject_type).await)
}

/// Adds a pinned subject to the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to add to pinned
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::add_pinned(subject).await
}

/// Removes a pinned subject from the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to remove from pinned
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::remove_pinned(subject).await
}

/// Gets the pinned subjects from the caller his profile - [`[query]`](query)
/// # Arguments
/// * `subject_type` - The pinned subjects type to fetch
/// # Returns
/// * `Vec<Principal>` - The group identifiers that were found
/// # Errors
/// * `ApiError` - If something went wrong while getting pinned subjects
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_pinned_by_subject_type(
    subject_type: SubjectType,
) -> CanisterResult<Vec<SubjectResponse>> {
    has_access().await?;
    ProfileCalls::get_pinned_by_subject(subject_type).await
}

/// Create a friend request on behalf of the caller - [`[update]`](update)
/// # Arguments
/// * `to` - The principal to send the friend request to
/// * `message` - The message to send with the friend request
/// # Returns
/// * `FriendRequestResponse` - The friend request that was created
/// # Errors
/// * `ApiError` - If something went wrong while creating the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_friend_request(
    to: Principal,
    message: String,
) -> CanisterResult<FriendRequestResponse> {
    _raw_has_access().await?;
    FriendRequestCalls::add_friend_request(to, message).await
}

/// Accept a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to accept
/// # Returns
/// * `bool` - If the friend request was accepted
/// # Errors
/// * `ApiError` - If something went wrong while accepting the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn accept_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    FriendRequestCalls::accept_friend_request(friend_request_id).await
}

/// Remove a friend request created by the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to remove
/// # Returns
/// * `bool` - If the friend request was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    FriendRequestCalls::remove_friend_request(friend_request_id).await
}

/// Gets the friend requests that are addressed to the caller - [`[query]`](query)
/// # Returns
/// * `Vec<FriendRequestResponse>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_incoming_friend_requests() -> CanisterResult<Vec<FriendRequestResponse>> {
    has_access().await?;
    FriendRequestCalls::get_incoming_friend_requests().await
}

/// Gets the friend requests that are addressed to the caller with the corresponding profile - [`[query]`](query)
/// # Returns
/// * `Vec<(FriendRequestResponse, ProfileResponse)>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_incoming_friend_requests_with_profile(
) -> CanisterResult<Vec<(FriendRequestResponse, ProfileResponse)>> {
    has_access().await?;
    FriendRequestCalls::get_incoming_friend_requests_with_profile().await
}

/// Gets the friend requests that are send by the caller - [`[query]`](query)
/// # Returns
/// * `Vec<FriendRequestResponse>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_outgoing_friend_requests() -> CanisterResult<Vec<FriendRequestResponse>> {
    has_access().await?;
    FriendRequestCalls::get_outgoing_friend_requests().await
}

/// Gets the friend requests that are send to the caller with the corresponding profile - [`[query]`](query)
/// # Returns
/// * `Vec<(FriendRequestResponse, ProfileResponse)>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_outgoing_friend_requests_with_profile(
) -> CanisterResult<Vec<(FriendRequestResponse, ProfileResponse)>> {
    has_access().await?;
    FriendRequestCalls::get_outgoing_friend_requests_with_profile().await
}

/// Decline a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to decline
/// # Returns
/// * `bool` - If the friend request was declined
/// # Errors
/// * `ApiError` - If something went wrong while declining the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn decline_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    FriendRequestCalls::decline_friend_request(friend_request_id).await
}

/// Remove friend from caller profile and remove caller from friend profile - [`[update]`](update)
/// # Arguments
/// * `principal` - The friend principal to remove from the caller his profile
/// # Returns
/// * `bool` - If the friend was removed from the caller his profile
/// # Errors
/// * `ApiError` - If something went wrong while removing the friend
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_friend(principal: Principal) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::remove_friend(principal).await
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
#[update(guard = "is_not_anonymous")]
pub async fn block_user(principal: Principal) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::block_user(principal).await
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
#[update(guard = "is_not_anonymous")]
pub async fn unblock_user(principal: Principal) -> CanisterResult<ProfileResponse> {
    _raw_has_access().await?;
    ProfileCalls::unblock_user(principal).await
}

/// Get the current relation for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<Principal>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations(relation_type: RelationType) -> CanisterResult<Vec<Principal>> {
    has_access().await?;
    ProfileCalls::get_relations(caller(), relation_type).await
}
/// Get the current relation for the principal based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation for
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<Principal>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations_by_principal(
    principal: Principal,
    relation_type: RelationType,
) -> CanisterResult<Vec<Principal>> {
    has_access().await?;
    ProfileCalls::get_relations(principal, relation_type).await
}

/// Get the current relation profiles for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<ProfileResponse>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations_with_profiles(
    relation_type: RelationType,
) -> CanisterResult<Vec<ProfileResponse>> {
    has_access().await?;
    ProfileCalls::get_relations_with_profiles(caller(), relation_type).await
}

/// Get the current relation profiles for the principal based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation for
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<ProfileResponse>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations_with_profiles_by_principal(
    principal: Principal,
    relation_type: RelationType,
) -> CanisterResult<Vec<ProfileResponse>> {
    has_access().await?;
    ProfileCalls::get_relations_with_profiles(principal, relation_type).await
}

/// Get the current relation count for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation count for
/// * `relation_type` - The relation type to get the relation count for `friend` or `blocked`
/// # Returns
/// * `u64` - The relation count that was found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations_count(relation_type: RelationType) -> CanisterResult<u64> {
    has_access().await?;
    Ok(ProfileCalls::get_relations(caller(), relation_type)
        .await?
        .len() as u64)
}

/// Get the current relation count for the principal based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation count for
/// * `relation_type` - The relation type to get the relation count for `friend` or `blocked`
/// # Returns
/// * `u64` - The relation count that was found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_relations_count_by_principal(
    principal: Principal,
    relation_type: RelationType,
) -> CanisterResult<u64> {
    has_access().await?;
    Ok(ProfileCalls::get_relations(principal, relation_type)
        .await?
        .len() as u64)
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
#[update(guard = "is_not_anonymous")]
pub async fn approve_code_of_conduct(version: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    ProfileCalls::approve_code_of_conduct(version).await
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
#[update(guard = "is_not_anonymous")]
pub async fn approve_privacy_policy(version: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    ProfileCalls::approve_privacy_policy(version).await
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
#[update(guard = "is_not_anonymous")]
pub async fn approve_terms_of_service(version: u64) -> CanisterResult<bool> {
    _raw_has_access().await?;
    ProfileCalls::approve_terms_of_service(version).await
}
