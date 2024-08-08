use crate::storage::{
    boosteds, profiles, AttendeeStore, EventAttendeeStore, EventStore, GroupEventsStore,
    MemberStore, StorageInsertable, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
};

use super::{
    boost_logic::BoostCalls, notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::{Boost, BoostedFilter},
    date_range::DateRange,
    event::{
        Event, EventCallerData, EventFilter, EventResponse, EventSort, EventsCount, PostEvent,
        UpdateEvent,
    },
    event_collection::EventCollection,
    invite_type::InviteType,
    member_collection::MemberCollection,
    paged_response::PagedResponse,
    privacy::PrivacyType,
    profile::{ProfileEntry, ProfileResponse},
    subject::{Subject, SubjectType},
    time_helper::hours_to_nanoseconds,
    CanisterResult, Filter, Sorter, StorageClient,
};
use ic_cdk::{api::time, caller};
use std::collections::HashMap;

pub struct EventCalls;

impl EventCalls {
    pub async fn add_event(post_event: PostEvent) -> CanisterResult<EventResponse> {
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
        GroupEventsStore::update(post_event.group_id, group_events)?;

        let boosted = Self::get_boosted_event(new_event_id).await?;

        Ok(EventResponse::new(
            new_event_id,
            new_event.clone(),
            boosted,
            Self::get_event_caller_data(new_event_id, new_event.group_id).await,
            Self::get_attendees_count(new_event_id),
        ))
    }

    pub async fn get_event(event_id: u64) -> CanisterResult<EventResponse> {
        let (_, event) = EventStore::get(event_id)?;

        if event.match_privacy(PrivacyType::InviteOnly) {
            let (_, caller_attendee) = AttendeeStore::get(caller())?;

            if !caller_attendee.is_event_joined(&event_id) {
                return Err(ApiError::unauthorized());
            }
        }

        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id).await?,
            Self::get_event_caller_data(event_id, event.group_id).await,
            Self::get_attendees_count(event_id),
        ))
    }

    pub async fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<EventFilter>,
    ) -> CanisterResult<PagedResponse<EventResponse>> {
        // get all the events and filter them based on the privacy
        // exclude all InviteOnly events that the caller is not a attendee of
        let mut events = EventStore::filter(|event_id, event| {
            if event.match_privacy(PrivacyType::InviteOnly) {
                if let Ok((_, caller_attendee)) = AttendeeStore::get(caller()) {
                    return caller_attendee.is_event_joined(event_id);
                }
                return false;
            }
            true
        })
        .into_iter()
        .collect::<HashMap<u64, Event>>();

        for filter in filters {
            for (id, event) in &events.clone() {
                if !filter.matches(id, event) {
                    events.remove(id);
                }
            }
        }

        let sorted_events = sort.sort(events.into_iter().collect());
        let profile = profiles().get(caller()).await;

        let mut result = vec![];

        for (id, event) in sorted_events {
            result.push(EventResponse::new(
                id,
                event.clone(),
                Self::get_boosted_event(id).await?,
                Self::get_event_caller_data_sync(id, event.group_id, profile.clone()),
                Self::get_attendees_count(id),
            ));
        }

        Ok(PagedResponse::new(page, limit, result))
    }

    pub async fn edit_event(
        event_id: u64,
        update_event: UpdateEvent,
        group_id: u64,
    ) -> CanisterResult<EventResponse> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.update(update_event);
        EventStore::update(event_id, event.clone())?;

        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id).await?,
            Self::get_event_caller_data(event_id, event.group_id).await,
            Self::get_attendees_count(event_id),
        ))
    }

    pub async fn get_boosted_events() -> CanisterResult<Vec<EventResponse>> {
        let ids = BoostCalls::get_boosts_by_subject(SubjectType::Event)
            .await?
            .into_iter()
            .map(|(_, boost)| *boost.subject.get_id())
            .collect();

        Self::get_events_by_id(ids).await
    }

    pub async fn get_events_count(
        group_ids: Option<Vec<u64>>,
        query: Option<String>,
    ) -> EventsCount {
        let events = match group_ids {
            Some(ids) => EventStore::filter(|_, event| {
                ids.contains(&event.group_id)
                    && match &query {
                        Some(q) => event.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    }
            }),
            None => EventStore::get_all(),
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
            .filter(|(_, event)| event.date.is_after_start_date(time()))
            .count() as u64;

        let past = events
            .iter()
            .filter(|(_, event)| event.date.is_before_start_date(time()))
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Event)
            .await
            .len() as u64;

        EventsCount {
            total: events.len() as u64,
            attending,
            invited,
            starred,
            new,
            future,
            past,
        }
    }

    pub async fn delete_event(event_id: u64, group_id: u64) -> CanisterResult<()> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let _ = EventStore::remove(event_id);

        let event_attendees =
            EventAttendeeStore::get(event_id).map_or(MemberCollection::new(), |(_, m)| m);

        if let Some((boost_id, _)) = boosteds()
            .find(BoostedFilter::Subject(Subject::Event(event_id)).into())
            .await?
        {
            boosteds().remove(boost_id).await?;
        }

        // remove all groups from the members
        let profile_list = profiles()
            .get_many(event_attendees.get_member_principals())
            .await?
            .iter_mut()
            .map(|(id, profile)| {
                let subject = Subject::Event(event_id);

                if profile.is_starred(&subject) || profile.is_pinned(&subject) {
                    profile.remove_starred(&subject);
                    profile.remove_pinned(&subject);
                }

                (*id, profile.clone())
            })
            .collect::<Vec<_>>();

        profiles().update_many(profile_list).await.unwrap();

        for member in event_attendees.get_member_principals() {
            if let Ok((principal, mut attendee)) = AttendeeStore::get(member) {
                attendee.remove_joined(group_id);
                AttendeeStore::update(principal, attendee).unwrap();
            }
        }

        // remove all invites from the members
        for member in event_attendees.get_invite_principals() {
            if let Ok((principal, mut attendee)) = AttendeeStore::get(member) {
                attendee.remove_invite(group_id);
                AttendeeStore::update(principal, attendee).unwrap();
            }
        }

        // remove attendees from the event
        EventAttendeeStore::remove(event_id);

        // remove event from group events
        let mut group_events =
            GroupEventsStore::get(group_id).map_or(EventCollection::new(), |(_, m)| m);
        group_events.remove_event(&event_id);
        GroupEventsStore::update(group_id, group_events)?;
        Ok(())
    }

    pub fn cancel_event(event_id: u64, reason: String, group_id: u64) -> CanisterResult<()> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.cancel(reason);
        EventStore::update(event_id, event.clone())?;

        Ok(())
    }

    // Attendee methods
    pub fn join_event(event_id: u64) -> CanisterResult<JoinedAttendeeResponse> {
        let (attendee_principal, mut attendee) = AttendeeStore::get(caller())?;
        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        match event.privacy {
            PrivacyType::Private => {
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
                attendees.add_invite(caller());
            }
            PrivacyType::Public => {
                NotificationCalls::notification_join_public_event(
                    vec![event.owner],
                    event.group_id,
                    event_id,
                );
                attendee.add_joined(event_id, event.group_id);
                attendees.add_member(caller());
            }
            _ => {
                return Err(ApiError::unsupported()
                    .add_message("This privacy type is not supported for events"));
            }
        }
        AttendeeStore::update(attendee_principal, attendee)?;
        EventAttendeeStore::update(event_id, attendees)?;

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
    ) -> CanisterResult<InviteAttendeeResponse> {
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
            vec![],
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

    pub fn accept_or_decline_user_request_event_invite(
        event_id: u64,
        attendee_principal: Principal,
        group_id: u64,
        accept: bool,
    ) -> CanisterResult<JoinedAttendeeResponse> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let (attendee_principal, mut attendee) = AttendeeStore::get(attendee_principal)?;

        if !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;

        if let Some(invite) = attendee.get_invite(&event_id) {
            if accept {
                attendee.turn_invite_into_joined(event_id);
                attendees.create_member_from_invite(attendee_principal);
            } else {
                attendee.remove_invite(event_id);
                attendees.remove_invite(&attendee_principal);
            }

            AttendeeStore::update(attendee_principal, attendee)?;
            EventAttendeeStore::update(event_id, attendees.clone())?;

            NotificationCalls::notification_user_join_request_event_accept_or_decline(
                attendee_principal,
                invite,
                attendees.get_member_principals(),
                accept,
            )?;
        }

        Ok(JoinedAttendeeResponse::new(
            event_id,
            group_id,
            attendee_principal,
        ))
    }

    pub fn accept_or_decline_owner_request_event_invite(
        event_id: u64,
        accept: bool,
    ) -> CanisterResult<Attendee> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;

        if !attendee.has_pending_invite(event_id) {
            return Err(
                ApiError::not_found().add_message("Attendee does not have a pending invite")
            );
        }

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;

        if let Some(invite) = attendee.get_invite(&event_id) {
            if accept {
                attendee.turn_invite_into_joined(event_id);
                attendees.create_member_from_invite(caller());
            } else {
                attendee.remove_invite(event_id);
                attendees.remove_invite(&caller());
            }

            AttendeeStore::update(caller(), attendee.clone())?;
            EventAttendeeStore::update(event_id, attendees.clone())?;

            NotificationCalls::notification_owner_join_request_event_accept_or_decline(
                caller(),
                invite,
                attendees.get_member_principals(),
                accept,
            )?;
        }
        Ok(attendee)
    }

    pub fn get_event_attendees(event_id: u64) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
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

    pub async fn get_event_attendees_profiles_and_roles(
        event_id: u64,
    ) -> CanisterResult<Vec<(ProfileResponse, Vec<String>)>> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        let profile_list = profiles()
            .get_many(event_attendees.get_member_principals())
            .await?;
        let members = MemberStore::get_many(event_attendees.get_member_principals());

        let result = profile_list
            .into_iter()
            .map(|(principal, profile)| {
                let (_, member) = members.iter().find(|(p, _)| p == &principal).unwrap();
                let roles = member.get_roles(event.group_id);
                (ProfileResponse::new(principal, profile), roles)
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    pub fn get_event_invites(
        event_id: u64,
        group_id: u64,
    ) -> CanisterResult<Vec<InviteAttendeeResponse>> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;

        let invites = AttendeeStore::get_many(event_attendees.get_invite_principals())
            .iter()
            .map(|(principal, attendee)| {
                let invite_type = attendee.invites.get(&event_id).unwrap().invite_type.clone();
                InviteAttendeeResponse::new(event_id, group_id, *principal, invite_type)
            })
            .collect();

        Ok(invites)
    }

    pub async fn get_event_invites_with_profiles(
        event_id: u64,
    ) -> CanisterResult<Vec<(ProfileResponse, InviteAttendeeResponse)>> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        let mut result: Vec<(ProfileResponse, InviteAttendeeResponse)> = vec![];

        let profiles = profiles()
            .get_many(event_attendees.get_invite_principals())
            .await?;

        for (principal, profile) in profiles {
            if let Ok((_, attendee)) = AttendeeStore::get(principal) {
                let invite = attendee.get_invite(&event_id);
                if let Some(invite) = invite {
                    result.push((
                        ProfileResponse::new(principal, profile),
                        InviteAttendeeResponse::new(
                            event_id,
                            event.group_id,
                            principal,
                            invite.invite_type,
                        ),
                    ));
                }
            }
        }

        Ok(result)
    }

    pub fn get_self_attendee() -> CanisterResult<Attendee> {
        let (_, attendee) = AttendeeStore::get(caller())?;
        Ok(attendee)
    }

    pub async fn get_self_events() -> CanisterResult<Vec<EventResponse>> {
        let (_, attendee) = AttendeeStore::get(caller())?;
        Self::get_events_by_id(attendee.get_multiple_joined().iter().map(|g| g.0).collect()).await
    }

    pub async fn get_events_by_id(event_ids: Vec<u64>) -> CanisterResult<Vec<EventResponse>> {
        let profile = profiles().get(caller()).await;
        let mut events = vec![];

        for (id, event) in EventStore::get_many(event_ids) {
            events.push(EventResponse::new(
                id,
                event.clone(),
                Self::get_boosted_event(id).await?,
                Self::get_event_caller_data_sync(id, event.group_id, profile.clone()),
                Self::get_attendees_count(id),
            ))
        }

        Ok(events)
    }

    pub fn get_attending_from_principal(
        principal: Principal,
    ) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
        let (_, attendee) = AttendeeStore::get(principal)?;
        let response = attendee
            .joined
            .into_iter()
            .map(|(event_id, join)| JoinedAttendeeResponse::new(event_id, join.group_id, principal))
            .collect();
        Ok(response)
    }

    pub fn leave_event(event_id: u64) -> CanisterResult<()> {
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

    pub fn remove_event_invite(event_id: u64) -> CanisterResult<()> {
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
    ) -> CanisterResult<()> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.is_event_joined(&event_id) {
            return Err(ApiError::not_found());
        }

        let (_, event) = EventStore::get(event_id)?;

        attendee.remove_joined(event_id);
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_member(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees.clone())?;

        NotificationCalls::notification_remove_event_attendee(
            JoinedAttendeeResponse::new(event_id, event.group_id, attendee_principal),
            attendees.get_member_principals(),
        );

        Ok(())
    }

    pub fn remove_attendee_invite_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> CanisterResult<()> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.is_event_invited(&event_id) {
            return Err(ApiError::bad_request().add_message("Attendee is not invited to the group"));
        }

        if let Some(invite) = attendee.get_invite(&event_id) {
            if let Some(notification_id) = invite.notification_id {
                NotificationCalls::notification_remove_event_invite(
                    notification_id,
                    InviteAttendeeResponse::new(
                        event_id,
                        invite.group_id,
                        attendee_principal,
                        invite.invite_type,
                    ),
                );
            }
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_invite(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    async fn get_boosted_event(id: u64) -> CanisterResult<Option<Boost>> {
        boosteds()
            .find(BoostedFilter::Subject(Subject::Event(id)).into())
            .await
            .map(|boost| boost.map(|(_, b)| b))
    }

    async fn get_event_caller_data(event_id: u64, group_id: u64) -> Option<EventCallerData> {
        let profile = profiles().get(caller()).await;
        Self::get_event_caller_data_sync(event_id, group_id, profile)
    }

    fn get_event_caller_data_sync(
        event_id: u64,
        group_id: u64,
        profile: CanisterResult<ProfileEntry>,
    ) -> Option<EventCallerData> {
        let is_starred =
            profile.is_ok_and(|(_, profile)| profile.is_starred(&Subject::Event(event_id)));

        let mut joined: Option<JoinedAttendeeResponse> = None;
        let mut invite: Option<InviteAttendeeResponse> = None;
        if let Ok((_, attendee)) = AttendeeStore::get(caller()) {
            if attendee.is_event_joined(&event_id) {
                joined = Some(JoinedAttendeeResponse::new(event_id, group_id, caller()));
            };

            if attendee.is_event_invited(&event_id) {
                invite = Some(InviteAttendeeResponse::new(
                    event_id,
                    group_id,
                    caller(),
                    // Can be safely unwrapped because we checked if the user is invited
                    attendee.get_invite(&event_id).unwrap().invite_type.clone(),
                ));
            }
        }

        Some(EventCallerData::new(joined, invite, is_starred))
    }

    pub fn get_attendees_count(event_id: u64) -> u64 {
        match EventAttendeeStore::get(event_id) {
            Ok((_, member)) => member.get_member_count(),
            Err(_) => 0,
        }
    }
}
