use candid::Principal;
use canister_types::models::identifier::Identifier;

use super::{read_stores::OldData, transform_models::NewData};

pub fn check_data_integrity(old_data: &OldData, new_data: &NewData) {
    check_store_sizes(old_data, new_data);
    check_unique_principals(old_data);
    check_unique_ids(old_data);

    check_unique_member_joined_ids(old_data);
    chech_unique_member_invite_ids(old_data);

    check_unique_attendee_joined_ids(old_data);
    chech_unique_attendee_invite_ids(old_data);
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

fn check_unique_principals(old_data: &OldData) {
    let old_profiles_principals: Vec<Principal> = old_data
        .old_profiles
        .iter()
        .map(|(_, profile)| profile.principal)
        .collect();
    let old_members_principals: Vec<Principal> = old_data
        .old_members
        .iter()
        .map(|(_, member)| member.principal)
        .collect();
    let old_attendees_principals: Vec<Principal> = old_data
        .old_event_attendees
        .iter()
        .map(|(_, attendee)| attendee.principal)
        .collect();

    assert_eq!(
        old_profiles_principals.len(),
        old_profiles_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_members_principals.len(),
        old_members_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_attendees_principals.len(),
        old_attendees_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}

fn check_unique_ids(old_data: &OldData) {
    let old_group_identifiers: Vec<String> = old_data
        .old_groups
        .iter()
        .map(|(id, _)| id.clone())
        .collect();
    let old_event_identifiers: Vec<String> = old_data
        .old_events
        .iter()
        .map(|(id, _)| id.clone())
        .collect();

    let old_group_ids = old_group_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();

    let old_event_ids = old_event_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();

    assert_eq!(
        old_group_ids.len(),
        old_group_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_event_ids.len(),
        old_event_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}

fn check_unique_member_joined_ids(old_data: &OldData) {
    let _ = old_data.old_members.iter().map(|(_, member)| {
        let mut ids = vec![];

        for key in member.joined.keys() {
            let id = Identifier::from(*key).id();
            ids.push(id);
        }

        assert_eq!(
            ids.len(),
            ids.iter().collect::<std::collections::HashSet<_>>().len()
        );
    });
}

fn chech_unique_member_invite_ids(old_data: &OldData) {
    let _ = old_data.old_members.iter().map(|(_, member)| {
        let mut ids = vec![];

        for key in member.invites.keys() {
            let id = Identifier::from(*key).id();
            ids.push(id);
        }

        assert_eq!(
            ids.len(),
            ids.iter().collect::<std::collections::HashSet<_>>().len()
        );
    });
}

fn check_unique_attendee_joined_ids(old_data: &OldData) {
    let _ = old_data.old_event_attendees.iter().map(|(_, attendee)| {
        let mut ids = vec![];

        for key in attendee.joined.keys() {
            let id = Identifier::from(*key).id();
            ids.push(id);
        }

        assert_eq!(
            ids.len(),
            ids.iter().collect::<std::collections::HashSet<_>>().len()
        );
    });
}

fn chech_unique_attendee_invite_ids(old_data: &OldData) {
    let _ = old_data.old_event_attendees.iter().map(|(_, attendee)| {
        let mut ids = vec![];

        for key in attendee.invites.keys() {
            let id = Identifier::from(*key).id();
            ids.push(id);
        }

        assert_eq!(
            ids.len(),
            ids.iter().collect::<std::collections::HashSet<_>>().len()
        );
    });
}
