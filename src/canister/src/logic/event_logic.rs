use candid::Principal;
use ic_cdk::caller;

use crate::{
    models::{
        api_error::ApiError,
        attendee::{InviteAttendeeResponse, JoinedAttendeeResponse},
        event::{Event, EventResponse, PostEvent, UpdateEvent},
        invite::InviteType,
    },
    storage::storage_api::{attendees, events, IdentifierRefMethods, StorageMethods},
};

pub fn add_event(post_event: PostEvent, group_id: u64) -> Result<EventResponse, ApiError> {
    let (new_event_id, new_event) = events().insert(Event::from(post_event))?;
    events().insert_identifier_ref(new_event_id)?;
    Ok(EventResponse::new(new_event_id, new_event))
}

pub fn get_event(event_id: u64, group_id: u64) -> Result<EventResponse, ApiError> {
    let (_, event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }
    Ok(EventResponse::new(event_id, event))
}

// fn get_events(
//     limit: usize,
//     page: usize,
//     sort: EventSort,
//     filter: Vec<EventFilter>,
//     filter_type: Vec<FilterType<EventFilter>>,
//     group_identifier: Option<Principal>,
// ) -> Result<PagedResponse<EventResponse>, ApiError> {
//     Err(ApiError::not_implemented())
// }

pub fn edit_event(
    event_id: u64,
    update_event: UpdateEvent,
    group_id: u64,
) -> Result<EventResponse, ApiError> {
    let (_, mut event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    event = event.update(update_event);
    events().update(event_id, event.clone())?;

    Ok(EventResponse::new(event_id, event))
}

pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
    let (_, event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    let _ = events().remove(event_id);
    Ok(())
}

pub fn cancel_event(event_id: u64, reason: String, group_id: u64) -> Result<(), ApiError> {
    let (_, mut event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    event = event.cancel(reason);
    events().update(event_id, event.clone())?;

    Ok(())
}

// Attendee methods
// TODO: add logic for event privacy
pub fn join_event(event_id: u64, group_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
    let (attendee_principal, mut attendee) = attendees().get(caller())?;
    let (_, event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    attendee.add_joined(event_id, group_id);
    attendees().update(attendee_principal, attendee)?;

    Ok(JoinedAttendeeResponse::new(
        event_id,
        group_id,
        attendee_principal,
    ))
}

pub fn invite_to_event(
    event_id: u64,
    attendee_principal: Principal,
    group_id: u64,
) -> Result<InviteAttendeeResponse, ApiError> {
    let (caller_principal, _) = attendees().get(caller())?;
    let (_, event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    let (attendee_principal, mut attendee) = attendees().get(attendee_principal)?;

    attendee.add_invite(event_id, group_id, InviteType::OwnerRequest);
    attendees().update(attendee_principal, attendee)?;

    Ok(InviteAttendeeResponse::new(
        event_id,
        group_id,
        attendee_principal,
        InviteType::OwnerRequest,
    ))
}

pub fn accept_user_request_event_invite(
    event_id: u64,
    attendee_principal: Principal,
    group_id: u64,
) -> Result<JoinedAttendeeResponse, ApiError> {
    let (_, event) = events().get(event_id)?;

    if !event.is_from_group(group_id) {
        return Err(ApiError::unauthorized());
    }

    let (attendee_principal, mut attendee) = attendees().get(attendee_principal)?;

    if !attendee.has_pending_join_request(event_id) {
        return Err(ApiError::not_found());
    }

    attendee.add_joined(event_id, group_id);
    attendees().update(attendee_principal, attendee)?;
    Ok(JoinedAttendeeResponse::new(
        event_id,
        group_id,
        attendee_principal,
    ))
}

// pub fn accept_owner_request_event_invite(
//     event_identifier: Principal,
// ) -> Result<(Principal, Attendee), ApiError> {
//     Err(ApiError::not_implemented())
// }

// #[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
// pub fn get_event_attendees_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
//     vec![]
// }

// #[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
// pub fn get_event_invites_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
//     vec![]
// }

// #[query(guard = "has_access")]
// pub fn get_event_attendees(
//     event_identifier: Principal,
// ) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
//     Err(ApiError::not_implemented())
// }

// #[query(guard = "has_access")]
// pub fn get_self_events() -> Result<(Principal, Attendee), ApiError> {
//     Err(ApiError::not_implemented())
// }

// #[query(guard = "has_access")]
// pub fn get_attending_from_principal(
//     principal: Principal,
// ) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
//     Err(ApiError::not_implemented())
// }

// pub fn leave_event(event_identifier: Principal) -> Result<(), ApiError> {
//     Err(ApiError::not_implemented())
// }

// pub fn remove_event_invite(event_identifier: Principal) -> Result<(), ApiError> {
//     Err(ApiError::not_implemented())
// }

// pub fn remove_attendee_from_event(
//     attendee_principal: Principal,
//     event_identifier: Principal,
//     group_identifier: Principal,
//     member_identifier: Principal,
// ) -> Result<(), ApiError> {
//     Err(ApiError::not_implemented())
// }

// pub fn remove_attendee_invite_from_event(
//     attendee_principal: Principal,
//     event_identifier: Principal,
//     group_identifier: Principal,
//     member_identifier: Principal,
// ) -> Result<(), ApiError> {
//     Err(ApiError::not_implemented())
// }

// #[query(guard = "has_access")]
// pub fn get_event_invites(
//     event_identifier: Principal,
//     group_identifier: Principal,
//     member_identifier: Principal,
// ) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
//     Err(ApiError::not_implemented())
// }

// #[deprecated = "Don't use this function anymore, it is kept for backwards compatibility"]
// pub fn add_owner_as_attendee(
//     user_principal: Principal,
//     event_identifier: Principal,
//     group_identifier: Principal,
// ) -> Result<(), bool> {
//     Err(false)
// }
