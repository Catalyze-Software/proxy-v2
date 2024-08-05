use candid::Principal;
use ic_cdk::query;

// canisters used in production, but are not environment dependant
pub static CATALYZE_MULTI_SIG: &str = "fcygz-gqaaa-aaaap-abpaa-cai";
pub static MULTISIG_INDEX: &str = "o7ouu-niaaa-aaaap-ahhdq-cai";

pub static E8S_PER_DAY_BOOST_COST: u64 = 3500000;
pub static USER_GROUP_CREATION_LIMIT: usize = 10;

pub mod calls;
pub mod helpers;
pub mod logic;
pub mod storage;

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;

    use catalyze_shared::api_error::*;
    use catalyze_shared::attendee::*;
    use catalyze_shared::event::*;
    use catalyze_shared::friend_request::*;
    use catalyze_shared::group::*;
    use catalyze_shared::http_types::HttpRequest;
    use catalyze_shared::log::*;
    use catalyze_shared::member::*;
    use catalyze_shared::notification::*;
    use catalyze_shared::paged_response::*;
    use catalyze_shared::permission::*;
    use catalyze_shared::profile::*;
    use catalyze_shared::relation_type::*;
    use catalyze_shared::report::*;
    use catalyze_shared::reward::*;
    use catalyze_shared::role::*;
    use catalyze_shared::subject::*;
    use catalyze_shared::topic::*;
    use catalyze_shared::transaction_data::*;
    use catalyze_shared::user_notifications::*;
    use catalyze_shared::wallet::*;
    use catalyze_shared::websocket_message::WSMessage;
    use catalyze_shared::CanisterResult;
    use ic_cdk::api::management_canister::http_request::HttpResponse;
    use ic_websocket_cdk::types::*;

    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file("./candid/proxy.did", __export_did_tmp_());
}
