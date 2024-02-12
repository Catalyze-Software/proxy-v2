use candid::Principal;

use crate::{
    models::{
        api_error::ApiError,
        member::{JoinedMemberResponse, Member},
    },
    storage::storage_api::{members, StorageMethods},
};

pub struct MemberCalls;
pub struct MemberMapper;

impl MemberCalls {
    pub fn create_empty_member(
        principal: Principal,
        profile_identifier: Principal,
    ) -> Result<(Principal, Member), ApiError> {
        if let Ok(_) = members().get(principal) {
            return Err(ApiError::duplicate()
                .add_method_name("create_empty_member")
                .add_message("Member already exists"));
        }

        let new_member = Member::new(principal, profile_identifier);
        members().insert_by_key(principal, new_member)
    }
}

impl MemberMapper {
    pub fn to_joined_member_response(
        member: Member,
        group_id: u64,
    ) -> Result<JoinedMemberResponse, ApiError> {
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::not_found()
                .add_method_name("to_joined_member_response")
                .add_message("Member not joined the group"));
        };
        Ok(JoinedMemberResponse::new(member, group_id))
    }
}
