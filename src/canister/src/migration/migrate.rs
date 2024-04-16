use std::collections::HashMap;

use canister_types::models::{
    event_collection::EventCollection, member_collection::MemberCollection,
};

use crate::storage::{
    AttendeeStore, EventAttendeeStore, EventStore, GroupEventsStore, GroupMemberStore, GroupStore,
    MemberStore, ProfileStore, StorageMethods,
};

use super::{
    read_stores::read_old_data, sanity_check::check_data_integrity,
    transform_models::transform_models,
};

#[ic_cdk::update]
async fn migrate() -> Vec<Vec<String>> {
    let old_data = read_old_data("staging".to_string()).await;

    let new_data = transform_models(&old_data);

    let results = check_data_integrity(&old_data, &new_data);

    for (key, profile) in new_data.new_profiles {
        ProfileStore::insert_by_key(key, profile).expect("Failed to insert profile");
    }

    for (group_id, group) in new_data.new_groups {
        GroupStore::insert(group).expect("Failed to insert group");
        GroupMemberStore::insert_by_key(group_id, MemberCollection::new())
            .expect("Failed to insert group members");
        GroupEventsStore::insert_by_key(group_id, EventCollection::new())
            .expect("Failed to insert group events");
    }

    // EVENTS
    let mut event_collections: HashMap<u64, EventCollection> = HashMap::new();

    for (event_id, event) in new_data.new_events {
        EventStore::insert(event.clone()).expect("Failed to insert event");
        EventAttendeeStore::insert_by_key(event_id, MemberCollection::new())
            .expect("Failed to insert event attendees");

        match event_collections.get(&event.group_id) {
            Some(collection) => {
                let mut new_collection = collection.clone();
                new_collection.add_event(event_id);
                event_collections.insert(event.group_id, new_collection);
            }
            None => {
                let mut new_collection = EventCollection::new();
                new_collection.add_event(event_id);
                event_collections.insert(event.group_id, new_collection);
            }
        }
    }

    for (group_id, collection) in event_collections {
        GroupEventsStore::update(group_id, collection).expect("Failed to insert group events");
    }

    // MEMBERS
    let mut member_collections: HashMap<u64, MemberCollection> = HashMap::new();

    for (key, member) in new_data.new_members {
        MemberStore::insert_by_key(key, member.clone()).expect("Failed to insert member");

        for (group_id, _) in member.joined {
            match member_collections.get(&group_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_member(key);
                    member_collections.insert(group_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_member(key);
                    member_collections.insert(group_id, new_collection);
                }
            }
        }

        for (group_id, _) in member.invites {
            match member_collections.get(&group_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_invite(key);
                    member_collections.insert(group_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_invite(key);
                    member_collections.insert(group_id, new_collection);
                }
            }
        }
    }

    for (group_id, collection) in member_collections {
        GroupMemberStore::update(group_id, collection).expect("Failed to insert group members");
    }

    // ATTENDEES
    let mut attendee_collections: HashMap<u64, MemberCollection> = HashMap::new();
    for (key, attendee) in new_data.new_attendees {
        AttendeeStore::insert_by_key(key, attendee.clone()).expect("Failed to insert attendee");

        for (event_id, _) in attendee.joined {
            match attendee_collections.get(&event_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_member(key);
                    attendee_collections.insert(event_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_member(key);
                    attendee_collections.insert(event_id, new_collection);
                }
            }
        }

        for (event_id, _) in attendee.invites {
            match attendee_collections.get(&event_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_invite(key);
                    attendee_collections.insert(event_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_invite(key);
                    attendee_collections.insert(event_id, new_collection);
                }
            }
        }
    }

    for (event_id, collection) in attendee_collections {
        EventAttendeeStore::update(event_id, collection).expect("Failed to insert event attendees");
    }

    results
}
