use std::collections::HashMap;

use candid::Principal;
use ic_cdk::caller;

use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    event::{Event, EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent},
    filter_type::FilterType,
    invite::InviteType,
    paged_response::PagedResponse,
    privacy::Privacy,
};

use crate::storage::storage_api::{attendees, events, IdentifierRefMethods, StorageMethods};

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

pub fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filters: Vec<FilterType<EventFilter>>,
) -> Result<PagedResponse<EventResponse>, ApiError> {
    // get all the events and filter them based on the privacy
    // exclude all InviteOnly events that the caller is not a attendee of
    let events = events().filter(|event_id, event| {
        if event.match_privacy(Privacy::InviteOnly) {
            if let Ok((_, caller_attendee)) = attendees().get(caller()) {
                return caller_attendee.is_event_joined(event_id);
            }
            return false;
        }
        return true;
    });

    // split the filters into or and and filters
    let mut or_filters: Vec<EventFilter> = vec![];
    let mut and_filters: Vec<EventFilter> = vec![];
    for filter_type in filters {
        use FilterType::*;
        match filter_type {
            And(filter_value) => and_filters.push(filter_value),
            Or(filter_value) => or_filters.push(filter_value),
        }
    }

    // filter the events based on the `OR` filters
    let mut or_filtered_events: HashMap<u64, Event> = HashMap::new();
    for filter in or_filters {
        for (id, event) in &events {
            if filter.is_match(&id, &event) {
                or_filtered_events.insert(id.clone(), event.clone());
            }
        }
    }

    // filter the `or_filtered` groups based on the `AND` filters
    let mut and_filtered_groups: HashMap<u64, Event> = HashMap::new();
    for filter in and_filters {
        for (id, group) in &or_filtered_events {
            if filter.is_match(&id, &group) {
                and_filtered_groups.insert(id.clone(), group.clone());
            }
        }
    }

    let sorted_groups = sort.sort(and_filtered_groups);
    let result: Vec<EventResponse> = sorted_groups
        .into_iter()
        .map(|data| EventResponse::new(data.0, data.1))
        .collect();

    Ok(PagedResponse::new(page, limit, result))
}

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

pub fn accept_owner_request_event_invite(event_id: u64) -> Result<Attendee, ApiError> {
    let (_, mut attendee) = attendees().get(caller())?;
    let (_, event) = events().get(event_id)?;

    if !attendee.has_pending_invite(event_id) {
        return Err(ApiError::not_found());
    }

    attendee.add_joined(event_id, event_id);
    attendees().update(caller(), attendee.clone())?;
    Ok(attendee)
}

pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    let attendees = attendees().filter(|principal, attendee| attendee.is_event_joined(&event_id));

    let response = attendees
        .into_iter()
        .map(|(principal, attendee)| {
            JoinedAttendeeResponse::new(
                event_id,
                attendee.joined.get(&event_id).unwrap().group_id,
                principal,
            )
        })
        .collect();
    Ok(response)
}

pub fn get_self_events() -> Result<Attendee, ApiError> {
    let (_, attendee) = attendees().get(caller())?;
    Ok(attendee)
}

pub fn get_attending_from_principal(
    principal: Principal,
) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    let (_, attendee) = attendees().get(principal)?;
    let response = attendee
        .joined
        .into_iter()
        .map(|(event_id, join)| JoinedAttendeeResponse::new(event_id, join.group_id, principal))
        .collect();
    Ok(response)
}

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
