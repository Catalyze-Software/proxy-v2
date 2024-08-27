use candid::Principal;
use catalyze_shared::{
    ic_call::ic_call, reward::RewardableActivityResponse, CanisterResult, CellStorage,
};

use super::global_canister;

#[derive(Clone)]
pub struct GlobalStorageClient;

impl GlobalStorageClient {
    pub async fn notify_group_member_count_changed(&self, group_id: u64) -> CanisterResult<()> {
        ic_call(
            global_canister().get()?,
            "notify_group_member_count_changed",
            (group_id,),
        )
        .await
    }

    pub async fn notify_active_user(&self, principal: Principal) -> CanisterResult<()> {
        ic_call(global_canister().get()?, "notify_active_user", (principal,)).await
    }

    pub async fn read_reward_buffer(&self) -> CanisterResult<Vec<RewardableActivityResponse>> {
        ic_call(global_canister().get()?, "read_reward_buffer", ()).await
    }

    pub async fn reward_timer_next_trigger(&self) -> CanisterResult<u64> {
        ic_call(global_canister().get()?, "reward_timer_next_trigger", ()).await
    }

    pub async fn get_history_point(&self) -> CanisterResult<u64> {
        ic_call(global_canister().get()?, "get_history_point", ()).await
    }

    pub async fn next_history_point(&self) -> CanisterResult<u64> {
        ic_call(global_canister().get()?, "next_history_point", ()).await
    }
}

pub fn global() -> GlobalStorageClient {
    GlobalStorageClient
}
