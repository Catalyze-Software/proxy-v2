use candid::Principal;
use ic_cdk::caller;

use crate::{
    models::{
        api_error::ApiError,
        friend_request::{FriendRequest, FriendRequestResponse},
    },
    storage::storage_api::{friend_requests, profiles, StorageMethods},
};

pub struct FriendRequestCalls;
pub struct FriendRequestMapper;
pub struct FriendRequestValidation;

impl FriendRequestCalls {
    pub fn add_friend_request(to: Principal, message: String) -> Result<FriendRequest, ApiError> {
        let friend_request = FriendRequest::new(caller(), to, message);
        let profile = profiles().get(caller())?;

        if profile.relations.contains_key(&to) {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        // if somebody tries to make the same friend request
        if friend_requests()
            .find(|request| request.to == to && request.requested_by == caller())
            .is_some()
        {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        // if the receipient tries to make a friend request to the caller accept the request
        if friend_requests()
            .find(|request| request.to == caller() && request.requested_by == to)
            .is_some()
        {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        let new_friend_request = friend_requests().insert(friend_request)?;
        Ok(new_friend_request)
    }
}

impl FriendRequestMapper {
    pub fn to_response(
        id: u64,
        friend_request_result: Result<FriendRequest, ApiError>,
    ) -> Result<FriendRequestResponse, ApiError> {
        match friend_request_result {
            Ok(friend_request) => Ok(FriendRequestResponse {
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

impl FriendRequestValidation {}
