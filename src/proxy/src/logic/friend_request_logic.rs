use std::time::Duration;

use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    friend_request::{FriendRequest, FriendRequestFilter, FriendRequestResponse},
    profile_with_refs::ProfileResponse,
    relation_type::RelationType,
    CanisterResult, Filter, StorageClient, StorageClientInsertable,
};
use ic_cdk::{caller, spawn};
use ic_cdk_timers::set_timer;

use crate::storage::{friend_requests, profiles};

use super::notification_logic::NotificationCalls;

pub struct FriendRequestCalls;
pub struct FriendRequestMapper;
pub struct FriendRequestValidation;

const SECONDS_IN_A_MONTH: u64 = 30 * 24 * 60 * 60;
const FRIEND_REQUEST_AUTO_REMOVE_DELAY: Duration = Duration::from_secs(SECONDS_IN_A_MONTH);

impl FriendRequestCalls {
    pub async fn add_friend_request(
        to: Principal,
        message: String,
    ) -> CanisterResult<FriendRequestResponse> {
        let friend_request = FriendRequest::new(caller(), to, message);
        let (_, caller_profile) = profiles().get(caller()).await?;

        if caller_profile.references.relations.contains_key(&to) {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        // if somebody tries to make the same friend request
        let is_retry = friend_requests()
            .find(FriendRequestFilter::Requestor(caller()).to_vec())
            .await?
            .is_some();
        if is_retry {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        let is_requested_by_recipient = friend_requests()
            .find(FriendRequestFilter::Recipient(to).to_vec())
            .await?
            .is_some();

        // if there is a friend request from the caller to the to
        if is_requested_by_recipient {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        let (friend_request_id, mut inserted_friend_request) =
            friend_requests().insert(friend_request).await?;

        let friend_request_response =
            FriendRequestResponse::new(friend_request_id, inserted_friend_request.clone());

        let notification_id =
            NotificationCalls::notification_add_friend_request(friend_request_response).await?;

        inserted_friend_request.set_notification_id(notification_id);

        let result = friend_requests()
            .update(friend_request_id, inserted_friend_request.clone())
            .await;

        set_timer(FRIEND_REQUEST_AUTO_REMOVE_DELAY, move || {
            spawn(async move {
                let _ = friend_requests().remove(friend_request_id).await;
                NotificationCalls::notification_remove_friend_request(
                    inserted_friend_request.to,
                    friend_request_id,
                )
                .await;
            })
        });

        FriendRequestMapper::to_result_response(result)
    }

    pub async fn accept_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
        let (_, friend_request) = friend_requests().get(friend_request_id).await?;

        if friend_request.to != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("accept_friend_request")
                .add_message("You are not authorized to accept this friend request"));
        }

        let (_, mut caller_profile) = profiles().get(caller()).await?;

        caller_profile.references.relations.insert(
            friend_request.requested_by,
            RelationType::Friend.to_string(),
        );

        let (requested_by_principal, mut to_profile) =
            profiles().get(friend_request.requested_by).await?;

        to_profile
            .references
            .relations
            .insert(friend_request.to, RelationType::Friend.to_string());

        profiles().update(caller(), caller_profile).await?;
        profiles()
            .update(requested_by_principal, to_profile)
            .await?;

        NotificationCalls::notification_accept_or_decline_friend_request(
            (friend_request_id, friend_request),
            true,
        )
        .await?;

        friend_requests().remove(friend_request_id).await
    }

    pub async fn decline_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
        let (_, friend_request) = friend_requests().get(friend_request_id).await?;

        if friend_request.to != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("decline_friend_request")
                .add_message("You are not authorized to decline this friend request"));
        }

        NotificationCalls::notification_accept_or_decline_friend_request(
            (friend_request_id, friend_request),
            false,
        )
        .await?;

        friend_requests().remove(friend_request_id).await
    }

    pub async fn remove_friend_request(friend_request_id: u64) -> CanisterResult<bool> {
        let (_, friend_request) = friend_requests().get(friend_request_id).await?;

        if friend_request.requested_by != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("remove_friend_request")
                .add_message("You are not authorized to remove this friend request"));
        }

        NotificationCalls::notification_remove_friend_request(friend_request.to, friend_request_id)
            .await;
        friend_requests().remove(friend_request_id).await
    }

    pub async fn get_incoming_friend_requests() -> CanisterResult<Vec<FriendRequestResponse>> {
        let reqs = friend_requests()
            .filter(FriendRequestFilter::Recipient(caller()).to_vec())
            .await?
            .into_iter()
            .map(FriendRequestMapper::to_response)
            .collect();

        Ok(reqs)
    }

    pub async fn get_incoming_friend_requests_with_profile(
    ) -> CanisterResult<Vec<(FriendRequestResponse, ProfileResponse)>> {
        let requests = friend_requests()
            .filter(FriendRequestFilter::Requestor(caller()).to_vec())
            .await?;

        let profiles = profiles()
            .get_many(
                requests
                    .iter()
                    .map(|(_, request)| request.requested_by)
                    .collect(),
            )
            .await?;

        if requests.len() != profiles.len() {
            return Err(ApiError::unexpected()
                .add_method_name("get_incoming_friend_requests_with_profile")
                .add_message("Amount of the found profiles and request is not equal"));
        }

        let response = requests
            .into_iter()
            .enumerate()
            .map(|(i, data)| {
                let (principal, profile) = profiles[i].clone();
                (
                    FriendRequestMapper::to_response(data),
                    ProfileResponse::new(principal, profile),
                )
            })
            .collect();

        Ok(response)
    }

    pub async fn get_outgoing_friend_requests() -> CanisterResult<Vec<FriendRequestResponse>> {
        let requests = friend_requests()
            .filter(FriendRequestFilter::Requestor(caller()).to_vec())
            .await?
            .into_iter()
            .map(FriendRequestMapper::to_response)
            .collect();

        Ok(requests)
    }

    pub async fn get_outgoing_friend_requests_with_profile(
    ) -> CanisterResult<Vec<(FriendRequestResponse, ProfileResponse)>> {
        let requests = friend_requests()
            .filter(FriendRequestFilter::Requestor(caller()).to_vec())
            .await?;

        let profiles = profiles()
            .get_many(requests.iter().map(|(_, request)| request.to).collect())
            .await?;

        if requests.len() != profiles.len() {
            return Err(ApiError::unexpected()
                .add_method_name("get_outgoing_friend_requests_with_profile")
                .add_message("Amount of the found profiles and request is not equal"));
        }

        let response = requests
            .into_iter()
            .enumerate()
            .map(|(i, data)| {
                let (principal, profile) = profiles[i].clone();
                (
                    FriendRequestMapper::to_response(data),
                    ProfileResponse::new(principal, profile),
                )
            })
            .collect();

        Ok(response)
    }
}

impl FriendRequestMapper {
    pub fn to_response(friend_request: (u64, FriendRequest)) -> FriendRequestResponse {
        FriendRequestResponse {
            id: friend_request.0,
            requested_by: friend_request.1.requested_by,
            message: friend_request.1.message,
            to: friend_request.1.to,
            created_at: friend_request.1.created_at,
        }
    }
    pub fn to_result_response(
        friend_request_result: Result<(u64, FriendRequest), ApiError>,
    ) -> Result<FriendRequestResponse, ApiError> {
        match friend_request_result {
            Ok((id, friend_request)) => Ok(FriendRequestResponse {
                id,
                requested_by: friend_request.requested_by,
                message: friend_request.message,
                to: friend_request.to,
                created_at: friend_request.created_at,
            }),
            Err(e) => Err(e),
        }
    }
}
