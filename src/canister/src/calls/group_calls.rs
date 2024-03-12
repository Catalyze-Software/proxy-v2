use crate::{
    helpers::{
        group_permission::{can_delete, can_edit, can_read},
        guards::has_access,
    },
    logic::group_logic::GroupCalls,
    storage::{IdentifierRefMethods, MemberStore},
};

/// # Group methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_groups` / `get_groups_by_id` give back a more summier response per group compared to the `get_group` function?
/// And what about the public / private access of these calls?\
///
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    group::{GroupFilter, GroupResponse, GroupSort, PostGroup, UpdateGroup},
    identifier::Identifier,
    member::{InviteMemberResponse, JoinedMemberResponse, Member},
    paged_response::PagedResponse,
    permission::{PermissionType, PostPermission},
    role::Role,
};
use ic_cdk::{query, update};

/// Add a group to the canister  - [`[update]`](update)
/// # Arguments
/// * `post_group` - The group to add
/// * `account_identifier` - Optional account identifier needed in case when the group is Gated
/// # Returns
/// * `GroupResponse` - The added group
/// # Errors
/// * `ApiError` - If something went wrong while adding the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub async fn add_group(
    post_group: PostGroup,
    account_identifier: Option<String>,
) -> Result<GroupResponse, ApiError> {
    GroupCalls::add_group(post_group, account_identifier).await
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
    GroupCalls::get_group(Identifier::from(identifier).id())
}

/// Get groups - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of groups to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// # Returns
/// * `PagedResponse<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query]
pub fn get_groups(
    limit: usize,
    page: usize,
    filters: Vec<FilterType<GroupFilter>>,
    sort: GroupSort,
) -> Result<PagedResponse<GroupResponse>, ApiError> {
    GroupCalls::get_groups(limit, page, filters, sort)
}

/// Edit a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `update_group` - The group to update
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
) -> Result<GroupResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::edit_group(group_id, update_group)
}

/// Get groups by their identifiers - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The identifiers of the groups
/// # Returns
/// * `Vec<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query(guard = "has_access")]
pub fn get_groups_by_id(group_identifiers: Vec<Principal>) -> Vec<GroupResponse> {
    let group_ids = group_identifiers
        .iter()
        .map(|identifier| Identifier::from(*identifier).id())
        .collect();

    GroupCalls::get_groups_by_id(group_ids)
}

/// Soft deletes a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The deleted group
/// # Errors
/// * `ApiError` - If something went wrong while deleting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_group(group_identifier: Principal) -> Result<GroupResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_delete(group_id, PermissionType::Group(None))?;
    GroupCalls::delete_group(group_id)
}

/// Add a wallet reference to the group - [`[update]`](update)
/// # Change
/// * was `add_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// * `description` - The description of the wallet
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_wallet_to_group(
    group_identifier: Principal,
    wallet_canister: Principal,
    description: String,
) -> Result<GroupResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::add_wallet_to_group(group_id, wallet_canister, description)
}

/// Remove a wallet reference from the group - [`[update]`](update)
/// # Change
/// * was `remove_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_wallet_from_group(
    group_identifier: Principal,
    wallet_canister: Principal,
) -> Result<GroupResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::remove_wallet_from_group(group_id, wallet_canister)
}

/// Add a role to the group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// * `color` - The color of the role
/// * `index` - The index of the role
/// # Returns
/// * `Role` - The added role
/// # Errors
/// * `ApiError` - If something went wrong while adding the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// Was `add_role`
#[update(guard = "has_access")]
pub fn add_role_to_group(
    group_identifier: Principal,
    role_name: String,
    color: String,
    index: u64,
) -> Result<Role, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::add_role_to_group(group_id, role_name, color, index)
}

/// Remove a role from the group - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the member methods
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// # Returns
/// * `bool` - Whether the role was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_group_role(group_identifier: Principal, role_name: String) -> Result<bool, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::remove_group_role(group_id, role_name)
}

/// Get the roles of the group - [`[query]`](query)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Vec<Role>` - The roles of the group
/// # Note
/// Default unmutable roles are always returned on top of the custom group specific roles.
/// /// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn get_group_roles(group_identifier: Principal) -> Result<Vec<Role>, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_read(group_id, PermissionType::Group(None))?;
    Ok(GroupCalls::get_group_roles(group_id))
}

/// Edit role permissions for a group role - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group
/// * `role_name` - The name of the role
/// * `post_permissions` - The permissions to update
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
) -> Result<bool, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::edit_role_permissions(group_id, role_name, post_permissions)
}

/// Join a group - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to join
/// * `account_identifier` - Optional account identifier of the user in case the group is Gated
/// # Returns
/// * `JoinedMemberResponse` - The joined member details
/// # Errors
/// * `ApiError` - If something went wrong while joining the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub async fn join_group(
    group_identifier: Principal,
    account_identifier: Option<String>,
) -> Result<JoinedMemberResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    GroupCalls::join_group(group_id, account_identifier).await
}

/// Invite a user to a group - [`[update]`](update)
/// # Arguments
/// * `member_principal` - The principal of the user to invite
/// * `group_identifier` - The identifier of the group to invite the user to
/// # Returns
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
) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::invite_to_group(member_principal, group_id)
}

/// Accept an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `member_principal` - The principal of the user to accept the invite for
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_user_request_group_invite(
    member_principal: Principal,
    group_identifier: Principal,
) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id)
}

/// Decline an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `member_principal` - The principal of the user to accept the invite for
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn decline_user_request_group_invite(
    member_principal: Principal,
    group_identifier: Principal,
) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id)
}

/// Accept an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
#[update(guard = "has_access")]
pub fn accept_owner_request_group_invite(group_identifier: Principal) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, true)
}

/// Decline an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_identifier` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
#[update(guard = "has_access")]
pub fn decline_owner_request_group_invite(group_identifier: Principal) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, false)
}

/// Assign a role to a specific group member - [`[update]`](update)
/// # Arguments
/// * `role` - The role to assign
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while assigning the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn assign_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    match MemberStore::get_id_by_identifier(&member_identifier) {
        Some(member_principal) => {
            GroupCalls::add_group_role_to_member(role, member_principal, group_id)
        }
        None => Err(ApiError::not_found().add_message("Member not found in id - identifier map")),
    }
}

/// Remove a role from a specific group member - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the group methods
/// # Arguments
/// * `role` - The role to remove
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<Member, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Group(None))?;
    match MemberStore::get_id_by_identifier(&member_identifier) {
        Some(member_principal) => {
            GroupCalls::remove_group_role_from_member(role, member_principal, group_id)
        }
        None => Err(ApiError::not_found().add_message("Member not found in id - identifier map")),
    }
}

/// Get the member entry of a specific group member - [`[query]`](query)
/// # Arguments
/// * `member_principal` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `JoinedMemberResponse` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query(guard = "has_access")]
pub fn get_group_member(
    member_principal: Principal,
    group_identifier: Principal,
) -> Result<JoinedMemberResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_member(member_principal, group_id)
}

/// Get the groups for specific members - [`[query]`](query)
/// # Arguments
/// * `member_identifiers` - The identifiers of the members
/// # Returns
/// * `Vec<JoinedMemberResponse>` - The groups per member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_groups_for_members(member_identifiers: Vec<Principal>) -> Vec<JoinedMemberResponse> {
    let principals = member_identifiers
        .iter()
        .map(|identifier| MemberStore::get_id_by_identifier(identifier).unwrap())
        .collect();
    GroupCalls::get_groups_for_members(principals)
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
    let group_id = Identifier::from(group_identifier).id();
    can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_members(group_id)
}

/// Get the caller member entry - [`[query]`](query)
/// # Change
/// * was `get_self` but due to conflict with other methods it was renamed
/// # Returns
/// * `Member` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_self_group() -> Result<Member, ApiError> {
    GroupCalls::get_self_group()
}

/// Get the roles of a specific group member - [`[query]`](query)
/// # Arguments
/// * `member_identifier` - The identifier of the group member
/// * `group_identifier` - The identifier of the group
/// # Returns
/// * `Vec<String>` - The roles of the group member
/// # Errors
/// * `ApiError` - If something went wrong while getting the roles
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_member_roles(
    member_identifier: Principal,
    group_identifier: Principal,
) -> Result<Vec<String>, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_read(group_id, PermissionType::Group(None))?;
    match MemberStore::get_id_by_identifier(&member_identifier) {
        Some(member_principal) => Ok(GroupCalls::get_member_roles(member_principal, group_id)?),
        None => Err(ApiError::not_found().add_message("Member not found in id - identifier map")),
    }
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
    let group_id = Identifier::from(group_identifier).id();
    GroupCalls::leave_group(group_id)
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
    let group_id = Identifier::from(group_identifier).id();
    GroupCalls::remove_invite(group_id)
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
    let group_id = Identifier::from(group_identifier).id();
    can_delete(group_id, PermissionType::Member(None))?;
    GroupCalls::remove_member_from_group(principal, group_id)
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
#[update(guard = "has_access")]
pub fn remove_member_invite_from_group(
    principal: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_delete(group_id, PermissionType::Invite(None))?;
    GroupCalls::remove_member_invite_from_group(principal, group_id)
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
    let group_id = Identifier::from(group_identifier).id();
    can_read(group_id, PermissionType::Invite(None))?;
    GroupCalls::get_group_invites(group_id)
}
