use candid::{CandidType, Principal};
use canister_types::models::{
    api_error::ApiError,
    history_event::{GroupRoleChangeKind, GroupRoleChanged, HistoryEvent, HistoryEventPayload},
};
use ic_cdk::api::call::CallResult;
use serde::Deserialize;

use crate::storage::{
    CellStorage, HistoryCanisterStorage, HistoryPointStorage, ProfileStore, StorageQueryable,
};

pub struct HistoryEventLogic;

impl HistoryEventLogic {
    pub fn send(
        group_id: u64,
        principal: Principal,
        roles: Vec<String>,
        kind: GroupRoleChangeKind,
    ) -> Result<(), ApiError> {
        if HistoryCanisterStorage::is_empty() {
            return Ok(());
        }

        let (_, profile) = ProfileStore::get(principal)?;

        let payload = HistoryEventPayload::GroupRoleChanged(GroupRoleChanged {
            group_id,
            principal,
            username: profile.username,
            roles,
            kind,
        });

        let event = HistoryEvent::new(HistoryPointStorage::get_next()?, payload);
        let history_canister_id = HistoryCanisterStorage::get()?;

        ic_cdk::spawn(async move {
            let _ = send_event(history_canister_id, event).await;
        });

        Ok(())
    }
}

#[derive(CandidType, Deserialize)]
pub enum SendHistoryEventResult {
    Ok(()),
    Err(ApiError),
}

async fn send_event(
    history_canister_id: Principal,
    event: HistoryEvent,
) -> CallResult<(SendHistoryEventResult,)> {
    ic_cdk::call(history_canister_id, "add_event", (event,)).await
}
