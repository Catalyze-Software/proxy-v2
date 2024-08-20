use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    permission::{PermissionActionType, PermissionType},
    role::Role,
    CanisterResult, StorageClient,
};
use ic_cdk::caller;

use crate::storage::groups;

/// Determine if the caller has permission to perform an action on group based entities
/// # Arguments
/// * `group_identifier` - The group identifier
/// * `permission_type` - The permission type to check
pub async fn can_edit(group_id: u64, permission_type: PermissionType) -> CanisterResult<()> {
    has_permission(
        caller(),
        group_id,
        &permission_type,
        &PermissionActionType::Edit,
    )
    .await
}

pub async fn can_write(group_id: u64, permission_type: PermissionType) -> CanisterResult<()> {
    has_permission(
        caller(),
        group_id,
        &permission_type,
        &PermissionActionType::Write,
    )
    .await
}

pub async fn can_delete(group_id: u64, permission_type: PermissionType) -> CanisterResult<()> {
    has_permission(
        caller(),
        group_id,
        &permission_type,
        &PermissionActionType::Delete,
    )
    .await
}

pub async fn can_read(group_id: u64, permission_type: PermissionType) -> CanisterResult<()> {
    has_permission(
        caller(),
        group_id,
        &permission_type,
        &PermissionActionType::Read,
    )
    .await
}

/// Check if the caller has permission to perform an action on a group
/// This function will check the caller's roles against the group's roles
/// and return true if the caller has the permission
/// # Arguments
/// * `group_identifier` - The group identifier
/// * `permission` - The permission to check
/// * `permission_action` - The permission action to check
/// # Returns
/// * `Result<(), String>` - Returns an error if the caller does not have permission
pub async fn has_permission(
    caller: Principal,
    group_id: u64,
    permission: &PermissionType,
    permission_action: &PermissionActionType,
) -> CanisterResult<()> {
    let (_, group) = groups().get(group_id).await?;

    let member = group
        .members
        .members
        .get(&caller)
        .ok_or_else(ApiError::unauthorized)?;

    let member_roles = member.roles.clone();

    let group_roles = group.get_roles();

    let mut found_roles: Vec<&Role> = vec![];

    for member_role in member_roles {
        if let Some(_found_role) = group_roles.iter().find(|role| role.name == member_role) {
            found_roles.push(_found_role);
        }
    }

    let has_access = found_roles.iter().any(|v| {
        use PermissionActionType::*;
        v.permissions.iter().any(|p| {
            p.name() == permission.to_string()
                && match permission_action {
                    Write => p.actions().write(),
                    Read => p.actions().read(),
                    Edit => p.actions().edit(),
                    Delete => p.actions().delete(),
                }
        })
    });

    if has_access {
        return Ok(());
    }

    Err(ApiError::unauthorized())
}
