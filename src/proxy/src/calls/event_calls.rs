use crate::{
    helpers::{
        group_permission::{can_delete, can_edit},
        guards::has_access,
    },
    logic::event_logic::EventCalls,
};
/// # Event methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_events` give back a more summier response per group compared to the `get_event` function?
///
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    attendee::{InviteAttendeeResponse, JoinedAttendeeResponse},
    event_with_attendees::{
        Attendee, EventFilter, EventResponse, EventSort, EventsCount, PostEvent, UpdateEvent,
    },
    guards::is_not_anonymous,
    paged_response::PagedResponse,
    permission::PermissionType,
    profile_with_refs::ProfileResponse,
    CanisterResult,
};
use ic_cdk::{query, update};

/// Add an event - [`[update]`](update)
/// # Arguments
/// * `post_event` - The event to add
/// # Returns
/// * `EventResponse` - The added event
/// # Errors
/// * `ApiError` - If something went wrong while adding the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_event(post_event: PostEvent) -> CanisterResult<EventResponse> {
    has_access().await?;

    let group_id = post_event
        .group_id
        .ok_or_else(|| ApiError::bad_request().add_message("Group id is required"))?;

    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::add_event(post_event).await
}

/// Get an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The event
/// # Errors
/// * `ApiError` - If something went wrong while getting the event
#[query(composite = true)]
pub async fn get_event(event_id: u64) -> CanisterResult<EventResponse> {
    EventCalls::get_event(event_id).await
}

/// Get paged events - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of events to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// # Returns
/// * `PagedResponse<EventResponse>` - The events in a paged response
/// # Errors
/// * `ApiError` - If something went wrong while getting the events
#[query(composite = true)]
async fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filters: Vec<EventFilter>,
) -> CanisterResult<PagedResponse<EventResponse>> {
    EventCalls::get_events(limit, page, sort, filters).await
}

/// Get events count - [`[query]`](query)
/// # Arguments
/// * `group_ids` - Optional group ids to filter the events count
/// * `query` - Optional query to filter the events
/// # Returns
/// * `EventsCount` - The events in a paged response
#[query(composite = true)]
async fn get_event_count(
    group_ids: Option<Vec<u64>>,
    query: Option<String>,
) -> CanisterResult<EventsCount> {
    EventCalls::get_events_count(group_ids, query).await
}

/// edit an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group
/// * `update_event` - The event to update
/// # Returns
/// * `EventResponse` - The updated event
/// # Errors
/// * `ApiError` - If something went wrong while updating the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn edit_event(
    event_id: u64,
    group_id: u64,
    update_event: UpdateEvent,
) -> CanisterResult<EventResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::edit_event(event_id, update_event, group_id).await
}

/// Delete an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the event was deleted
/// # Errors
/// * `ApiError` - If something went wrong while deleting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn delete_event(event_id: u64, group_id: u64) -> CanisterResult<()> {
    has_access().await?;
    can_delete(group_id, PermissionType::Event(None)).await?;
    EventCalls::delete_event(event_id, group_id).await
}

/// Cancel an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `reason` - The reason why the event was cancelled
/// # Returns
/// * `()` - If the event was cancelled
/// # Errors
/// * `ApiError` - If something went wrong while cancelling the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn cancel_event(event_id: u64, group_id: u64, reason: String) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::cancel_event(event_id, reason, group_id).await
}

// Attendee methods

/// Join an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while joining the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn join_event(event_id: u64) -> CanisterResult<JoinedAttendeeResponse> {
    has_access().await?;
    EventCalls::join_event(event_id).await
}

/// Invite a user to an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to invite
/// # Returns
/// * `InviteAttendeeResponse` - The event invite details
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn invite_to_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> CanisterResult<InviteAttendeeResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::invite_to_event(event_id, attendee_principal, group_id).await
}

/// Accept an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to accept
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn accept_user_request_event_invite(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> CanisterResult<JoinedAttendeeResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::accept_or_decline_user_request_event_invite(
        event_id,
        attendee_principal,
        group_id,
        true,
    )
    .await
}

/// Decline an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to accept
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn decline_user_request_event_invite(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> CanisterResult<JoinedAttendeeResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::accept_or_decline_user_request_event_invite(
        event_id,
        attendee_principal,
        group_id,
        false,
    )
    .await
}

/// Accept an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn accept_owner_request_event_invite(event_id: u64) -> CanisterResult<Attendee> {
    has_access().await?;
    EventCalls::accept_or_decline_owner_request_event_invite(event_id, true).await
}

/// Decline an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn decline_owner_request_event_invite(event_id: u64) -> CanisterResult<Attendee> {
    has_access().await?;
    EventCalls::accept_or_decline_owner_request_event_invite(event_id, false).await
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_event_attendees(event_id: u64) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
    has_access().await?;
    EventCalls::get_event_attendees(event_id).await
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_event_attendees_profiles_and_roles(
    event_id: u64,
) -> CanisterResult<Vec<(ProfileResponse, Vec<String>)>> {
    has_access().await?;
    EventCalls::get_event_attendees_profiles_and_roles(event_id).await
}

/// Get the attendees for an event with their profiles - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_event_invites_with_profiles(
    event_id: u64,
) -> CanisterResult<Vec<(ProfileResponse, InviteAttendeeResponse)>> {
    has_access().await?;
    EventCalls::get_event_invites_with_profiles(event_id).await
}

/// Get the caller attendee entry - [`[query]`](query)
/// # Change
/// * was `get_self` but due to conflict with other methods it was renamed
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendee
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_self_attendee() -> CanisterResult<Attendee> {
    has_access().await?;
    EventCalls::get_self_attendee().await
}
/// Get the caller joined groups - [`[query]`](query)
/// # Returns
/// * `Vec<GroupResponse>` - All groups the user is part of
/// # Errors
/// * `ApiError` - If something went wrong while getting the self events
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_self_events() -> CanisterResult<Vec<EventResponse>> {
    has_access().await?;
    EventCalls::get_self_events().await
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
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_attending_from_principal(
    principal: Principal,
) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
    has_access().await?;
    EventCalls::get_attending_from_principal(principal).await
}

/// Leave an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `()` - If the user left the event
/// # Errors
/// * `ApiError` - If something went wrong while leaving the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn leave_event(event_id: u64) -> CanisterResult<()> {
    has_access().await?;
    EventCalls::leave_event(event_id).await
}

/// Remove an event invite as a user - [`[update]`](update)
/// # Change
/// * was `remove_event`
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_event_invite(event_id: u64) -> CanisterResult<()> {
    has_access().await?;
    EventCalls::remove_event_invite(event_id).await
}

/// Remove an event attendee as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to remove
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_attendee_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::remove_attendee_from_event(attendee_principal, event_id).await
}

/// Remove an event invite as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to remove
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_attendee_invite_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Event(None)).await?;
    EventCalls::remove_attendee_invite_from_event(attendee_principal, event_id).await
}

/// Get the invites for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the invites from
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `Vec<InviteAttendeeResponse>` - The invites for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the invites for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_event_invites(
    event_id: u64,
    group_id: u64,
) -> CanisterResult<Vec<InviteAttendeeResponse>> {
    has_access().await?;
    EventCalls::get_event_invites(event_id, group_id).await
}
