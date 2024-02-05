use candid::Principal;

use crate::{
    models::{api_error::ApiError, member::Member},
    storage::storage_api::{members, StorageMethods},
};

pub struct MemberCalls;

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
