use crate::{ENV, SENDER};
use candid::Principal;
use models::models::{
    api_error::ApiError,
    friend_request::FriendRequestResponse,
    profile::{PostProfile, ProfileResponse, UpdateProfile},
    relation_type::RelationType,
    wallet::PostWallet,
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_profile(post_profile: PostProfile, member_canister: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(PostProfile, Principal), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_profile",
            (post_profile, member_canister),
        )
        .expect("Failed to call add_profile from pocketIC")
        .0
        .expect("Failed to add profile");

    profile_response
}

pub fn get_profile_by_user_principal(principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        query_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_profile_by_user_principal",
            (principal,),
        )
        .expect("Failed to call get_profile_by_user_principal from pocketIC")
        .0
        .expect("Failed to get profile by user principal");

    profile_response
}

// #[deprecated = "should be removed in favor of get_profile_by_user_principal"]
// pub fn get_profile_by_identifier(id: Principal) -> Result<ProfileResponse, ApiError>

pub fn get_profiles_by_user_principal(principals: Vec<Principal>) -> Vec<ProfileResponse> {
    let profiles: Vec<ProfileResponse> =
        query_candid_as::<(Vec<Principal>,), (Result<Vec<ProfileResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_profiles_by_user_principal",
            (principals,),
        )
        .expect("Failed to call get_profiles_by_user_principal from pocketIC")
        .0
        .expect("Failed to get profiles by user principal");

    profiles
}

// #[deprecated = "should be removed in favor of get_profiles_by_user_principal"]
// pub fn get_profiles_by_identifier(identifiers: Vec<Principal>) -> Vec<ProfileResponse>

pub fn edit_profile(update_profile: UpdateProfile) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(UpdateProfile,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "edit_profile",
            (update_profile,),
        )
        .expect("Failed to call update_profile from pocketIC")
        .0
        .expect("Failed to update profile");

    profile_response
}

pub fn add_wallet_to_profile(wallet: PostWallet) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(PostWallet,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_wallet_to_profile",
            (wallet,),
        )
        .expect("Failed to call add_wallet_to_profile from pocketIC")
        .0
        .expect("Failed to add wallet to profile");

    profile_response
}

pub fn set_wallet_as_primary(wallet_principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "set_wallet_as_primary",
            (wallet_principal,),
        )
        .expect("Failed to call set_wallet_as_primary from pocketIC")
        .0
        .expect("Failed to set wallet as primary");

    profile_response
}

pub fn remove_wallet_from_profile(wallet_principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "remove_wallet_from_profile",
            (wallet_principal,),
        )
        .expect("Failed to call remove_wallet_from_profile from pocketIC")
        .0
        .expect("Failed to remove wallet from profile");

    profile_response
}

pub fn add_starred(identifier: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_starred",
            (identifier,),
        )
        .expect("Failed to call add_starred from pocketIC")
        .0
        .expect("Failed to add starred");

    profile_response
}

pub fn remove_starred(identifier: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "remove_starred",
            (identifier,),
        )
        .expect("Failed to call remove_starred from pocketIC")
        .0
        .expect("Failed to remove starred");

    profile_response
}

pub fn get_starred_events() -> Vec<Principal> {
    let starred_events: Vec<Principal> =
        query_candid_as::<(), (Result<Vec<Principal>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_starred_events",
            (),
        )
        .expect("Failed to call get_starred_events from pocketIC")
        .0
        .expect("Failed to get starred events");

    starred_events
}

pub fn get_starred_tasks() -> Vec<Principal> {
    let starred_tasks: Vec<Principal> = query_candid_as::<(), (Result<Vec<Principal>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_starred_tasks",
        (),
    )
    .expect("Failed to call get_starred_tasks from pocketIC")
    .0
    .expect("Failed to get starred tasks");

    starred_tasks
}

pub fn get_starred_groups() -> Vec<Principal> {
    let starred_groups: Vec<Principal> =
        query_candid_as::<(), (Result<Vec<Principal>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_starred_groups",
            (),
        )
        .expect("Failed to call get_starred_groups from pocketIC")
        .0
        .expect("Failed to get starred groups");

    starred_groups
}

pub fn add_friend_request(to: Principal, message: String) -> FriendRequestResponse {
    let friend_request_response: FriendRequestResponse =
        update_candid_as::<(Principal, String), (Result<FriendRequestResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_friend_request",
            (to, message),
        )
        .expect("Failed to call add_friend_request from pocketIC")
        .0
        .expect("Failed to add friend request");

    friend_request_response
}

pub fn accept_friend_request(id: u64) -> bool {
    let friend_request_accepted: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_friend_request",
        (id,),
    )
    .expect("Failed to call accept_friend_request from pocketIC")
    .0
    .expect("Failed to accept friend request");

    friend_request_accepted
}

pub fn remove_friend_request(id: u64) -> bool {
    let friend_request_removed: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_friend_request",
        (id,),
    )
    .expect("Failed to call remove_friend_request from pocketIC")
    .0
    .expect("Failed to remove friend request");

    friend_request_removed
}

pub fn get_incoming_friend_requests() -> Vec<FriendRequestResponse> {
    let friend_requests: Vec<FriendRequestResponse> =
        query_candid_as::<(), (Result<Vec<FriendRequestResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_incoming_friend_requests",
            (),
        )
        .expect("Failed to call get_incoming_friend_requests from pocketIC")
        .0
        .expect("Failed to get incoming friend requests");

    friend_requests
}

pub fn get_outgoing_friend_requests() -> Vec<FriendRequestResponse> {
    let friend_requests: Vec<FriendRequestResponse> =
        query_candid_as::<(), (Result<Vec<FriendRequestResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_outgoing_friend_requests",
            (),
        )
        .expect("Failed to call get_outgoing_friend_requests from pocketIC")
        .0
        .expect("Failed to get outgoing friend requests");

    friend_requests
}

pub fn decline_friend_request(id: u64) -> bool {
    let friend_request_declined: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "decline_friend_request",
        (id,),
    )
    .expect("Failed to call decline_friend_request from pocketIC")
    .0
    .expect("Failed to decline friend request");

    friend_request_declined
}

pub fn remove_friend(principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "remove_friend",
            (principal,),
        )
        .expect("Failed to call remove_friend from pocketIC")
        .0
        .expect("Failed to remove friend");

    profile_response
}

pub fn block_user(principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "block_user",
            (principal,),
        )
        .expect("Failed to call block_user from pocketIC")
        .0
        .expect("Failed to block user");

    profile_response
}

pub fn unblock_user(principal: Principal) -> ProfileResponse {
    let profile_response: ProfileResponse =
        update_candid_as::<(Principal,), (Result<ProfileResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "unblock_user",
            (principal,),
        )
        .expect("Failed to call unblock_user from pocketIC")
        .0
        .expect("Failed to unblock user");

    profile_response
}

pub fn get_relations(relation_type: String) -> Vec<Principal> {
    let relations: Vec<Principal> =
        query_candid_as::<(String,), (Result<Vec<Principal>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_relations",
            (relation_type,),
        )
        .expect("Failed to call get_relations from pocketIC")
        .0
        .expect("Failed to get relations");

    relations
}

pub fn get_relations_count(principal: Principal, relation_type: RelationType) -> u64 {
    let relations_count: u64 = query_candid_as::<(Principal, RelationType), (u64,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_relations_count",
        (principal, relation_type),
    )
    .expect("Failed to get relations count")
    .0;

    relations_count
}

pub fn approve_code_of_conduct(version: u64) -> bool {
    let code_of_conduct_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "approve_code_of_conduct",
        (version,),
    )
    .expect("Failed to call approve_code_of_conduct from pocketIC")
    .0
    .expect("Failed to approve code of conduct");

    code_of_conduct_approved
}

pub fn approve_privacy_policy(version: u64) -> bool {
    let privacy_policy_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "approve_privacy_policy",
        (version,),
    )
    .expect("Failed to call approve_privacy_policy from pocketIC")
    .0
    .expect("Failed to approve privacy policy");

    privacy_policy_approved
}

pub fn approve_terms_of_service(version: u64) -> bool {
    let terms_of_service_approved: bool = update_candid_as::<(u64,), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "approve_terms_of_service",
        (version,),
    )
    .expect("Failed to call approve_terms_of_service from pocketIC")
    .0
    .expect("Failed to approve terms of service");

    terms_of_service_approved
}
