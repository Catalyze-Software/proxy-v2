use candid::Principal;
use catalyze_shared::{CellStorage, GenericCellStorage};

use super::{
    storage_api::{
        GLOBAL_CANISTER, HISTORY_CANISTER, PROFILE_CANISTER, REPORT_CANISTER, TOPIC_CANISTER,
    },
    BOOSTED_CANISTER, EVENT_CANISTER, GROUP_CANISTER, NOTIFICATION_CANISTER,
    TRANSACTION_HANDLER_CANISTER,
};

pub fn history_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("history_canister", &HISTORY_CANISTER)
}

pub fn profile_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("profile_canister", &PROFILE_CANISTER)
}

pub fn group_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("group_canister", &GROUP_CANISTER)
}

pub fn event_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("event_canister", &EVENT_CANISTER)
}

pub fn notification_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("notification_canister", &NOTIFICATION_CANISTER)
}

pub fn report_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("report_canister", &REPORT_CANISTER)
}

pub fn topic_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("topic_canister", &TOPIC_CANISTER)
}

pub fn global_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("global_canister", &GLOBAL_CANISTER)
}

pub fn boost_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("boost_canister", &BOOSTED_CANISTER)
}

pub fn transaction_handler_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new(
        "transaction_handler_canister",
        &TRANSACTION_HANDLER_CANISTER,
    )
}
