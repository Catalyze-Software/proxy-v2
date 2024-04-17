use crate::{
    helpers::time_helper::hours_to_nanoseconds,
    storage::{
        AttendeeStore, EventAttendeeStore, EventStore, GroupEventsStore, ProfileStore,
        StorageMethods,
    },
};

use super::{
    boost_logic::BoostCalls, notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::Boost,
    date_range::DateRange,
    event::{
        Event, EventCallerData, EventFilter, EventResponse, EventSort, EventsCount, PostEvent,
        UpdateEvent,
    },
    event_collection::EventCollection,
    invite_type::InviteType,
    member_collection::MemberCollection,
    paged_response::PagedResponse,
    privacy::Privacy,
    subject::{Subject, SubjectType},
};
use ic_cdk::{api::time, caller};
use std::collections::HashMap;

pub struct EventCalls;

impl EventCalls {
    pub fn add_event(post_event: PostEvent) -> Result<EventResponse, ApiError> {
        let (new_event_id, new_event) = EventStore::insert(Event::from(post_event.clone()))?;

        let (_, mut attendee) = AttendeeStore::get(caller())?;

        attendee.add_joined(new_event_id, new_event.group_id);
        AttendeeStore::update(caller(), attendee)?;

        // initialize attendees with the caller
        let mut attendees = MemberCollection::new();
        attendees.add_member(caller());
        EventAttendeeStore::insert_by_key(new_event_id, attendees)?;

        // initialize group events with the new event
        let mut group_events = EventCollection::new();
        group_events.add_event(new_event_id);
        GroupEventsStore::insert_by_key(post_event.group_id, group_events)?;

        Ok(EventResponse::new(
            new_event_id,
            new_event.clone(),
            Self::get_boosted_event(new_event_id),
            Self::get_event_caller_data(new_event_id, new_event.group_id),
        ))
    }

    pub fn get_event(event_id: u64) -> Result<EventResponse, ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if event.match_privacy(Privacy::InviteOnly) {
            let (_, caller_attendee) = AttendeeStore::get(caller())?;

            if caller_attendee.is_event_joined(&event_id) {
                return Ok(EventResponse::new(
                    event_id,
                    event.clone(),
                    Self::get_boosted_event(event_id),
                    Self::get_event_caller_data(event_id, event.group_id),
                ));
            } else {
                return Err(ApiError::unauthorized());
            }
        } else {
            return Ok(EventResponse::new(
                event_id,
                event.clone(),
                Self::get_boosted_event(event_id),
                Self::get_event_caller_data(event_id, event.group_id),
            ));
        }
    }

    pub fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<EventFilter>,
    ) -> Result<PagedResponse<EventResponse>, ApiError> {
        // get all the events and filter them based on the privacy
        // exclude all InviteOnly events that the caller is not a attendee of
        let mut events = EventStore::filter(|event_id, event| {
            if event.match_privacy(Privacy::InviteOnly) {
                if let Ok((_, caller_attendee)) = AttendeeStore::get(caller()) {
                    return caller_attendee.is_event_joined(event_id);
                }
                return false;
            }
            return true;
        })
        .into_iter()
        .collect::<HashMap<u64, Event>>();

        for filter in filters {
            for (id, event) in &events.clone() {
                if !filter.is_match(id, event) {
                    events.remove(id);
                }
            }
        }

        let sorted_events = sort.sort(events);
        let result: Vec<EventResponse> = sorted_events
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

    pub fn get_events_count(group_ids: Option<Vec<u64>>, query: Option<String>) -> EventsCount {
        let events = match group_ids {
            Some(ids) => EventStore::filter(|_, event| {
                ids.contains(&event.group_id)
                    && match &query {
                        Some(q) => event.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    }
            }),
            None => EventStore::filter(|_, _| true),
        };

        let (attending, invited) = match AttendeeStore::get(caller()) {
            Ok((_, attendee)) => (attendee.joined.len() as u64, attendee.invites.len() as u64),
            Err(_) => (0, 0),
        };

        let new = events
            .iter()
            .filter(|(_, event)| {
                DateRange::new(time() - hours_to_nanoseconds(24), time())
                    .is_within(event.created_on)
            })
            .count() as u64;

        let future = events
            .iter()
            .filter(|(_, event)| event.date.is_after(time()))
            .count() as u64;

        let past = events
            .iter()
            .filter(|(_, event)| event.date.is_before(time()))
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Event).len() as u64;

        let result = EventsCount {
            total: events.len() as u64,
            attending,
            invited,
            starred,
            new,
            future,
            past,
        };

        return result;
    }

    pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let _ = EventStore::remove(event_id);

        // remove attendees from the event
        EventAttendeeStore::remove(event_id);

        // remove event from group events
        let mut group_events = EventCollection::new();
        group_events.remove_event(&event_id);
        GroupEventsStore::update(group_id, group_events)?;
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
    pub fn join_event(event_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
        let (attendee_principal, mut attendee) = AttendeeStore::get(caller())?;
        let (_, event) = EventStore::get(event_id)?;

        match event.privacy {
            Privacy::Private => {
                let invite_attendee_response = InviteAttendeeResponse::new(
                    event_id,
                    event.group_id,
                    caller(),
                    InviteType::UserRequest,
                );
                let notification_id = NotificationCalls::notification_user_join_request_event(
                    vec![event.owner],
                    invite_attendee_response,
                )?;
                attendee.add_invite(
                    event_id,
                    event.group_id,
                    InviteType::UserRequest,
                    Some(notification_id),
                );
            }
            Privacy::Public => {
                // let (_, event_attendees_principals) = EventAttendeeStore::get(event_id)?;
                NotificationCalls::notification_join_public_event(vec![event.owner], event_id);
                attendee.add_joined(event_id, event.group_id);

                AttendeeStore::update(attendee_principal, attendee)?;

                let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
                attendees.add_member(caller());
                EventAttendeeStore::update(event_id, attendees)?;
            }
            _ => {}
        }

        Ok(JoinedAttendeeResponse::new(
            event_id,
            event.group_id,
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

        let invite_attendee_response = InviteAttendeeResponse::new(
            event_id,
            group_id,
            attendee_principal,
            InviteType::OwnerRequest,
        );

        let notification_id = NotificationCalls::notification_owner_join_request_event(
            attendee_principal,
            invite_attendee_response.clone(),
        )?;

        attendee.add_invite(
            event_id,
            group_id,
            InviteType::OwnerRequest,
            Some(notification_id),
        );
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.add_invite(attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(invite_attendee_response)
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

            let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
            attendees.create_member_from_invite(attendee_principal);
            EventAttendeeStore::update(event_id, attendees)?;
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

            let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
            attendees.create_member_from_invite(caller());
            EventAttendeeStore::update(event_id, attendees)?;
        }
        Ok(attendee)
    }

    pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let attendees = AttendeeStore::get_many(event_attendees.get_member_principals());

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

    pub fn get_self_attendee() -> Result<Attendee, ApiError> {
        let (_, attendee) = AttendeeStore::get(caller())?;
        Ok(attendee)
    }

    pub fn get_self_events() -> Vec<EventResponse> {
        match AttendeeStore::get(caller()) {
            Ok((_, attendee)) => {
                let events = Self::get_events_by_id(
                    attendee.get_multiple_joined().iter().map(|g| g.0).collect(),
                );
                events
            }
            Err(_) => vec![],
        }
    }

    pub fn get_events_by_id(event_ids: Vec<u64>) -> Vec<EventResponse> {
        EventStore::get_many(event_ids)
            .into_iter()
            .map(|data| {
                EventResponse::new(
                    data.0,
                    data.1.clone(),
                    Self::get_boosted_event(data.0),
                    Self::get_event_caller_data(data.0, data.1.group_id),
                )
            })
            .collect()
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

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_member(&caller());
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    pub fn remove_event_invite(event_id: u64) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;
        if !attendee.has_pending_invite(event_id) || !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(caller(), attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_invite(&caller());
        EventAttendeeStore::update(event_id, attendees)?;

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

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_member(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;
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

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_invite(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    pub fn get_event_invites(
        event_id: u64,
        group_id: u64,
    ) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;

        let invites = AttendeeStore::get_many(event_attendees.get_invite_principals())
            .iter()
            .map(|(principal, attendee)| {
                let invite_type = attendee.invites.get(&event_id).unwrap().invite_type.clone();
                InviteAttendeeResponse::new(event_id, group_id, principal.clone(), invite_type)
            })
            .collect();

        Ok(invites)
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
