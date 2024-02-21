use crate::{ENV, SENDER};
use models::models::{
    api_error::ApiError,
    group::{GroupResponse, PostGroup},
};
use pocket_ic::update_candid_as;

pub fn add_group(
    post_group: PostGroup,
    account_identifier: Option<String>
) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(PostGroup, Option<String>), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_group",
            (post_group, account_identifier),
        )
        .expect("Failed to call add_group from pocketIC")
        .0
        .expect("Failed to add group");

    group_response
}
