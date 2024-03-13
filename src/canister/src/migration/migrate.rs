use crate::storage::{
    AttendeeStore, EventStore, GroupStore, MemberStore, ProfileStore, StorageMethods,
};

use super::{
    read_stores::read_old_data, sanity_check::check_data_integrity,
    transform_models::transform_models,
};

#[ic_cdk::update]
async fn migrate() -> Vec<Vec<String>> {
    let old_data = read_old_data("development".to_string()).await;

    let new_data = transform_models(&old_data);

    let results = check_data_integrity(&old_data, &new_data);

    for (key, profile) in new_data.new_profiles {
        ProfileStore::insert_by_key(key, profile).expect("Failed to insert profile");
    }

    for (key, member) in new_data.new_members {
        MemberStore::insert_by_key(key, member).expect("Failed to insert member");
    }

    for (key, attendee) in new_data.new_attendees {
        AttendeeStore::insert_by_key(key, attendee).expect("Failed to insert attendee");
    }

    for (_, group) in new_data.new_groups {
        GroupStore::insert(group).expect("Failed to insert group");
    }

    for (_, event) in new_data.new_events {
        EventStore::insert(event).expect("Failed to insert event");
    }

    results
}
