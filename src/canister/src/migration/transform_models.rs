use super::read_stores::OldData;
use candid::Principal;

use canister_types::models::{
    attendee::Attendee, boosted::Boost, event::Event, friend_request::FriendRequest, group::Group,
    member::Member, notification::Notification, profile::Profile, report::Report,
    user_notifications::UserNotifications,
};

pub struct NewData {
    pub new_profiles: Vec<(Principal, Profile)>,
    pub new_members: Vec<(Principal, Member)>,
    pub new_attendees: Vec<(Principal, Attendee)>,

    pub new_groups: Vec<(u64, Group)>,
    pub new_events: Vec<(u64, Event)>,
    pub new_reports: Vec<(u64, Report)>,

    pub new_friend_requests: Vec<(u64, FriendRequest)>,
    pub new_boosted: Vec<(u64, Boost)>,
    // pub new_notifications: Vec<(u64, Notification)>,
    // pub new_user_notifications: Vec<(Principal, UserNotifications)>,
}

fn transform_models(old_data: OldData) -> NewData {
    let new_profiles = profiles_from_old(&old_data);
    let new_members = members_from_old(&old_data);
    let new_attendees = attendees_from_old(&old_data);

    let new_groups = groups_from_old(&old_data);
    let new_events = events_from_old(&old_data);
    let new_reports = reports_from_old(&old_data);

    let new_friend_requests = friend_requests_from_old(&old_data);
    let new_boosted = boosted_from_old(&old_data);

    // let new_notifications = notifications_from_old(&old_data);
    // let new_user_notifications = user_notifications_from_old(&old_data);

    NewData {
        new_profiles,
        new_members,
        new_attendees,
        new_groups,
        new_events,
        new_reports,
        new_friend_requests,
        new_boosted,
        // new_notifications,
        // new_user_notifications,
    }
}

fn profiles_from_old(old_data: &OldData) -> Vec<(Principal, Profile)> {
    let mut new_profiles = Vec::new();

    for (_, old_profile) in old_data.old_profiles.iter() {
        let principal: Principal = old_profile.principal;

        let profile = Profile {
            principal: old_profile.principal,
            member_identifier: old_profile.member_identifier,

            username: old_profile.username.clone(),
            display_name: old_profile.display_name.clone(),
            application_role: old_profile.application_role.clone(),
            first_name: old_profile.first_name.clone(),
            last_name: old_profile.last_name.clone(),

            privacy: old_profile.privacy.clone(),

            about: old_profile.about.clone(),
            email: old_profile.email.clone(),
            date_of_birth: old_profile.date_of_birth,
            city: old_profile.city.clone(),
            state_or_province: old_profile.state_or_province.clone(),
            country: old_profile.country.clone(),
            profile_image: old_profile.profile_image.clone(),
            banner_image: old_profile.banner_image.clone(),
            skills: old_profile.skills.clone(),
            interests: old_profile.interests.clone(),
            causes: old_profile.causes.clone(),
            website: old_profile.website.clone(),

            code_of_conduct: old_profile.code_of_conduct.clone(),
            privacy_policy: old_profile.privacy_policy.clone(),
            terms_of_service: old_profile.terms_of_service.clone(),
            wallets: old_profile.wallets.clone(),

            starred: old_profile.starred.clone(),
            relations: old_profile.relations.clone(),
            extra: old_profile.extra.clone(),
            updated_on: old_profile.updated_on,
            created_on: old_profile.created_on,
        };

        new_profiles.push((principal, profile));
    }

    new_profiles
}

fn members_from_old(old_data: &OldData) -> Vec<(Principal, Member)> {
    todo!()
}

fn attendees_from_old(old_data: &OldData) -> Vec<(Principal, Attendee)> {
    todo!()
}

fn groups_from_old(old_data: &OldData) -> Vec<(u64, Group)> {
    todo!()
}

fn events_from_old(old_data: &OldData) -> Vec<(u64, Event)> {
    todo!()
}

fn reports_from_old(old_data: &OldData) -> Vec<(u64, Report)> {
    todo!()
}

fn friend_requests_from_old(old_data: &OldData) -> Vec<(u64, FriendRequest)> {
    todo!()
}

fn boosted_from_old(old_data: &OldData) -> Vec<(u64, Boost)> {
    todo!()
}

fn notifications_from_old(old_data: &OldData) -> Vec<(u64, Notification)> {
    todo!()
}

fn user_notifications_from_old(old_data: &OldData) -> Vec<(Principal, UserNotifications)> {
    todo!()
}
