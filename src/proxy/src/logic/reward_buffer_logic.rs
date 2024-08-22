use crate::storage::{
    groups, reward_canister, RewardBufferStore, RewardTimerStore, StorageQueryable,
    StorageUpdateable,
};

use catalyze_shared::{
    reward::{Activity, GroupReward, RewardDataPackage, UserActivity},
    CanisterResult, CellStorage, StorageClient,
};
use ic_cdk::call;

pub async fn process_buffer() -> CanisterResult<RewardDataPackage> {
    let rewardables = RewardBufferStore::get_all();

    let mut user_activity = vec![];
    let mut group_ids = vec![];

    for (_, rewardable) in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupMemberCount(id) => {
                group_ids.push(id);
            }
            Activity::UserActivity(principal) => {
                user_activity.push(UserActivity::new(principal, rewardable.get_timestamp()));
            }
        }
    }

    let group_member_counts = groups()
        .get_many(group_ids)
        .await?
        .into_iter()
        // collect owner, group id and member count
        .map(|(id, group)| GroupReward::new(group.owner, id, group.get_members().len() as u64))
        .collect::<Vec<_>>();

    Ok(RewardDataPackage {
        group_member_counts,
        user_activity,
    })
}

pub async fn send_reward_data() -> CanisterResult<()> {
    RewardTimerStore::set_next_trigger();

    let reward_canister = reward_canister().get()?;
    let reward_data = process_buffer().await?;

    let _ = call::<(Vec<GroupReward>, Vec<UserActivity>), ()>(
        reward_canister,
        "process_buffer",
        (reward_data.group_member_counts, reward_data.user_activity),
    )
    .await;

    // clear buffer
    RewardBufferStore::clear();

    Ok(())
}
