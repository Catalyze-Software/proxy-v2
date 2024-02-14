use ic_cdk::caller;

use crate::storage::storage_api::{groups, members, StorageMethods};
use canister_types::models::{
    api_error::ApiError,
    permission::{PermissionActionType, PermissionType},
    role::Role,
};

/// Determine if the caller has permission to perform an action on group based entities
/// # Arguments
/// * `group_identifier` - The group identifier
/// * `permission_type` - The permission type to check
pub fn can_edit(group_id: u64, permission_type: PermissionType) -> Result<(), ApiError> {
    has_permission(group_id, &permission_type, &PermissionActionType::Edit)
}

pub fn can_write(group_id: u64, permission_type: PermissionType) -> Result<(), ApiError> {
    has_permission(group_id, &permission_type, &PermissionActionType::Write)
}

pub fn can_delete(group_id: u64, permission_type: PermissionType) -> Result<(), ApiError> {
    has_permission(group_id, &permission_type, &PermissionActionType::Delete)
}

pub fn can_read(group_id: u64, permission_type: PermissionType) -> Result<(), ApiError> {
    has_permission(group_id, &permission_type, &PermissionActionType::Read)
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
fn has_permission(
    group_id: u64,
    permission: &PermissionType,
    permission_action: &PermissionActionType,
) -> Result<(), ApiError> {
    let member_roles = members().get(caller())?.1.get_roles(group_id);

    let group_roles = groups().get(group_id)?.1.get_roles();

    let mut found_roles: Vec<&Role> = vec![];

    for member_role in member_roles {
        if let Some(_found_role) = group_roles.iter().find(|role| role.name == member_role) {
            found_roles.push(_found_role);
        }
    }

    let has_access = found_roles.iter().any(|v| {
        use PermissionActionType::*;
        v.permissions.iter().any(|p| {
            &p.name() == &permission.to_string()
                && match permission_action {
                    Write => p.actions().write() == true,
                    Read => p.actions().read() == true,
                    Edit => p.actions().edit() == true,
                    Delete => p.actions().delete() == true,
                }
        })
    });

    if has_access {
        return Ok(());
    } else {
        return Err(ApiError::unauthorized());
    }
}
