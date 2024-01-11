/// # Group methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_groups` / `get_groups_by_id` give back a more summier response per group compared to the `get_group` function?
/// And what about the public / private access of these calls?\
///
use candid::Principal;
use ic_cdk::{query, update};

use crate::{
    entities::{
        group::{GroupFilter, GroupResponse, GroupSort, PostGroup, UpdateGroup},
        member::{InviteMemberResponse, JoinedMemberResponse, Member},
    },
    helpers::guards::has_access,
    models::{
        api_error::ApiError,
        filter_type::FilterType,
        paged_response::PagedResponse,
        privacy::Privacy,
        role::{PostPermission, Role},
    },
};

/// Add a group to the canister  - [`[update]`](update)
/// # Arguments
/// * `post_group` - The group to add
/// * `member_canister` - The member canister principal to store the group owner on (icc)
/// * `account_identifier` - Optional account identifier needed in case when the group is Gated
/// # Returns
/// * `GroupResponse` - The added group
/// # Errors
/// * `ApiError` - If something went wrong while adding the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_group(
    post_group: PostGroup,
    member_canister: Principal,
    account_identifier: Option<String>,
) -> Result<GroupResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get a group - [`[query]`](query)
/// # Arguments
/// * `identifier` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The group
/// # Errors
/// * `ApiError` - If something went wrong while getting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_group(identifier: Principal) -> Result<GroupResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get groups - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of groups to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// * `filter_type` - The filter type
/// * `include_invite_only` - Whether to include invite only groups
/// # Returns
/// * `PagedResponse<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query]
pub fn get_groups(
    limit: usize,
    page: usize,
    filters: Vec<GroupFilter>,
    filter_type: FilterType,
    sort: GroupSort,
    include_invite_only: bool,
) -> Result<PagedResponse<GroupResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Edit a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `update_group` - The group to update
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while updating the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_group(
    group_identifier: Principal,
    update_group: UpdateGroup,
    member_identifier: Principal,
) -> Result<GroupResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the group owner and privacy - [`[query]`](query)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Principal` - The group owner
/// * `Privacy` - The group privacy
/// # Errors
/// * `ApiError` - If something went wrong while getting the group owner and privacy
#[query]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn get_group_owner_and_privacy(
    group_identifier: Principal,
) -> Result<(Principal, Privacy), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get groups by their identifiers - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The identifiers of the groups
/// # Returns
/// * `Vec<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query]
pub fn get_groups_by_id(group_identifiers: Vec<Principal>) -> Result<Vec<GroupResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Soft deletes a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `GroupResponse` - The deleted group
/// # Errors
/// * `ApiError` - If something went wrong while deleting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_group(
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<GroupResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Add a wallet reference to the group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// * `description` - The description of the wallet
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_wallet(
    group_identifier: Principal,
    wallet_canister: Principal,
    description: String,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove a wallet reference from the group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_wallet(
    group_identifier: Principal,
    wallet_canister: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Add a role to the group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// * `color` - The color of the role
/// * `index` - The index of the role
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `Role` - The added role
/// # Errors
/// * `ApiError` - If something went wrong while adding the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_role(
    group_identifier: Principal,
    role_name: String,
    color: String,
    index: u64,
    member_identifier: Principal,
) -> Result<Role, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove a role from the group - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the member methods
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `bool` - Whether the role was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_group_role(
    group_identifier: Principal,
    role_name: String,
    member_identifier: Principal,
) -> Result<bool, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the roles of the group - [`[query]`](query)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Vec<Role>` - The roles of the group
/// # Note
/// Default unmutable roles are always returned on top of the custom group specific roles.
#[query]
pub fn get_group_roles(group_identifier: Principal) -> Vec<Role> {
    vec![]
}

/// Edit role permissions for a group role - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// * `post_permissions` - The permissions to update
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `bool` - Whether the permissions were updated
/// # Errors
/// * `ApiError` - If something went wrong while updating the permissions
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_role_permissions(
    group_identifier: Principal,
    role_name: String,
    post_permissions: Vec<PostPermission>,
    member_identifier: Principal,
) -> Result<bool, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Update the member count on a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `member_canister` - The member canister principal to do the call to
/// * `member_count` - The member count to update
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `bool` - Whether the member count was updated
/// # Note
/// This function was triggered by an inter-canister call to update the member count on the group.
/// TODO: if used it required a auth guard so it can only be called by the known canisters
#[update]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn update_member_count(
    group_identifier: Principal,
    member_canister: Principal,
    member_count: usize,
) -> Result<(), bool> {
    Err(false)
}

// MEMBER METHODS

/// Join a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to join
/// * `account_identifier` - The account identifier of the user in case the group is Gated
/// # Returns
/// * `Principal` - The newly created member identifier
/// * `Member` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while joining the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn join_group(
    group_identifier: Principal,
    account_identifier: Option<String>,
) -> Result<(Principal, Member), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Create an empty member - [`[update]`](update)
/// # Arguments
/// * `caller` - The caller of the function passed through as an argument by the inter-canister call
/// * `profile_identifier` - The profile identifier of the user
/// # Returns
/// * `Principal` - The newly created member identifier
/// # Errors
/// * `ApiError` - If something went wrong while creating the empty member
/// # Note
/// This function was triggered by an inter-canister call to create an empty member during profile creation.
/// TODO: if used it required a auth guard so it can only be called by the known canisters
#[update]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn create_empty_member(
    caller: Principal,
    profile_identifier: Principal,
) -> Result<Principal, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Invite a user to a group - [`[update]`](update)
/// # Arguments
/// * `member_principal` - The principal of the user to invite
/// * `group_identifier` - The identifier of the group to invite the user to
/// # Returns
/// * `Principal` - The invited member identifier
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn invite_to_group(
    member_principal: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Member), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Accept an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `member_principal` - The principal of the user to accept the invite for
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Principal` - The accepted member identifier
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn accept_user_request_group_invite(
    member_principal: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Member), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Accept an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Principal` - The accepted member identifier
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
#[update]
pub fn accept_owner_request_group_invite(
    group_identifier: Principal,
) -> Result<(Principal, Member), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Add a group owner to the member entry - [`[update]`](update)
/// # Arguments
/// * `owner_principal` - The principal of the group owner
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Principal` - The updated member identifier
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while adding the group owner
/// # Note
/// This function was triggered by an inter-canister call to create an empty member during profile creation.
/// TODO: if used it required a auth guard so it can only be called by the known canisters
#[update]
#[deprecated = "This function was used as an inter-canister call, but should not be used anymore."]
pub fn add_owner(
    owner_principal: Principal,
    group_identifier: Principal,
) -> Result<Principal, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Assign a role to a specific group member - [`[update]`](update)
/// # Arguments
/// * `role` - The role to assign
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `()` - Empty tuple
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn assign_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ()> {
    Err(())
}

/// Remove a role from a specific group member - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the group methods
/// # Arguments
/// * `role` - The role to remove
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `()` - Empty tuple
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ()> {
    Err(())
}

/// Set the member roles for a specific group member - [`[update]`](update)
/// # Change
/// * was `set_roles`
/// # Arguments
/// * `roles` - The roles to set
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `()` - Empty tuple
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn set_member_roles(
    roles: Vec<String>,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ()> {
    Err(())
}

/// Get the member entry of a specific group member - [`[query]`](query)
/// # Arguments
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `JoinedMemberResponse` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query]
pub fn get_group_member(
    principal: Principal,
    group_identifier: Principal,
) -> Result<JoinedMemberResponse, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the member counts of specific groups - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The identifiers of the groups
/// # Returns
/// * `Vec<(Principal, usize)>` - `(group identifier, member count)` The member counts per group
#[query]
pub fn get_group_members_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    vec![]
}

/// Get the groups for specific members - [`[query]`](query)
/// # Arguments
/// * `member_identifiers` - The identifiers of the members
/// # Returns
/// * `Vec<(Principal, Vec<Principal>)>` - `(member identifier, group identifiers)` The groups per member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_groups_for_members(
    member_identifiers: Vec<Principal>,
) -> Vec<(Principal, Vec<Principal>)> {
    vec![]
}

/// Get the group invites count for specific groups - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The identifiers of the groups
/// # Returns
/// * `Vec<(Principal, usize)>` - `(group identifier, invite count)` The invite counts per group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_group_invites_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    vec![]
}

/// Get the group members for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Vec<JoinedMemberResponse>` - The group members
/// # Errors
/// * `ApiError` - If something went wrong while getting the group members
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_group_members(
    group_identifier: Principal,
) -> Result<Vec<JoinedMemberResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the caller member entry - [`[query]`](query)
/// # Returns
/// * `(Principal, Member)` - (member identifier, member) The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_self() -> Result<(Principal, Member), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the roles of a specific group member - [`[query]`](query)
/// # Arguments
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `(Principal, Vec<String>)` - (member identifier, roles) The roles of the group member
/// # Errors
/// * `String` - If something went wrong while getting the roles
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_member_roles(
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Vec<String>), String> {
    Err("Not implemented".to_string())
}

/// Leave a group as a caller - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to leave
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while leaving the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn leave_group(group_identifier: Principal) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove an invite for a group as a user
/// # Arguments
/// * `group_identifier` - The identifier of the group to remove the invite for
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_invite(group_identifier: Principal) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove a member from a group
/// # Arguments
/// * `principal` - The principal of the member to remove
/// * `group_identifier` - The identifier of the group to remove the member from
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_from_group(
    principal: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Remove a member invite from a group as an admin
/// # Arguments
/// * `principal` - The principal of the member to remove the invite for
/// * `group_identifier` - The identifier of the group to remove the invite from
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_invite_from_group(
    principal: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get the group invites for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Vec<InviteMemberResponse>` - The group invites
/// # Errors
/// * `ApiError` - If something went wrong while getting the group invites
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn get_group_invites(
    group_identifier: Principal,
) -> Result<Vec<InviteMemberResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}
