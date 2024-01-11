use std::f32::consts::E;

/// # Event methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_events` give back a more summier response per group compared to the `get_event` function?
/// And what about the public / private access of these calls?
///
use candid::Principal;
use ic_cdk::{query, update};

use crate::{
    entities::{
        attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
        event::{EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent},
    },
    helpers::auth_helper::has_access,
    models::{
        api_error::ApiError, filter_type::FilterType, paged_response::PagedResponse,
        privacy::Privacy,
    },
};

/// Add an event - [`[update]`](update)
/// # Arguments
/// * `value` - The event to add
/// * `group_identifier` - Used to check if the user has access to the group
/// * `member_identifier` - Used to check if the user has the correct group roles
/// * `event_attendee_canister` - The event attendee canister to store the event owner on (icc)
/// # Returns
/// * `EventResponse` - The added event
/// # Errors
/// * `ApiError` - If something went wrong while adding the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_event(
    value: PostEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> Result<EventResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get an event - [`[query]`](query)
/// # Arguments
/// * `identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The event
/// # Errors
/// * `ApiError` - If something went wrong while getting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_event(
    identifier: Principal,
    group_identifier: Option<Principal>,
) -> Result<EventResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get an event privacy and owner - [`[query]`](query)
/// # Arguments
/// * `identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The event
/// # Errors
/// * `ApiError` - If something went wrong while getting the event
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
#[query]
pub fn get_event_privacy_and_owner(
    identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Privacy), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get paged events - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of events to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// * `filter_type` - The filter type
/// * `group_identifier` -The group identifier to get the events from
/// # Returns
/// * `PagedResponse<EventResponse>` - The events in a paged response
/// # Errors
/// * `ApiError` - If something went wrong while getting the events
#[query]
fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filter: Vec<EventFilter>,
    filter_type: FilterType,
    group_identifier: Option<Principal>,
) -> Result<PagedResponse<EventResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the number of events per group - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The group identifiers to get the events count from
/// # Returns
/// * `Vec<(Principal, usize)>` - The events count per group
#[query]
pub fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    vec![]
}

/// edit an event - [`[update]`](update)
/// # Arguments
/// * `identifier` - The identifier of the event
/// * `value` - The event to update
/// * `group_identifier` - Used to check if the user has access to the group
/// * `member_identifier` - Used to check if the user has the correct group roles
/// * `event_attendee_canister` - The event attendee canister to store the event owner on (icc)
/// # Returns
/// * `EventResponse` - The updated event
/// # Errors
/// * `ApiError` - If something went wrong while updating the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_event(
    identifier: Principal,
    value: UpdateEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> Result<EventResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Delete an event - [`[update]`](update)
/// # Arguments
/// * `identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `()` - If the event was deleted
/// # Errors
/// * `ApiError` - If something went wrong while deleting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_event(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Cancel an event - [`[update]`](update)
/// # Arguments
/// * `identifier` - The identifier of the event
/// * `reason` - The reason why the event was cancelled
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `()` - If the event was cancelled
/// # Errors
/// * `ApiError` - If something went wrong while cancelling the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn cancel_event(
    identifier: Principal,
    reason: String,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Update the attendee count on the event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `event_attendee_canister` - The event attendee canister to store the event owner on (icc)
/// * `attendee_count` - The new attendee count
/// # Returns
/// * `()` - If the attendee count was updated
/// # Errors
/// * `bool` - If something went wrong while updating the attendee count
/// # Note
/// This function was triggered by an inter-canister call to update the event attendee count on the event.
/// TODO: if used it required a auth guard so it can only be called by the known canisters
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
#[update]
pub fn update_attendee_count_on_event(
    event_identifier: Principal,
    event_attendee_canister: Principal,
    attendee_count: usize,
) -> Result<(), bool> {
    Err(false)
}

// Attendee methods

/// Join an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `(Principal, Attendee)` - The event identifier and the attendee
/// # Errors
/// * `ApiError` - If something went wrong while joining the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn join_event(
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Attendee), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Invite a user to an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `attendee_principal` - The principal of the user to invite
/// * `member_identifier` - Used to check if the user has the correct group roles
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `(Principal, Attendee)` - The event identifier and the attendee
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn invite_to_event(
    event_identifier: Principal,
    attendee_principal: Principal,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Attendee), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Accept an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to accept
/// * `event_identifier` - The identifier of the event
/// * `member_identifier` - Used to check if the user has the correct group roles
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `(Principal, Attendee)` - The event identifier and the attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn accept_user_request_event_invite(
    attendee_principal: Principal,
    event_identifier: Principal,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Attendee), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Accept an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `(Principal, Attendee)` - The event identifier and the attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_owner_request_event_invite(
    event_identifier: Principal,
) -> Result<(Principal, Attendee), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the number of attendees for events - [`[query]`](query)
/// # Arguments
/// * `event_identifiers` - The event identifiers to get the attendees count from
/// # Returns
/// * `Vec<(Principal, usize)>` - (event identifier, attendee count) The attendees count per event
/// # Note
/// This call shouldnt be needed anymore, but is kept for now for backwards compatibility
#[query]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn get_event_attendees_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    vec![]
}

/// Get the number of invites for events - [`[query]`](query)
/// # Arguments
/// * `event_identifiers` - The event identifiers to get the attendees count from
/// # Returns
/// * `Vec<(Principal, usize)>` - (event identifier, invite count) The invite count per event
/// # Note
/// This call shouldnt be needed anymore, but is kept for now for backwards compatibility
#[query]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn get_event_invites_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    vec![]
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_identifier` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_event_attendees(
    event_identifier: Principal,
) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the caller attendee entry - [`[query]`](query)
/// # Returns
/// * `(Principal, Attendee)` - The attendee identifier and the attendee
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendee
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_self() -> Result<(Principal, Attendee), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the joined events from a principal - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the joined events from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The joined events
/// # Errors
/// * `ApiError` - If something went wrong while getting the joined events
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_attending_from_principal(
    principal: Principal,
) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Leave an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `()` - If the user left the event
/// # Errors
/// * `ApiError` - If something went wrong while leaving the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn leave_event(event_identifier: Principal) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove an event invite as a user - [`[update]`](update)
/// # Change
/// * was `remove_event`
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_event_invite(event_identifier: Principal) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove an event attendee as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to remove
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_attendee_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove an event invite as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to remove
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_attendee_invite_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the invites for an event - [`[query]`](query)
/// # Arguments
/// * `event_identifier` - The event identifier to get the invites from
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `Vec<InviteAttendeeResponse>` - The invites for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the invites for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(guard = "has_access")]
pub fn get_event_invites(
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Add the event creator as an attendee - [`[update]`](update)
/// # Arguments
/// * `user_principal` - The principal of the user to add
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the user was added as an attendee
/// # Errors
/// * `bool` - If something went wrong while adding the user as an attendee
/// # Note
/// This method can probably be removed, but is kept for now for backwards compatibility
#[deprecated = "Don't use this function anymore, it is kept for backwards compatibility"]
#[update]
pub fn add_owner_as_attendee(
    user_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), bool> {
    Err(false)
}
