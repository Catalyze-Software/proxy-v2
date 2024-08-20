use crate::{
    helpers::{
        group_permission::{can_delete, can_edit, can_read},
        guards::has_access,
    },
    logic::group_logic::GroupCalls,
};

/// # Group methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_groups` / `get_groups_by_id` give back a more summier response per group compared to the `get_group` function?
///
use candid::Principal;
use catalyze_shared::{
    group_with_members::{
        GroupFilter, GroupResponse, GroupSort, GroupsCount, PostGroup, UpdateGroup,
    },
    guards::is_not_anonymous,
    old_member::{InviteMemberResponse, JoinedMemberResponse},
    paged_response::PagedResponse,
    permission::{PermissionType, PostPermission},
    profile_with_refs::ProfileResponse,
    relation_type::RelationType,
    role::Role,
    CanisterResult,
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
#[update(guard = "is_not_anonymous")]
pub async fn add_group(
    post_group: PostGroup,
    account_identifier: Option<String>,
) -> CanisterResult<GroupResponse> {
    has_access().await?;
    GroupCalls::add_group(post_group, account_identifier).await
}

/// Get a group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The group
/// # Errors
/// * `ApiError` - If something went wrong while getting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true)]
pub async fn get_group(group_id: u64) -> CanisterResult<GroupResponse> {
    GroupCalls::get_group(group_id).await
}

/// Get a group by name - [`[query]`](query)
/// # Arguments
/// * `name` - The name of the group
/// # Returns
/// * `GroupResponse` - The group
/// # Errors
/// * `ApiError` - If something went wrong while getting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true)]
pub async fn get_group_by_name(name: String) -> CanisterResult<GroupResponse> {
    GroupCalls::get_group_by_name(name).await
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
#[query(composite = true)]
pub async fn get_groups(
    limit: usize,
    page: usize,
    filters: Vec<GroupFilter>,
    sort: GroupSort,
) -> CanisterResult<PagedResponse<GroupResponse>> {
    GroupCalls::get_groups(limit, page, filters, sort).await
}

/// Get group counts - [`[query]`](query)
/// # Returns
/// * `GroupsCount` - The groups count
#[query(composite = true)]
pub async fn get_groups_count(query: Option<String>) -> CanisterResult<GroupsCount> {
    GroupCalls::get_groups_count(query).await
}

/// Edit a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `update_group` - The group to update
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while updating the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn edit_group(group_id: u64, update_group: UpdateGroup) -> CanisterResult<GroupResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::edit_group(group_id, update_group).await
}

/// Get groups by their identifiers - [`[query]`](query)
/// # Arguments
/// * `group_ids` - The identifiers of the groups
/// # Returns
/// * `Vec<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_groups_by_id(group_ids: Vec<u64>) -> CanisterResult<Vec<GroupResponse>> {
    has_access().await?;
    GroupCalls::get_groups_by_id(group_ids).await
}

/// Soft deletes a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The deleted group
/// # Errors
/// * `ApiError` - If something went wrong while deleting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn delete_group(group_id: u64) -> CanisterResult<bool> {
    has_access().await?;
    can_delete(group_id, PermissionType::Group(None)).await?;
    GroupCalls::delete_group(group_id).await
}

/// Add a wallet reference to the group - [`[update]`](update)
/// # Change
/// * was `add_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// * `description` - The description of the wallet
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn add_wallet_to_group(
    group_id: u64,
    wallet_canister: Principal,
    description: String,
) -> CanisterResult<GroupResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::add_wallet_to_group(group_id, wallet_canister, description).await
}

/// Remove a wallet reference from the group - [`[update]`](update)
/// # Change
/// * was `remove_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_wallet_from_group(
    group_id: u64,
    wallet_canister: Principal,
) -> CanisterResult<GroupResponse> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::remove_wallet_from_group(group_id, wallet_canister).await
}

/// Add a role to the group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
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
#[update(guard = "is_not_anonymous")]
pub async fn add_role_to_group(
    group_id: u64,
    role_name: String,
    color: String,
    index: u64,
) -> CanisterResult<Role> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::add_role_to_group(group_id, role_name, color, index).await
}

/// Remove a role from the group - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the member methods
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role_name` - The name of the role
/// # Returns
/// * `bool` - Whether the role was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_group_role(group_id: u64, role_name: String) -> CanisterResult<bool> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::remove_group_role(group_id, role_name).await
}

/// Get the roles of the group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<Role>` - The roles of the group
/// # Note
/// Default unmutable roles are always returned on top of the custom group specific roles.
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn get_group_roles(group_id: u64) -> CanisterResult<Vec<Role>> {
    has_access().await?;
    can_read(group_id, PermissionType::Group(None)).await?;
    GroupCalls::get_group_roles(group_id).await
}

/// Edit role permissions for a group role - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role_name` - The name of the role
/// * `post_permissions` - The permissions to update
/// # Returns
/// * `bool` - Whether the permissions were updated
/// # Errors
/// * `ApiError` - If something went wrong while updating the permissions
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn edit_role_permissions(
    group_id: u64,
    role_name: String,
    post_permissions: Vec<PostPermission>,
) -> CanisterResult<bool> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::edit_role_permissions(group_id, role_name, post_permissions).await
}

/// Join a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to join
/// * `account_identifier` - Optional account identifier of the user in case the group is Gated
/// # Returns
/// * `JoinedMemberResponse` - The joined member details
/// # Errors
/// * `ApiError` - If something went wrong while joining the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn join_group(
    group_id: u64,
    account_identifier: Option<String>,
) -> CanisterResult<JoinedMemberResponse> {
    has_access().await?;
    GroupCalls::join_group(group_id, account_identifier).await
}

/// Invite a user to a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to invite the user to
/// * `member_principal` - The principal of the user to invite
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn invite_to_group(group_id: u64, member_principal: Principal) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::invite_to_group(member_principal, group_id).await
}

/// Accept an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// * `member_principal` - The principal of the user to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn accept_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id, true).await
}

/// Decline an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// * `member_principal` - The principal of the user to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn decline_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id, false).await
}

/// Accept an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
#[update(guard = "is_not_anonymous")]
pub async fn accept_owner_request_group_invite(group_id: u64) -> CanisterResult<()> {
    has_access().await?;
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, true).await
}

/// Decline an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
#[update(guard = "is_not_anonymous")]
pub async fn decline_owner_request_group_invite(group_id: u64) -> CanisterResult<()> {
    has_access().await?;
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, false).await
}

/// Assign a role to a specific group member - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role` - The role to assign
/// * `member_identifier` - The identifier of the group member
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while assigning the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn assign_role(
    group_id: u64,
    role: String,
    member_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::add_group_role_to_member(role, member_principal, group_id).await
}

/// Remove a role from a specific group member - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the group methods
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role` - The role to remove
/// * `member_identifier` - The identifier of the group member
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn remove_member_role(
    group_id: u64,
    role: String,
    member_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Group(None)).await?;
    GroupCalls::remove_group_role_from_member(role, member_principal, group_id).await
}

/// Get the member entry of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `JoinedMemberResponse` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query(composite = true)]
pub async fn get_group_member(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<JoinedMemberResponse> {
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_member(member_principal, group_id).await
}

/// Get the member entry and profile of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `(JoinedMemberResponse, ProfileResponse)` - The member entry and profile
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query(composite = true)]
pub async fn get_group_member_with_profile(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<(JoinedMemberResponse, ProfileResponse)> {
    GroupCalls::get_group_member_with_profile(member_principal, group_id).await
}

/// Get the group members for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<JoinedMemberResponse>` - The group members
/// # Errors
/// * `ApiError` - If something went wrong while getting the group members
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_group_members(group_id: u64) -> CanisterResult<Vec<JoinedMemberResponse>> {
    has_access().await?;
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_members(group_id).await
}

/// Get the group members for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<(JoinedMemberResponse, ProfileResponse)` - The group members with profiles
/// # Errors
/// * `ApiError` - If something went wrong while getting the group members
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_group_members_with_profiles(
    group_id: u64,
) -> CanisterResult<Vec<(JoinedMemberResponse, ProfileResponse)>> {
    has_access().await?;
    GroupCalls::get_group_members_with_profiles(group_id).await
}

/// Get the caller joined groups - [`[query]`](query)
/// # Returns
/// * `Vec<GroupResponse>` - All groups the user is part of
/// # Errors
/// * `ApiError` - If something went wrong while getting the self groups
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_self_groups() -> CanisterResult<Vec<GroupResponse>> {
    has_access().await?;
    GroupCalls::get_self_groups().await
}

/// Get the roles of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `Vec<String>` - The roles of the group member
/// # Errors
/// * `ApiError` - If something went wrong while getting the roles
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(composite = true)]
pub async fn get_member_roles(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<Vec<String>> {
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_member_roles(member_principal, group_id).await
}

/// Leave a group as a caller - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to leave
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while leaving the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn leave_group(group_id: u64) -> CanisterResult<()> {
    has_access().await?;
    GroupCalls::leave_group(group_id).await
}

/// Remove an invite for a group as a user
/// # Arguments
/// * `group_id` - The identifier of the group to remove the invite for
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_invite(group_id: u64) -> CanisterResult<()> {
    has_access().await?;
    GroupCalls::remove_invite(group_id).await
}

/// Remove a member from a group
/// # Arguments
/// * `group_id` - The identifier of the group to remove the member from
/// * `principal` - The principal of the member to remove
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "is_not_anonymous")]
pub async fn remove_member_from_group(group_id: u64, principal: Principal) -> CanisterResult<()> {
    has_access().await?;
    can_delete(group_id, PermissionType::Member(None)).await?;
    GroupCalls::remove_member_from_group(principal, group_id).await
}

/// Remove a member invite from a group as an admin
/// # Arguments
/// * `group_id` - The identifier of the group to remove the invite from
/// * `principal` - The principal of the member to remove the invite for
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "is_not_anonymous")]
pub async fn remove_member_invite_from_group(
    group_id: u64,
    principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_delete(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::remove_member_invite_from_group(principal, group_id).await
}

/// Get the group invites for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<InviteMemberResponse>` - The group invites
/// # Errors
/// * `ApiError` - If something went wrong while getting the group invites
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_group_invites(group_id: u64) -> CanisterResult<Vec<InviteMemberResponse>> {
    has_access().await?;
    can_read(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::get_group_invites(group_id).await
}

/// Get the group invites with profiles for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<(InviteMemberResponse, ProfileResponse)>` - The group invites
/// # Errors
/// * `ApiError` - If something went wrong while getting the group invites
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_group_invites_with_profiles(
    group_id: u64,
) -> CanisterResult<Vec<(InviteMemberResponse, ProfileResponse)>> {
    has_access().await?;
    can_read(group_id, PermissionType::Invite(None)).await?;
    GroupCalls::get_group_invites_with_profiles(group_id).await
}

#[query(composite = true, guard = "is_not_anonymous")]
pub async fn get_banned_group_members(group_id: u64) -> CanisterResult<Vec<Principal>> {
    has_access().await?;
    can_edit(group_id, PermissionType::Member(None)).await?;
    Ok(GroupCalls::get_banned_group_members(group_id).await)
}

#[update(guard = "is_not_anonymous")]
pub async fn ban_group_member(group_id: u64, member_principal: Principal) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Member(None)).await?;
    GroupCalls::remove_member_from_group(member_principal, group_id).await?;
    GroupCalls::add_special_member_to_group(group_id, member_principal, RelationType::Blocked).await
}

#[update(guard = "is_not_anonymous")]
pub async fn remove_ban_from_group_member(
    group_id: u64,
    member_principal: Principal,
) -> CanisterResult<()> {
    has_access().await?;
    can_edit(group_id, PermissionType::Member(None)).await?;
    GroupCalls::remove_special_member_from_group(group_id, member_principal).await
}
