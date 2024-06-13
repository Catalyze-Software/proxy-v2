use crate::storage::{
    reward_canister_storage::RewardCanisterStorage, CellStorage, GroupMemberStore, GroupStore,
    RewardBufferStore, RewardTimerStore, StorageQueryable, StorageUpdateable,
};

use canister_types::models::reward::{
    Activity, GroupRewardData, RewardDataPackage, UserActivityData,
};
use ic_cdk::call;

pub fn process_buffer() -> RewardDataPackage {
    let rewardables = RewardBufferStore::get_all();

    let mut group_member_counts: Vec<GroupRewardData> = Vec::new();
    let mut user_activity: Vec<UserActivityData> = Vec::new();

    for (_, rewardable) in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupMemberCount(id) => {
                // collect owner, group id and member count
                if let Ok((_, group)) = GroupStore::get(id) {
                    let (_, members) = GroupMemberStore::get(id).unwrap_or((0, Default::default()));

                    group_member_counts.push(GroupRewardData::new(
                        group.owner,
                        id,
                        members.get_member_count(),
                    ));
                }
            }
            Activity::UserActivity(principal) => {
                user_activity.push(UserActivityData::new(principal, rewardable.get_timestamp()));
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

    let _ = call::<(Vec<GroupRewardData>, Vec<UserActivityData>), ()>(
        reward_canister,
        "add_and_process_proxy_buffer",
        (reward_data.group_member_counts, reward_data.user_activity),
    )
    .await;

    // clear buffer
    RewardBufferStore::clear();
}
