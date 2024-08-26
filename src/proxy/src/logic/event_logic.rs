use std::collections::HashMap;

use crate::storage::{boosts, events, groups, profiles};

use super::{
    boost_logic::BoostCalls, notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    attendee::{AttendeeInvite, InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::{Boost, BoostedFilter},
    date_range::DateRange,
    event_with_attendees::{
        Attendee, EventFilter, EventResponse, EventSort, EventWithAttendees, EventsCount,
        PostEvent, UpdateEvent,
    },
    invite_type::InviteType,
    paged_response::PagedResponse,
    privacy::PrivacyType,
    profile_with_refs::ProfileResponse,
    subject::{Subject, SubjectType},
    time_helper::hours_to_nanoseconds,
    CanisterResult, StorageClient, StorageClientInsertable,
};
use ic_cdk::{api::time, caller};

pub struct EventCalls;

impl EventCalls {
    pub async fn add_event(post_event: PostEvent) -> CanisterResult<EventResponse> {
        let (new_event_id, new_event) = events()
            .insert(EventWithAttendees::from(post_event.clone()))
            .await?;

        let group_id = new_event.ensured_group_id()?;

        let (_, mut group) = groups().get(group_id).await?;
        group.add_event(new_event_id);

        groups().update(group_id, group).await?;

        let (_, mut profile) = profiles().get(caller()).await?;
        profile.add_event(new_event_id);

        profiles().update(caller(), profile).await?;

        Ok(EventResponse::new(
            new_event_id,
            new_event.clone(),
            Self::get_boosted_event(new_event_id).await?,
        ))
    }

    pub async fn get_event(event_id: u64) -> CanisterResult<EventResponse> {
        let (_, event) = events().get(event_id).await?;

        if (event.privacy.privacy_type == PrivacyType::InviteOnly) && !event.is_attendee(caller()) {
            return Err(ApiError::unauthorized());
        }

        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id).await?,
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
        let filters = vec![EventFilter::OptionallyInvited(caller())]
            .into_iter()
            .chain(filters.into_iter())
            .collect::<Vec<_>>();

        let resp = events()
            .filter_paginated(limit, page, sort, filters)
            .await?;

        let mut boosted_events = HashMap::new();

        for (id, _) in resp.data.clone() {
            let boosted_event = Self::get_boosted_event(id).await?;
            boosted_events.insert(id, boosted_event);
        }

        resp.map(|(id, event)| {
            EventResponse::new(*id, event.clone(), boosted_events.get(id).unwrap().clone())
        })
        .into_result()
    }

    pub async fn edit_event(
        id: u64,
        payload: UpdateEvent,
        group_id: u64,
    ) -> CanisterResult<EventResponse> {
        let mut event = Self::get_event_by_id_and_group(id, group_id).await?;
        let (_, event) = events().update(id, event.update(payload)).await?;
        Ok(EventResponse::new(
            id,
            event,
            Self::get_boosted_event(id).await?,
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
    ) -> CanisterResult<EventsCount> {
        let mut filters = vec![];

        if let Some(group_ids) = group_ids {
            let group_ids = group_ids.into_iter().map(Some).collect();
            filters = EventFilter::Groups(group_ids).into();
        }
        if let Some(query) = query {
            filters.push(EventFilter::Name(query));
        }

        let events = match !filters.is_empty() {
            true => events().filter(filters).await,
            false => events().get_all().await,
        }?;

        let user_id = caller();

        let (attending, invited) =
            events
                .iter()
                .fold((0, 0), |(mut attending, mut invited), (_, event)| {
                    if event.is_attendee(user_id) {
                        attending += 1;
                    }

                    if event.is_invited(user_id) {
                        invited += 1;
                    }

                    (attending, invited)
                });

        let now = time();

        let new = events
            .iter()
            .filter(|(_, event)| {
                DateRange::new(time() - hours_to_nanoseconds(24), now).is_within(event.created_on)
            })
            .count() as u64;

        let future = events
            .iter()
            .filter(|(_, event)| event.get_total_date_range().is_after_start_date(now))
            .count() as u64;

        let past = events
            .iter()
            .filter(|(_, event)| event.get_total_date_range().is_before_start_date(now))
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Event)
            .await
            .len() as u64;

        Ok(EventsCount {
            total: events.len() as u64,
            attending,
            invited,
            starred,
            new,
            future,
            past,
        })
    }

    pub async fn delete_event(id: u64, group_id: u64) -> CanisterResult<()> {
        let event = Self::get_event_by_id_and_group(id, group_id).await?;

        let _ = events().remove(id).await?;

        let boosted = boosts()
            .find(BoostedFilter::Subject(Subject::Event(id)).into())
            .await?;

        if let Some((boost_id, _)) = boosted {
            let _ = boosts().remove(boost_id).await?;
        }

        // remove all groups from the members
        let profile_list = profiles()
            .get_many(event.get_members())
            .await?
            .iter_mut()
            .map(|(profile_id, profile)| {
                let subject = Subject::Event(id);

                if profile.is_starred(&subject) || profile.is_pinned(&subject) {
                    profile.remove_starred(&subject);
                    profile.remove_pinned(&subject);
                }

                profile.remove_event(id);

                (*profile_id, profile.clone())
            })
            .collect::<Vec<_>>();

        profiles().update_many(profile_list).await?;

        let (_, mut group) = groups().get(group_id).await?;
        group.remove_event(id);

        groups().update(group_id, group).await?;

        Ok(())
    }

    pub async fn cancel_event(event_id: u64, reason: String, group_id: u64) -> CanisterResult<()> {
        let (_, mut event) = events().get(event_id).await?;

        if event.group_id != Some(group_id) {
            return Err(ApiError::unauthorized());
        }

        events().update(event_id, event.cancel(reason)).await?;

        Ok(())
    }

    // Attendee methods
    pub async fn join_event(event_id: u64) -> CanisterResult<JoinedAttendeeResponse> {
        let (_, mut event) = events().get(event_id).await?;

        let group_id = event.ensured_group_id()?;
        let member = caller();

        match event.privacy.privacy_type {
            PrivacyType::Private => {
                let invite_attendee_response = InviteAttendeeResponse::new(
                    event_id,
                    group_id,
                    member,
                    InviteType::UserRequest,
                );
                let notification_id = NotificationCalls::notification_user_join_request_event(
                    vec![event.owner],
                    invite_attendee_response,
                )
                .await?;

                event.add_invite(caller(), InviteType::UserRequest, Some(notification_id));
            }
            PrivacyType::Public => {
                NotificationCalls::notification_join_public_event(
                    vec![event.owner],
                    group_id,
                    event_id,
                )
                .await;

                event.add_attendee(member);
            }
            _ => {
                return Err(ApiError::unsupported()
                    .add_message("This privacy type is not supported for events"));
            }
        }

        events().update(event_id, event).await?;
        let (_, mut profile) = profiles().get(member).await?;

        if !profile.is_event_attendee(event_id) {
            profile.add_event(event_id);
            profiles().update(member, profile).await?;
        }

        Ok(JoinedAttendeeResponse::new(event_id, group_id, member))
    }

    pub async fn invite_to_event(
        id: u64,
        attendee_principal: Principal,
        group_id: u64,
    ) -> CanisterResult<InviteAttendeeResponse> {
        let mut event = Self::get_event_by_id_and_group(id, group_id).await?;

        let invite_attendee_response =
            InviteAttendeeResponse::new(id, group_id, attendee_principal, InviteType::OwnerRequest);

        let notification_id = NotificationCalls::notification_owner_join_request_event(
            attendee_principal,
            invite_attendee_response.clone(),
            vec![],
        )
        .await?;

        event.add_invite(
            attendee_principal,
            InviteType::OwnerRequest,
            Some(notification_id),
        );

        events().update(id, event).await?;

        let (_, mut profile) = profiles().get(attendee_principal).await?;

        if !profile.is_event_attendee(id) {
            profile.add_event(id);
            profiles().update(attendee_principal, profile).await?;
        }

        Ok(invite_attendee_response)
    }

    pub async fn accept_or_decline_user_request_event_invite(
        id: u64,
        attendee_principal: Principal,
        group_id: u64,
        accept: bool,
    ) -> CanisterResult<JoinedAttendeeResponse> {
        let event = Self::get_event_by_id_and_group(id, group_id).await?;

        if !Self::has_pending_join_request(event.clone(), attendee_principal) {
            return Err(ApiError::not_found());
        }

        Self::update_invite_status(
            id,
            event,
            attendee_principal,
            InviteType::UserRequest,
            accept,
        )
        .await?;

        Ok(JoinedAttendeeResponse::new(
            id,
            group_id,
            attendee_principal,
        ))
    }

    pub async fn accept_or_decline_owner_request_event_invite(
        id: u64,
        accept: bool,
    ) -> CanisterResult<Attendee> {
        let (_, event) = events().get(id).await?;
        let attendee_principal = caller();

        if !Self::has_pending_invite(event.clone(), attendee_principal) {
            return Err(
                ApiError::not_found().add_message("Attendee does not have a pending invite")
            );
        }

        Self::update_invite_status(
            id,
            event.clone(),
            attendee_principal,
            InviteType::OwnerRequest,
            accept,
        )
        .await?;

        let (_, profile) = profiles().get(attendee_principal).await?;

        Ok((attendee_principal, profile.get_event_ids()))
    }

    pub async fn get_event_attendees(event_id: u64) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
        let (_, event) = events().get(event_id).await?;

        let group_id = event.ensured_group_id()?;

        let response = event
            .get_members()
            .into_iter()
            .map(|principal| JoinedAttendeeResponse::new(event_id, group_id, principal))
            .collect();

        Ok(response)
    }

    pub async fn get_event_attendees_profiles_and_roles(
        event_id: u64,
    ) -> CanisterResult<Vec<(ProfileResponse, Vec<String>)>> {
        let (_, event) = events().get(event_id).await?;
        let profile_list = profiles().get_many(event.get_members()).await?;

        let result = profile_list
            .into_iter()
            .map(|(principal, profile)| {
                let member = event.attendees.members.get(&principal).unwrap();
                (
                    ProfileResponse::new(principal, profile),
                    member.roles.clone(),
                )
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    pub async fn get_event_invites(
        event_id: u64,
        group_id: u64,
    ) -> CanisterResult<Vec<InviteAttendeeResponse>> {
        let (_, event) = events().get(event_id).await?;

        let invites = event
            .attendees
            .invites
            .iter()
            .map(|(principal, invite)| {
                InviteAttendeeResponse::new(
                    event_id,
                    group_id,
                    *principal,
                    invite.invite_type.clone(),
                )
            })
            .collect();

        Ok(invites)
    }

    pub async fn get_event_invites_with_profiles(
        event_id: u64,
    ) -> CanisterResult<Vec<(ProfileResponse, InviteAttendeeResponse)>> {
        let (_, event) = events().get(event_id).await?;
        let group_id = event.ensured_group_id()?;

        let profiles = profiles()
            .get_many(event.get_invites())
            .await?
            .into_iter()
            .map(|(principal, profile)| {
                let invite = event.attendees.invites.get(&principal).unwrap();
                (
                    ProfileResponse::new(principal, profile),
                    InviteAttendeeResponse::new(
                        event_id,
                        group_id,
                        principal,
                        invite.invite_type.clone(),
                    ),
                )
            })
            .collect();

        Ok(profiles)
    }

    pub async fn get_self_attendee() -> CanisterResult<Attendee> {
        let (_, profile) = profiles().get(caller()).await?;
        Ok((caller(), profile.get_event_ids()))
    }

    pub async fn get_self_events() -> CanisterResult<Vec<EventResponse>> {
        let (_, profile) = profiles().get(caller()).await?;
        Self::get_events_by_id(profile.get_event_ids()).await
    }

    pub async fn get_events_by_id(event_ids: Vec<u64>) -> CanisterResult<Vec<EventResponse>> {
        let events = events().get_many(event_ids).await?;

        let mut boosted_events = HashMap::new();

        for (id, _) in events.clone() {
            let boosted_event = Self::get_boosted_event(id).await?;
            boosted_events.insert(id, boosted_event);
        }

        let events = events
            .iter()
            .map(|(id, event)| {
                EventResponse::new(*id, event.clone(), boosted_events.get(id).unwrap().clone())
            })
            .collect();

        Ok(events)
    }

    pub async fn get_attending_from_principal(
        principal: Principal,
    ) -> CanisterResult<Vec<JoinedAttendeeResponse>> {
        let (_, profile) = profiles().get(principal).await?;

        let mut response = vec![];

        for (id, event) in events().get_many(profile.get_event_ids()).await? {
            if !event.is_attendee(principal) {
                continue;
            }

            let group_id = event.ensured_group_id()?;
            let attendee = JoinedAttendeeResponse::new(id, group_id, principal);
            response.push(attendee);
        }

        Ok(response)
    }

    pub async fn leave_event(event_id: u64) -> CanisterResult<()> {
        let (_, mut event) = events().get(event_id).await?;
        let user_id = caller();

        if !event.is_attendee(user_id) {
            return Err(ApiError::not_found());
        }

        if event.owner == user_id {
            return Err(ApiError::bad_request().add_message("Owner cannot leave event"));
        }

        event.remove_attendee(user_id);
        events().update(event_id, event).await?;

        let (_, mut profile) = profiles().get(user_id).await?;
        profile.remove_event(event_id);
        profiles().update(user_id, profile).await?;

        Ok(())
    }

    pub async fn remove_event_invite(event_id: u64) -> CanisterResult<()> {
        let (_, mut event) = events().get(event_id).await?;

        let user_id = caller();
        let not_found = !Self::has_pending_invite(event.clone(), user_id)
            || !Self::has_pending_join_request(event.clone(), user_id);

        if not_found {
            return Err(ApiError::not_found());
        }

        event.remove_invite(user_id);
        events().update(event_id, event).await?;

        let (_, mut profile) = profiles().get(user_id).await?;
        if profile.is_event_attendee(event_id) {
            profile.remove_event(event_id);
            profiles().update(user_id, profile).await?;
        }

        Ok(())
    }

    pub async fn remove_attendee_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> CanisterResult<()> {
        let (_, mut event) = events().get(event_id).await?;
        if !event.is_attendee(attendee_principal) {
            return Err(ApiError::not_found());
        }

        event.remove_attendee(attendee_principal);
        events().update(event_id, event.clone()).await?;

        let group_id = event.ensured_group_id()?;

        let (_, mut profile) = profiles().get(attendee_principal).await?;
        if profile.is_event_attendee(event_id) {
            profile.remove_event(event_id);
            profiles().update(attendee_principal, profile).await?;
        }

        NotificationCalls::notification_remove_event_attendee(
            JoinedAttendeeResponse::new(event_id, group_id, attendee_principal),
            event.get_members(),
        )
        .await;

        Ok(())
    }

    pub async fn remove_attendee_invite_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> CanisterResult<()> {
        let (_, mut event) = events().get(event_id).await?;

        let user_id = caller();
        let not_found = !Self::has_pending_invite(event.clone(), user_id)
            || !Self::has_pending_join_request(event.clone(), user_id);

        if not_found {
            return Err(ApiError::not_found());
        }

        if let Some(invite) = event.attendees.invites.get(&attendee_principal) {
            if let Some(notification_id) = invite.notification_id {
                let group_id = event.ensured_group_id()?;

                NotificationCalls::notification_remove_event_invite(
                    notification_id,
                    InviteAttendeeResponse::new(
                        event_id,
                        group_id,
                        attendee_principal,
                        invite.invite_type.clone(),
                    ),
                )
                .await;
            }
        }

        event.remove_invite(user_id);
        events().update(event_id, event).await?;

        let (_, mut profile) = profiles().get(user_id).await?;
        if profile.is_event_attendee(event_id) {
            profile.remove_event(event_id);
            profiles().update(user_id, profile).await?;
        }

        Ok(())
    }

    async fn get_boosted_event(id: u64) -> CanisterResult<Option<Boost>> {
        let boosted = BoostCalls::get_boost_by_subject(Subject::Event(id)).await?;
        Ok(boosted.map(|(_, x)| x))
    }

    async fn get_event_by_id_and_group(
        event_id: u64,
        group_id: u64,
    ) -> CanisterResult<EventWithAttendees> {
        let (_, event) = events().get(event_id).await?;

        if event.group_id != Some(group_id) {
            return Err(ApiError::unauthorized());
        }

        Ok(event)
    }

    fn has_pending_join_request(event: EventWithAttendees, principal: Principal) -> bool {
        if let Some(invite) = event.attendees.invites.get(&principal) {
            return invite.invite_type == InviteType::UserRequest;
        }
        false
    }

    pub fn has_pending_invite(event: EventWithAttendees, principal: Principal) -> bool {
        if let Some(invite) = event.attendees.invites.get(&principal) {
            return invite.invite_type == InviteType::OwnerRequest;
        }
        false
    }

    pub async fn update_invite_status(
        id: u64,
        event: EventWithAttendees,
        attendee_principal: Principal,
        invite_type: InviteType,
        accept: bool,
    ) -> CanisterResult<()> {
        let invite = event.attendees.invites.get(&attendee_principal);

        if invite.is_none() {
            return Ok(());
        }

        let invite = invite.unwrap();
        let mut event = event.clone();

        if accept {
            event.convert_invite_to_attendee(attendee_principal);
        } else {
            event.remove_invite(attendee_principal);
            let (_, mut profile) = profiles().get(attendee_principal).await?;

            if profile.is_event_attendee(id) {
                profile.remove_event(id);
                profiles().update(attendee_principal, profile).await?;
            }
        }

        events().update(id, event.clone()).await?;

        let attendee_invite = AttendeeInvite {
            group_id: event.ensured_group_id()?,
            invite_type: invite_type.clone(),
            notification_id: invite.notification_id,
            updated_at: time(),
            created_at: invite.created_at,
        };

        match invite_type {
            InviteType::UserRequest => {
                NotificationCalls::notification_user_join_request_event_accept_or_decline(
                    attendee_principal,
                    attendee_invite,
                    event.get_members(),
                    accept,
                )
                .await?;
            }
            InviteType::OwnerRequest => {
                NotificationCalls::notification_owner_join_request_event_accept_or_decline(
                    attendee_principal,
                    attendee_invite,
                    event.get_members(),
                    accept,
                )
                .await?;
            }
        }

        Ok(())
    }
}
