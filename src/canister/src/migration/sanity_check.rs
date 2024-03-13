use candid::Principal;
use canister_types::models::identifier::Identifier;

use super::{read_stores::OldData, transform_models::NewData};

pub fn check_data_integrity(old_data: &OldData, new_data: &NewData) -> Vec<Vec<String>> {
    vec![
        check_store_sizes(old_data, new_data),
        check_unique_principals(old_data),
        check_unique_and_ascending_ids(old_data),
        check_unique_member_joined_ids(old_data),
        chech_unique_member_invite_ids(old_data),
        check_unique_attendee_joined_ids(old_data),
        chech_unique_attendee_invite_ids(old_data),
        check_member_joined_and_invites_len(old_data),
        check_attendee_joined_and_invites_len(old_data),
    ]
}

fn check_store_sizes(old_data: &OldData, new_data: &NewData) -> Vec<String> {
    assert_eq!(old_data.old_members.len(), new_data.new_members.len());
    assert_eq!(old_data.old_groups.len(), new_data.new_groups.len());
    assert_eq!(old_data.old_profiles.len(), new_data.new_profiles.len());
    assert_eq!(old_data.old_events.len(), new_data.new_events.len());
    assert_eq!(
        old_data.old_event_attendees.len(),
        new_data.new_attendees.len()
    );

    vec![
        format!("Profiles store size {}", new_data.new_profiles.len()),
        format!("Members store size {}", new_data.new_members.len()),
        format!("Attendees store size {}", new_data.new_attendees.len()),
        format!("Groups store size {}", new_data.new_groups.len()),
        format!("Events store size {}", new_data.new_events.len()),
    ]
}

fn check_unique_principals(old_data: &OldData) -> Vec<String> {
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

    vec![
        format!(
            "Profiles store unique principals {}",
            old_profiles_principals.len()
        ),
        format!(
            "Members store unique principals {}",
            old_members_principals.len()
        ),
        format!(
            "Attendees store unique principals {}",
            old_attendees_principals.len()
        ),
    ]
}

fn check_unique_and_ascending_ids(old_data: &OldData) -> Vec<String> {
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

    let mut old_group_ids = old_group_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();
    old_group_ids.sort();

    let mut old_event_ids = old_event_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();
    old_event_ids.sort();

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

    assert_eq!(
        old_group_ids,
        (0..old_group_ids.len() as u64).collect::<Vec<u64>>()
    );

    assert_eq!(
        old_event_ids,
        (0..old_event_ids.len() as u64).collect::<Vec<u64>>()
    );

    vec![
        format!(
            "Groups store unique and ascending ids {}",
            old_group_ids.len()
        ),
        format!(
            "Events store unique and ascending ids {}",
            old_event_ids.len()
        ),
    ]
}

fn check_unique_member_joined_ids(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

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

        results.push(format!("Member joined ids unique {}", ids.len()));
    });

    results
}

fn chech_unique_member_invite_ids(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

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

        results.push(format!("Member invite ids unique {}", ids.len()));
    });

    results
}

fn check_unique_attendee_joined_ids(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

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

        results.push(format!("Attendee joined ids unique {}", ids.len()));
    });

    results
}

fn chech_unique_attendee_invite_ids(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

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

        results.push(format!("Attendee invite ids unique {}", ids.len()));
    });

    results
}

fn check_member_joined_and_invites_len(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

    let _ = old_data.old_members.iter().map(|(_, member)| {
        let joined_len = member.joined.len();
        let invites_len = member.invites.len();

        results.push(format!("Member joined {}", joined_len));
        results.push(format!("Member invites {}", invites_len));
    });

    results
}

fn check_attendee_joined_and_invites_len(old_data: &OldData) -> Vec<String> {
    let mut results = vec![];

    let _ = old_data.old_event_attendees.iter().map(|(_, attendee)| {
        let joined_len = attendee.joined.len();
        let invites_len = attendee.invites.len();

        results.push(format!("Attendee joined {}", joined_len));
        results.push(format!("Attendee invites {}", invites_len));
    });

    results
}
