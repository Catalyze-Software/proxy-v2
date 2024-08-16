use candid::{CandidType, Principal};
use catalyze_shared::{
    api_error::ApiError,
    history_event::{GroupRoleChangeKind, GroupRoleChanged, HistoryEvent},
    CanisterResult, CellStorage, StorageClient,
};
use ic_cdk::api::call::CallResult;
use serde::Deserialize;

use crate::storage::{get_next_history_point, history_canister, profiles};

pub struct HistoryEventLogic;

impl HistoryEventLogic {
    pub async fn send(
        group_id: u64,
        principal: Principal,
        roles: Vec<String>,
        kind: GroupRoleChangeKind,
    ) -> CanisterResult<()> {
        if history_canister().is_empty() {
            return Ok(());
        }

        let (_, profile) = profiles().get(principal).await?;

        let event = GroupRoleChanged::new(group_id, principal, profile.username, roles, kind)
            .try_into()
            .map_err(|e: candid::Error| ApiError::unexpected().add_message(e.to_string()))?;

        let history_canister_id = history_canister().get()?;
        let history_point = get_next_history_point()?;

        ic_cdk::spawn(async move {
            let _ = send_event(history_canister_id, history_point, event).await;
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
    history_point: u64,
    event: HistoryEvent,
) -> CallResult<(SendHistoryEventResult,)> {
    ic_cdk::call(history_canister_id, "add_event", (history_point, event)).await
}
