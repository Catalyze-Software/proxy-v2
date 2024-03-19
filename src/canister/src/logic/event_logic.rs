use crate::storage::{AttendeeStore, EventStore, ProfileStore, StorageMethods};

use super::{boost_logic::BoostCalls, notification_logic::NotificationCalls};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::Boost,
    event::{
        Event, EventCallerData, EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent,
    },
    filter_type::FilterType,
    identifier::{Identifier, IdentifierKind},
    invite_type::InviteType,
    paged_response::PagedResponse,
    privacy::Privacy,
    subject::Subject,
};
use ic_cdk::caller;
use std::collections::HashMap;

pub struct EventCalls;

impl EventCalls {
    pub fn add_event(post_event: PostEvent) -> Result<EventResponse, ApiError> {
        let (new_event_id, new_event) = EventStore::insert(Event::from(post_event))?;

        let (_, mut attendee) = AttendeeStore::get(caller())?;

        attendee.add_joined(new_event_id, new_event.group_id);
        AttendeeStore::update(caller(), attendee)?;

        Ok(EventResponse::new(
            new_event_id,
            new_event.clone(),
            Self::get_boosted_event(new_event_id),
            Self::get_event_caller_data(new_event_id, new_event.group_id),
        ))
    }

    pub fn get_event(event_id: u64, group_id: u64) -> Result<EventResponse, ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }
        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id),
            Self::get_event_caller_data(event_id, event.group_id),
        ))
    }

    pub fn get_events_count(group_ids: Vec<u64>) -> Vec<(Principal, u64)> {
        let events = EventStore::filter(|_, event| group_ids.contains(&event.group_id));
        let mut event_count: HashMap<Principal, u64> = HashMap::new();
        for (_, event) in events {
            let count = event_count
                .entry(
                    Identifier::generate(IdentifierKind::Group(event.group_id))
                        .to_principal()
                        .unwrap(),
                )
                .or_insert(0);
            *count += 1;
        }
        event_count.into_iter().collect()
    }

    pub fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<FilterType<EventFilter>>,
    ) -> Result<PagedResponse<EventResponse>, ApiError> {
        // get all the events and filter them based on the privacy
        // exclude all InviteOnly events that the caller is not a attendee of
        let events = EventStore::filter(|event_id, event| {
            if event.match_privacy(Privacy::InviteOnly) {
                if let Ok((_, caller_attendee)) = AttendeeStore::get(caller()) {
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
            .map(|data| {
                EventResponse::new(
                    data.0,
                    data.1.clone(),
                    Self::get_boosted_event(data.0),
                    Self::get_event_caller_data(data.0, data.1.group_id),
                )
            })
            .collect();

        Ok(PagedResponse::new(page, limit, result))
    }

    pub fn edit_event(
        event_id: u64,
        update_event: UpdateEvent,
        group_id: u64,
    ) -> Result<EventResponse, ApiError> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.update(update_event);
        EventStore::update(event_id, event.clone())?;

        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id),
            Self::get_event_caller_data(event_id, event.group_id),
        ))
    }

    pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let _ = EventStore::remove(event_id);
        Ok(())
    }

    pub fn cancel_event(event_id: u64, reason: String, group_id: u64) -> Result<(), ApiError> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.cancel(reason);
        EventStore::update(event_id, event.clone())?;

        Ok(())
    }

    // Attendee methods
    // TODO: add logic for event privacy
    pub fn join_event(event_id: u64, group_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
        let (attendee_principal, mut attendee) = AttendeeStore::get(caller())?;
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        attendee.add_joined(event_id, group_id);
        let event_attendees_principals: Vec<Principal> = EventCalls::get_event_attendees(event_id)?
            .iter()
            .map(|member| member.principal)
            .collect();

        NotificationCalls::notification_join_public_event(event_attendees_principals, event_id);
        AttendeeStore::update(attendee_principal, attendee)?;

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
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let (attendee_principal, mut attendee) = AttendeeStore::get(attendee_principal)?;

        let notification_id =
            NotificationCalls::notification_owner_join_request_event(attendee_principal, event_id)?;
        attendee.add_invite(
            event_id,
            group_id,
            InviteType::OwnerRequest,
            Some(notification_id),
        );
        AttendeeStore::update(attendee_principal, attendee)?;

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
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let (attendee_principal, mut attendee) = AttendeeStore::get(attendee_principal)?;

        if !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        if let Some(invite) = attendee.get_invite(event_id) {
            attendee.turn_invite_into_joined(event_id);
            let _ = NotificationCalls::notification_user_join_request_event_accept_or_decline(
                attendee_principal,
                invite,
                true,
            );

            AttendeeStore::update(attendee_principal, attendee)?;
        }

        Ok(JoinedAttendeeResponse::new(
            event_id,
            group_id,
            attendee_principal,
        ))
    }

    pub fn accept_owner_request_event_invite(event_id: u64) -> Result<Attendee, ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;

        if !attendee.has_pending_invite(event_id) {
            return Err(ApiError::not_found());
        }

        if let Some(invite) = attendee.get_invite(event_id) {
            attendee.turn_invite_into_joined(event_id);
            let _ = NotificationCalls::notification_owner_join_request_event_accept_or_decline(
                caller(),
                invite,
                true,
            );
            AttendeeStore::update(caller(), attendee.clone())?;
        }
        Ok(attendee)
    }

    pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
        let attendees = AttendeeStore::filter(|_, attendee| attendee.is_event_joined(&event_id));

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
        let (_, attendee) = AttendeeStore::get(caller())?;
        Ok(attendee)
    }

    pub fn get_attending_from_principal(
        principal: Principal,
    ) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
        let (_, attendee) = AttendeeStore::get(principal)?;
        let response = attendee
            .joined
            .into_iter()
            .map(|(event_id, join)| JoinedAttendeeResponse::new(event_id, join.group_id, principal))
            .collect();
        Ok(response)
    }

    pub fn leave_event(event_id: u64) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;
        if !attendee.is_event_joined(&event_id) {
            return Err(ApiError::not_found());
        }

        let (_, event) = EventStore::get(event_id)?;
        if event.owner == caller() {
            return Err(ApiError::bad_request().add_message("Owner cannot leave event"));
        }

        attendee.remove_joined(event_id);
        AttendeeStore::update(caller(), attendee)?;
        Ok(())
    }

    pub fn remove_event_invite(event_id: u64) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;
        if !attendee.has_pending_invite(event_id) || !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(caller(), attendee)?;
        Ok(())
    }

    pub fn remove_attendee_from_event(
        attendee_principal: Principal,
        event_id: u64,
        group_id: u64,
    ) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.is_event_joined(&event_id) {
            return Err(ApiError::not_found());
        }

        attendee.remove_joined(group_id);
        AttendeeStore::update(attendee_principal, attendee)?;
        Ok(())
    }

    pub fn remove_attendee_invite_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.has_pending_invite(event_id) || !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(attendee_principal, attendee)?;
        Ok(())
    }

    pub fn get_event_invites(
        event_id: u64,
        group_id: u64,
    ) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
        let invites = AttendeeStore::filter(|_, attendee| {
            attendee.is_event_invited(&event_id)
                && attendee.invites.get(&event_id).unwrap().group_id == group_id
        });

        let response = invites
            .into_iter()
            .map(|(principal, attendee)| {
                let invite_type = attendee.invites.get(&event_id).unwrap().invite_type.clone();
                InviteAttendeeResponse::new(event_id, group_id, principal, invite_type)
            })
            .collect();
        Ok(response)
    }

    fn get_boosted_event(id: u64) -> Option<Boost> {
        match BoostCalls::get_boost_by_subject(Subject::Event(id)) {
            Ok((_, boosted)) => Some(boosted),
            Err(_) => None,
        }
    }

    fn get_event_caller_data(event_id: u64, group_id: u64) -> Option<EventCallerData> {
        let is_starred = ProfileStore::get(caller())
            .is_ok_and(|(_, profile)| profile.is_starred(&Subject::Event(event_id)));

        let (joined, invite) = match AttendeeStore::get(caller()) {
            Ok((principal, member)) => {
                let joined = JoinedAttendeeResponse::new(event_id, group_id, principal);
                match member.get_invite(event_id) {
                    Some(invite) => {
                        let invite = InviteAttendeeResponse::new(
                            event_id,
                            group_id,
                            principal,
                            invite.invite_type,
                        );
                        (Some(joined), Some(invite))
                    }
                    None => (Some(joined), None),
                }
            }
            Err(_) => (None, None),
        };

        Some(EventCallerData::new(joined, invite, is_starred))
    }
}
