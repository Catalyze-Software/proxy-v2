use std::collections::HashMap;

use crate::storage::{
    EventAttendeeStore, EventStore, GroupMemberStore, GroupStore, RewardBufferStore,
    RewardTimerStore, StorageQueryable, REWARD_CANISTER_ID,
};
use candid::Principal;
use canister_types::models::reward::{Activity, RewardData, RewardDataPackage, RewardableActivity};
use ic_cdk::call;

pub fn process_buffer() -> RewardDataPackage {
    let rewardables = RewardBufferStore::get_all();

    let mut group_member_counts: Vec<RewardData> = Vec::new();
    let mut group_activity_counts: Vec<RewardData> = Vec::new();
    let mut event_attendee_counts: Vec<RewardData> = Vec::new();

    let mut group_activities: Vec<RewardableActivity> = Vec::new();
    let mut event_activities: Vec<RewardableActivity> = Vec::new();

    for (_, rewardable) in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupCount(id) => {
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
            Activity::GroupActivity(_) => {
                group_activities.push(rewardable.clone());
            }
            Activity::EventAttendance(_) => {
                event_activities.push(rewardable.clone());
            }
        }
    }

    // handle group activities
    let mut group_average_counts: HashMap<u64, u64> = HashMap::new();

    for activity in group_activities {
        // check not older than 30 days
        if activity.before(30) {
            continue;
        }

        group_average_counts
            .entry(activity.get_id())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for (group_id, count) in group_average_counts {
        if let Ok((_, group)) = GroupStore::get(group_id) {
            group_activity_counts.push(RewardData::new(group.owner, group_id, count));
        }
    }

    for rewardable_activity in event_activities.iter() {
        if let Ok((event_id, event)) = EventStore::get(rewardable_activity.get_id()) {
            let (_, attendees) =
                EventAttendeeStore::get(event_id).unwrap_or((0, Default::default()));

            event_attendee_counts.push(RewardData {
                owner: event.owner,
                id: event_id,
                count: attendees.get_member_count(),
            });
        };
    }

    RewardDataPackage {
        group_member_counts,
        group_activity_counts,
        event_attendee_counts,
    }
}

pub async fn send_reward_data() {
    RewardTimerStore::set_next_trigger();

    let reward_data = process_buffer();

    let reward_canister_principal =
        Principal::from_text(REWARD_CANISTER_ID).expect("Invalid principal");

    let _ = call::<(Vec<RewardData>, Vec<RewardData>, Vec<RewardData>), ()>(
        reward_canister_principal,
        "send_buffer",
        (
            reward_data.group_member_counts,
            reward_data.group_activity_counts,
            reward_data.event_attendee_counts,
        ),
    )
    .await;

    // clear buffer
    RewardBufferStore::clear();
}
