use candid::Principal;

use crate::models::identifier::Identifier;

pub static PROFILE_CANISTER_ID: &str = "4vy4w-gaaaa-aaaap-aa4pa-cai";
pub static MEMBER_CANISTER_ID: &str = "5nrjv-iaaaa-aaaap-aa4la-cai";
pub static GROUP_CANISTER_ID: &str = "5rvte-7aaaa-aaaap-aa4ja-cai";
pub static EVENT_CANISTER_ID: &str = "zocah-aqaaa-aaaap-aa4qa-cai";
pub static EVENT_ATTENDEE_CANISTER_ID: &str = "zaanp-3aaaa-aaaap-aa4ra-cai";
pub static REPORT_CANISTER_ID: &str = "zsg2w-xqaaa-aaaap-aa4sa-cai";

pub enum IdentifierKind {
    Profile(u64),
    Member(u64),
    Group(u64),
    Event(u64),
    EventAttendee(u64),
    Report(u64),
}

#[deprecated = "Should be deprecated in the future, this only exists for backward compatibility"]
pub fn generate_identifier(identifier: IdentifierKind) -> Identifier {
    let new_identifier = match identifier {
        IdentifierKind::Profile(id) => Identifier::new(
            id,
            Principal::from_text(PROFILE_CANISTER_ID).unwrap(),
            "pfe".to_string(),
        ),
        IdentifierKind::Member(id) => Identifier::new(
            id,
            Principal::from_text(MEMBER_CANISTER_ID).unwrap(),
            "mbr".to_string(),
        ),
        IdentifierKind::Group(id) => Identifier::new(
            id,
            Principal::from_text(GROUP_CANISTER_ID).unwrap(),
            "grp".to_string(),
        ),
        IdentifierKind::Event(id) => Identifier::new(
            id,
            Principal::from_text(EVENT_CANISTER_ID).unwrap(),
            "evt".to_string(),
        ),
        IdentifierKind::EventAttendee(id) => Identifier::new(
            id,
            Principal::from_text(EVENT_ATTENDEE_CANISTER_ID).unwrap(),
            "eae".to_string(),
        ),
        IdentifierKind::Report(id) => Identifier::new(
            id,
            Principal::from_text(REPORT_CANISTER_ID).unwrap(),
            "rpt".to_string(),
        ),
    };

    // We can unwrap here because we know that the identifier is valid
    new_identifier.unwrap()
}
