use crate::storage::{
    GroupMemberStore, GroupStore, RewardStore, StorageQueryable, GROUP_COUNT, REWARD_CANISTER_ID,
};
use candid::Principal;
use canister_types::models::reward::{RewardData, RewardDataPackage, RewardableActivity};
use ic_cdk::{call, trap};

pub fn process_buffer() -> RewardDataPackage {
    let rewardables = RewardStore::get_all();

    let mut group_member_counts: Vec<RewardData> = Vec::new();
    let mut group_activity_counts: Vec<RewardData> = Vec::new();
    let mut event_attendee_counts: Vec<RewardData> = Vec::new();

    let mut group_activities: Vec<RewardableActivity> = Vec::new();
    let mut event_activities: Vec<RewardableActivity> = Vec::new();

    for rewardable in rewardables.iter().map(|(_, v)| v) {
        let id = rewardable.id;
        let activity = rewardable.activity.clone();

        match activity.as_str() {
            GROUP_COUNT => {
                // collect owner, group id and member count
                let owner = GroupStore::get(id).expect("Group should exist").1.owner;
                let member_count = GroupMemberStore::get(id)
                    .expect("Group should exist")
                    .1
                    .get_member_count() as u64;

                group_member_counts.push(RewardData {
                    owner,
                    id,
                    count: member_count,
                });
            }

            GROUP_ACTIVITY => {
                group_activities.push(rewardable.clone());
            }

            EVENT_ATTENDANCE => {
                event_activities.push(rewardable.clone());
            }
            _ => {
                trap(&format!("Unknown activity: {}", activity));
            }
        }
    }

    // handle group activities
    // handle event activities

    // clear buffer

    RewardDataPackage {
        group_member_counts,
        group_activity_counts,
        event_attendee_counts,
    }
}

pub async fn send_reward_data() {
    let reward_data = process_buffer();

    let group_member_counts = reward_data.group_member_counts.clone();
    let group_activity_counts = reward_data.group_activity_counts.clone();
    let event_attendee_counts = reward_data.event_attendee_counts.clone();

    let reward_canister_principal =
        Principal::from_text(REWARD_CANISTER_ID).expect("Invalid principal");

    call::<(Vec<RewardData>,), ()>(
        reward_canister_principal,
        "notify_group_counts",
        (group_member_counts,),
    )
    .await
    .expect("Failed to send group member counts");

    call::<(Vec<RewardData>,), ()>(
        reward_canister_principal,
        "notify_group_activity",
        (group_activity_counts,),
    )
    .await
    .expect("Failed to send group activity counts");

    call::<(Vec<RewardData>,), ()>(
        reward_canister_principal,
        "notify_event_attendees",
        (event_attendee_counts,),
    )
    .await
    .expect("Failed to send event attendee counts");
}
