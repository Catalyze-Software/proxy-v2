use candid::Principal;
use catalyze_shared::{CellStorage, GenericCellStorage};

use super::storage_api::{
    GLOBAL_CANISTER, HISTORY_CANISTER, PROFILE_CANISTER, REPORT_CANISTER, TOPIC_CANISTER,
};

pub fn history_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("history_canister", &HISTORY_CANISTER)
}

pub fn profile_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("profile_canister", &PROFILE_CANISTER)
}

pub fn report_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("report_canister", &REPORT_CANISTER)
}

pub fn topic_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("topic_canister", &TOPIC_CANISTER)
}

pub fn global_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("global_canister_id", &GLOBAL_CANISTER)
}
