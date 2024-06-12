use crate::storage::{
    reward_canister_storage::RewardCanisterStorage, CellStorage, GroupMemberStore, GroupStore,
    RewardBufferStore, RewardTimerStore, StorageQueryable, StorageUpdateable,
};

use canister_types::models::reward::{Activity, RewardData, RewardDataPackage};
use ic_cdk::call;

pub fn process_buffer() -> RewardDataPackage {
    let rewardables = RewardBufferStore::get_all();

    let mut group_member_counts: Vec<RewardData> = Vec::new();
    let mut user_activity: Vec<RewardData> = Vec::new();

    for (_, rewardable) in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupMemberCount(id) => {
                // collect owner, group id and member count
                if let Ok((_, group)) = GroupStore::get(id) {
                    let (_, members) = GroupMemberStore::get(id).unwrap_or((0, Default::default()));

                    group_member_counts.push(RewardData::new(
                        group.owner,
                        id,
                        members.get_member_count(),
                    ));
                }
            }
            Activity::UserActivity(principal) => {
                user_activity.push(RewardData::new(principal, 0, 1));
            }
        }
    }

    RewardDataPackage {
        group_member_counts,
        user_activity,
    }
}

pub async fn send_reward_data() {
    RewardTimerStore::set_next_trigger();

    let reward_canister = RewardCanisterStorage::get().expect("Reward canister not set");

    let reward_data = process_buffer();

    let _ = call::<(Vec<RewardData>, Vec<RewardData>), ()>(
        reward_canister,
        "add_and_process_proxy_buffer",
        (reward_data.group_member_counts, reward_data.user_activity),
    )
    .await;

    // clear buffer
    RewardBufferStore::clear();
}
