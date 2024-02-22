use crate::{ENV, SENDER};
use candid::Principal;
use models::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    event::{EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent},
    filter_type::FilterType,
    paged_response::PagedResponse,
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_event(
    value: PostEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> EventResponse {
    let event_response: EventResponse = update_candid_as::<
        (PostEvent, Principal, Principal, Principal),
        (Result<EventResponse, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_event",
        (
            value,
            group_identifier,
            member_identifier,
            event_attendee_canister,
        ),
    )
    .expect("Failed to call add_event from pocket ic")
    .0
    .expect("Failed to call add_event");

    event_response
}

pub fn get_event(identifier: Principal, group_identifier: Option<Principal>) -> EventResponse {
    let event_response: EventResponse =
        query_candid_as::<(Principal, Option<Principal>), (Result<EventResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_event",
            (identifier, group_identifier),
        )
        .expect("Failed to call get_event from pocket ic")
        .0
        .expect("Failed to call get_event");

    event_response
}

// deprecated
// pub fn get_event_privacy_and_owner(
//     identifier: Principal,
//     group_identifier: Principal,
// ) -> Result<(Principal, Privacy), ApiError>

pub fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filter: Vec<EventFilter>,
    filter_type: Vec<FilterType<EventFilter>>,
    group_identifier: Option<Principal>,
) -> PagedResponse<EventResponse> {
    let paged_response: PagedResponse<EventResponse> = query_candid_as::<
        (
            usize,
            usize,
            EventSort,
            Vec<EventFilter>,
            Vec<FilterType<EventFilter>>,
            Option<Principal>,
        ),
        (Result<PagedResponse<EventResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_events",
        (limit, page, sort, filter, filter_type, group_identifier),
    )
    .expect("Failed to call get_events from pocket ic")
    .0
    .expect("Failed to call get_events");

    paged_response
}

pub fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    let events_count: Vec<(Principal, usize)> =
        query_candid_as::<(Vec<Principal>,), (Result<Vec<(Principal, usize)>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_events_count",
            (group_identifiers,),
        )
        .expect("Failed to call get_events_count from pocket ic")
        .0
        .expect("Failed to call get_events_count");

    events_count
}

pub fn edit_event(
    identifier: Principal,
    value: UpdateEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> EventResponse {
    let event_response: EventResponse = update_candid_as::<
        (Principal, UpdateEvent, Principal, Principal, Principal),
        (Result<EventResponse, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "edit_event",
        (
            identifier,
            value,
            group_identifier,
            member_identifier,
            event_attendee_canister,
        ),
    )
    .expect("Failed to call edit_event from pocket ic")
    .0
    .expect("Failed to call edit_event");

    event_response
}

pub fn delete_event(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> () {
    update_candid_as::<(Principal, Principal, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "delete_event",
        (identifier, group_identifier, member_identifier),
    )
    .expect("Failed to call delete_event from pocket ic")
    .0
    .expect("Failed to call delete_event");
}

pub fn cancel_event(
    identifier: Principal,
    reason: String,
    group_identifier: Principal,
    member_identifier: Principal,
) -> () {
    update_candid_as::<(Principal, String, Principal, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "cancel_event",
        (identifier, reason, group_identifier, member_identifier),
    )
    .expect("Failed to call cancel_event from pocket ic")
    .0
    .expect("Failed to call cancel_event");
}

// deprecated
// pub fn update_attendee_count_on_event(
//     event_identifier: Principal,
//     event_attendee_canister: Principal,
//     attendee_count: usize,
// ) -> Result<(), bool>

pub fn join_event(
    event_identifier: Principal,
    group_identifier: Principal,
) -> (Principal, Attendee) {
    update_candid_as::<(Principal, Principal), (Result<(Principal, Attendee), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "join_event",
        (event_identifier, group_identifier),
    )
    .expect("Failed to call join_event from pocket ic")
    .0
    .expect("Failed to call join_event")
}

pub fn invite_to_event(
    event_identifier: Principal,
    attendee_principal: Principal,
    member_identifier: Principal,
    group_identifier: Principal,
) -> (Principal, Attendee) {
    update_candid_as::<
        (Principal, Principal, Principal, Principal),
        (Result<(Principal, Attendee), ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "invite_to_event",
        (
            event_identifier,
            attendee_principal,
            member_identifier,
            group_identifier,
        ),
    )
    .expect("Failed to call invite_to_event from pocket ic")
    .0
    .expect("Failed to call invite_to_event")
}

pub fn accept_user_request_event_invite(
    attendee_principal: Principal,
    event_identifier: Principal,
    member_identifier: Principal,
    group_identifier: Principal,
) -> (Principal, Attendee) {
    update_candid_as::<
        (Principal, Principal, Principal, Principal),
        (Result<(Principal, Attendee), ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_user_request_event_invite",
        (
            attendee_principal,
            event_identifier,
            member_identifier,
            group_identifier,
        ),
    )
    .expect("Failed to call accept_user_request_event_invite from pocket ic")
    .0
    .expect("Failed to call accept_user_request_event_invite")
}

pub fn accept_owner_request_event_invite(event_identifier: Principal) -> (Principal, Attendee) {
    update_candid_as::<(Principal,), (Result<(Principal, Attendee), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_owner_request_event_invite",
        (event_identifier,),
    )
    .expect("Failed to call accept_owner_request_event_invite from pocket ic")
    .0
    .expect("Failed to call accept_owner_request_event_invite")
}

// deprecated
// pub fn get_event_attendees_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)>

// deprecated
// pub fn get_event_invites_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)>

pub fn get_event_attendees(
    event_identifier: Principal,
) -> Vec<JoinedAttendeeResponse> {
    query_candid_as::<(Principal,), (Result<Vec<JoinedAttendeeResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_event_attendees",
        (event_identifier,),
    )
    .expect("Failed to call get_event_attendees from pocket ic")
    .0
    .expect("Failed to call get_event_attendees")
}

pub fn get_self_events() -> (Principal, Attendee) {
    query_candid_as::<(), (Result<(Principal, Attendee), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_self_events",
        (),
    )
    .expect("Failed to call get_self_events from pocket ic")
    .0
    .expect("Failed to call get_self_events")
}

pub fn get_attending_from_principal(
    principal: Principal,
) -> Vec<JoinedAttendeeResponse> {
    query_candid_as::<(Principal,), (Result<Vec<JoinedAttendeeResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_attending_from_principal",
        (principal,),
    )
    .expect("Failed to call get_attending_from_principal from pocket ic")
    .0
    .expect("Failed to call get_attending_from_principal")
}

pub fn leave_event(event_identifier: Principal) -> () {
    update_candid_as::<(Principal,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "leave_event",
        (event_identifier,),
    )
    .expect("Failed to call leave_event from pocket ic")
    .0
    .expect("Failed to call leave_event");
}

pub fn remove_event_invite(event_identifier: Principal) -> () {
    update_candid_as::<(Principal,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_event_invite",
        (event_identifier,),
    )
    .expect("Failed to call remove_event_invite from pocket ic")
    .0
    .expect("Failed to call remove_event_invite");
}

pub fn remove_attendee_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> () {
    update_candid_as::<
        (Principal, Principal, Principal, Principal),
        (Result<(), ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_attendee_from_event",
        (attendee_principal, event_identifier, group_identifier, member_identifier),
    )
    .expect("Failed to call remove_attendee_from_event from pocket ic")
    .0
    .expect("Failed to call remove_attendee_from_event");
}

pub fn remove_attendee_invite_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> () {
    update_candid_as::<
        (Principal, Principal, Principal, Principal),
        (Result<(), ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_attendee_invite_from_event",
        (attendee_principal, event_identifier, group_identifier, member_identifier),
    )
    .expect("Failed to call remove_attendee_invite_from_event from pocket ic")
    .0
    .expect("Failed to call remove_attendee_invite_from_event");
}

pub fn get_event_invites(
    event_identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Vec<InviteAttendeeResponse> {
    query_candid_as::<
        (Principal, Principal, Principal),
        (Result<Vec<InviteAttendeeResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_event_invites",
        (event_identifier, group_identifier, member_identifier),
    )
    .expect("Failed to call get_event_invites from pocket ic")
    .0
    .expect("Failed to call get_event_invites")
}

// deprecated
// pub fn add_owner_as_attendee(
//     user_principal: Principal,
//     event_identifier: Principal,
//     group_identifier: Principal,
// ) -> Result<(), bool>
