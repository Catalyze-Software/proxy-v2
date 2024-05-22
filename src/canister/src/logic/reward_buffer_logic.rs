use std::collections::HashMap;

use crate::storage::{
    EventAttendeeStore, EventStore, GroupMemberStore, GroupStore, RewardStore, StorageQueryable,
    EVENT_ATTENDANCE, GROUP_ACTIVITY, GROUP_COUNT, REWARD_CANISTER_ID,
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
    let mut group_average_counts: HashMap<u64, u64> = HashMap::new();
    let last_month = ic_cdk::api::time() - 30 * 24 * 60 * 60;

    for activity in group_activities {
        // check not older than 30 days
        if activity.timestamp < last_month {
            continue;
        }
        group_average_counts
            .entry(activity.id)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for (group_id, count) in group_average_counts {
        group_activity_counts.push(RewardData {
            owner: GroupStore::get(group_id)
                .expect("Group should exist")
                .1
                .owner,
            id: group_id,
            count,
        });
    }

    // handle event activities
    event_activities
        .into_iter()
        .for_each(|rewardable_activity| {
            let event_id = rewardable_activity.id;
            let owner = EventStore::get(event_id)
                .expect("Event should exist")
                .1
                .owner;
            let attendees = EventAttendeeStore::get(event_id)
                .expect("Event should exist")
                .1
                .get_member_count() as u64;

            event_attendee_counts.push(RewardData {
                owner,
                id: event_id,
                count: attendees,
            });
        });

    // clear buffer
    RewardStore::clear();

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
