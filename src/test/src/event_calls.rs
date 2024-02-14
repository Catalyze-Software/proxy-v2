use crate::ENV;
use candid::Principal;
use models::models::{event::EventResponse, paged_response::PagedResponse};
use pocket_ic::query_candid;

#[test]
fn add_event() {
    todo!()
}

#[test]
fn get_event() {
    query_candid::<(Principal, Option<Principal>), (EventResponse,)>(
        &ENV.pic,
        ENV.canister_id,
        "get_event",
        (Principal::anonymous(), Some(Principal::anonymous())),
    )
    .expect("Failed to get event");
}

#[test]
fn get_events() {
    query_candid::<
        (
            usize,
            usize,
            String,
            Vec<String>,
            Vec<String>,
            Option<Principal>,
        ),
        (PagedResponse<EventResponse>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        "get_events",
        (
            0,
            0,
            "asc".to_string(),
            vec![],
            vec![],
            Some(Principal::anonymous()),
        ),
    )
    .expect("Failed to get events");
}
