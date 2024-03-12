use candid::Principal;

use super::{read_stores::OldData, transform_models::NewData};

pub fn check_data_integrity(old_data: &OldData, new_data: &NewData) {
    check_store_sizes(old_data, new_data);
    check_unique_principals(new_data);
    check_unique_ids(new_data);
}

fn check_store_sizes(old_data: &OldData, new_data: &NewData) {
    assert_eq!(old_data.old_members.len(), new_data.new_members.len());
    assert_eq!(old_data.old_groups.len(), new_data.new_groups.len());
    assert_eq!(old_data.old_profiles.len(), new_data.new_profiles.len());
    assert_eq!(old_data.old_events.len(), new_data.new_events.len());
    assert_eq!(
        old_data.old_event_attendees.len(),
        new_data.new_attendees.len()
    );
}

fn check_unique_principals(new_data: &NewData) {
    let new_profiles_principals: Vec<Principal> =
        new_data.new_profiles.iter().map(|(id, _)| *id).collect();
    let new_members_principals: Vec<Principal> =
        new_data.new_members.iter().map(|(id, _)| *id).collect();
    let new_attendees_principals: Vec<Principal> =
        new_data.new_attendees.iter().map(|(id, _)| *id).collect();

    assert_eq!(
        new_profiles_principals.len(),
        new_profiles_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_members_principals.len(),
        new_members_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_attendees_principals.len(),
        new_attendees_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}

fn check_unique_ids(new_data: &NewData) {
    let new_group_ids: Vec<u64> = new_data.new_groups.iter().map(|(id, _)| *id).collect();
    let new_event_ids: Vec<u64> = new_data.new_events.iter().map(|(id, _)| *id).collect();
    let new_report_ids: Vec<u64> = new_data.new_reports.iter().map(|(id, _)| *id).collect();
    let new_friend_request_ids: Vec<u64> = new_data
        .new_friend_requests
        .iter()
        .map(|(id, _)| *id)
        .collect();
    let new_boosted_ids: Vec<u64> = new_data.new_boosted.iter().map(|(id, _)| *id).collect();

    assert_eq!(
        new_group_ids.len(),
        new_group_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_event_ids.len(),
        new_event_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_report_ids.len(),
        new_report_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_friend_request_ids.len(),
        new_friend_request_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        new_boosted_ids.len(),
        new_boosted_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}
