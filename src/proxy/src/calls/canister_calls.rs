use candid::Principal;
use catalyze_shared::{CanisterResult, CellStorage};
use ic_cdk::{id, query, update};

use crate::{
    helpers::guards::is_developer,
    storage::{
        boost_canister, event_canister, group_canister, history_canister, notification_canister,
        profile_canister, report_canister, topic_canister, transaction_handler_canister,
    },
};

// History canister
#[query(guard = "is_developer")]
fn _dev_get_history_canister() -> CanisterResult<Principal> {
    history_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_history_canister(principal: Principal) -> CanisterResult<Principal> {
    history_canister().set(principal)
}

// Profile canister
#[query(guard = "is_developer")]
fn _dev_get_profile_canister() -> CanisterResult<Principal> {
    profile_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_profile_canister(principal: Principal) -> CanisterResult<Principal> {
    profile_canister().set(principal)
}

// Event canister
#[query(guard = "is_developer")]
fn _dev_get_event_canister() -> CanisterResult<Principal> {
    event_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_event_canister(principal: Principal) -> CanisterResult<Principal> {
    event_canister().set(principal)
}

// Group canister
#[query(guard = "is_developer")]
fn _dev_get_group_canister() -> CanisterResult<Principal> {
    group_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_group_canister(principal: Principal) -> CanisterResult<Principal> {
    group_canister().set(principal)
}

// Notification canister
#[query(guard = "is_developer")]
fn _dev_get_notification_canister() -> CanisterResult<Principal> {
    notification_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_notification_canister(principal: Principal) -> CanisterResult<Principal> {
    notification_canister().set(principal)
}

// Report canister
#[query(guard = "is_developer")]
fn _dev_get_report_canister() -> CanisterResult<Principal> {
    report_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_report_canister(principal: Principal) -> CanisterResult<Principal> {
    report_canister().set(principal)
}

// Transaction handler canister
#[query(guard = "is_developer")]
fn _dev_get_transaction_handler_canister() -> CanisterResult<Principal> {
    transaction_handler_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_transaction_handler_canister(principal: Principal) -> CanisterResult<Principal> {
    transaction_handler_canister().set(principal)
}

// Boost canister
#[query(guard = "is_developer")]
fn _dev_get_boost_canister() -> CanisterResult<Principal> {
    boost_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_boost_canister(principal: Principal) -> CanisterResult<Principal> {
    boost_canister().set(principal)
}

// Boost canister
#[query(guard = "is_developer")]
fn _dev_get_topic_canister() -> CanisterResult<Principal> {
    topic_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_topic_canister(principal: Principal) -> CanisterResult<Principal> {
    topic_canister().set(principal)
}

#[update(guard = "is_developer")]
pub fn _dev_canisters_init() {
    // not enviroment specific
    let _ = transaction_handler_canister()
        .set(Principal::from_text("r4bli7-7iaaa-aaaap-ahd4a-cai").unwrap());

    // development
    if id().to_string().as_str() == "puwkw-6qaaa-aaaap-ahmvq-cai" {
        let _ =
            profile_canister().set(Principal::from_text("qj423-uyaaa-aaaap-aho4a-cai").unwrap());
        let _ = notification_canister()
            .set(Principal::from_text("xz5up-6qaaa-aaaap-ahwxq-cai").unwrap());
        let _ =
            history_canister().set(Principal::from_text("vkcxp-wiaaa-aaaap-ahwza-cai").unwrap());
        let _ = report_canister().set(Principal::from_text("iha5a-maaaa-aaaap-ahtda-cai").unwrap());
        let _ = group_canister().set(Principal::from_text("ivgkz-aqaaa-aaaap-ahtaa-cai").unwrap());
        let _ = event_canister().set(Principal::from_text("i4fbf-wyaaa-aaaap-ahtbq-cai").unwrap());
        let _ = topic_canister().set(Principal::from_text("v7fgc-xaaaa-aaaap-ahw2q-cai").unwrap());
        let _ = boost_canister().set(Principal::from_text("vyeaw-2yaaa-aaaap-ahw2a-cai").unwrap());
    }

    // // staging
    // if id().to_string().as_str() == "unset" {}

    // // production
    // if id().to_string().as_str() == "unset" {}
}
