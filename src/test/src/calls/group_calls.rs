#![allow(unused)]
use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    group::{GroupFilter, GroupResponse, GroupSort, PostGroup, UpdateGroup},
    member::{InviteMemberResponse, JoinedMemberResponse, Member},
    paged_response::PagedResponse,
    permission::PostPermission,
    role::Role,
    sort_direction::SortDirection,
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_group(post_group: PostGroup, account_identifier: Option<String>) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(PostGroup, Option<String>), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_group",
            (post_group, account_identifier),
        )
        .expect("Failed to call add_group from pocketIC")
        .0
        .expect("Failed to add group");

    group_response
}

pub fn get_group(identifier: Principal) -> GroupResponse {
    let group_response: GroupResponse =
        query_candid_as::<(Principal,), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_group",
            (identifier,),
        )
        .expect("Failed to call get_group from pocketIC")
        .0
        .expect("Failed to get group");

    group_response
}

pub fn get_groups() -> PagedResponse<GroupResponse> {
    let sort_direction: SortDirection = SortDirection::Asc;

    let paged_response: PagedResponse<GroupResponse> = query_candid_as::<
        (usize, usize, Vec<FilterType<GroupFilter>>, GroupSort),
        (Result<PagedResponse<GroupResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_groups",
        (10, 1, vec![], GroupSort::Name(sort_direction)),
    )
    .expect("Failed to call get_groups from pocketIC")
    .0
    .expect("Failed to get groups");

    paged_response
}

pub fn edit_group(group_identifier: Principal, update_group: UpdateGroup) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(Principal, UpdateGroup), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "edit_group",
            (group_identifier, update_group),
        )
        .expect("Failed to call edit_group from pocketIC")
        .0
        .expect("Failed to edit group");

    group_response
}

// deprecated
// pub fn get_group_owner_and_privacy(
//     _group_identifier: Principal,
// ) -> Result<(Principal, Privacy), ApiError>

pub fn get_groups_by_id(group_identifiers: Vec<Principal>) -> Vec<GroupResponse> {
    let group_response: Vec<GroupResponse> =
        query_candid_as::<(Vec<Principal>,), (Result<Vec<GroupResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_groups_by_id",
            (group_identifiers,),
        )
        .expect("Failed to call get_groups_by_id from pocketIC")
        .0
        .expect("Failed to get groups by id");

    group_response
}

pub fn delete_group(group_identifier: Principal) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(Principal,), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "delete_group",
            (group_identifier,),
        )
        .expect("Failed to call delete_group from pocketIC")
        .0
        .expect("Failed to delete group");

    group_response
}

pub fn add_wallet_to_group(
    group_identifier: Principal,
    wallet_canister: Principal,
    description: String,
) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(Principal, Principal, String), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_wallet_to_group",
            (group_identifier, wallet_canister, description),
        )
        .expect("Failed to call add_wallet_to_group from pocketIC")
        .0
        .expect("Failed to add wallet to group");

    group_response
}

pub fn remove_wallet_from_group(
    group_identifier: Principal,
    wallet_canister: Principal,
) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(Principal, Principal), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "remove_wallet_from_group",
            (group_identifier, wallet_canister),
        )
        .expect("Failed to call remove_wallet_from_group from pocketIC")
        .0
        .expect("Failed to remove wallet from group");

    group_response
}

pub fn add_role_to_group(
    group_identifier: Principal,
    role_name: String,
    color: String,
    index: u64,
) -> Role {
    let role: Role =
        update_candid_as::<(Principal, String, String, u64), (Result<Role, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_role_to_group",
            (group_identifier, role_name, color, index),
        )
        .expect("Failed to call add_role_to_group from pocketIC")
        .0
        .expect("Failed to add role to group");

    role
}

pub fn remove_group_role(group_identifier: Principal, role_name: String) -> bool {
    let removed: bool = update_candid_as::<(Principal, String), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_group_role",
        (group_identifier, role_name),
    )
    .expect("Failed to call remove_group_role from pocketIC")
    .0
    .expect("Failed to remove role from group");

    removed
}

pub fn get_group_roles(group_identifier: Principal) -> Vec<Role> {
    let roles: Vec<Role> = update_candid_as::<(Principal,), (Result<Vec<Role>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group_roles",
        (group_identifier,),
    )
    .expect("Failed to call get_group_roles from pocketIC")
    .0
    .expect("Failed to get group roles");

    roles
}

pub fn edit_role_permissions(
    group_identifier: Principal,
    role_name: String,
    post_permissions: Vec<PostPermission>,
) -> bool {
    let edited: bool =
        update_candid_as::<(Principal, String, Vec<PostPermission>), (Result<bool, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "edit_role_permissions",
            (group_identifier, role_name, post_permissions),
        )
        .expect("Failed to call edit_role_permissions from pocketIC")
        .0
        .expect("Failed to edit role permissions");

    edited
}

pub fn join_group(
    group_identifier: Principal,
    account_identifier: Option<String>,
) -> JoinedMemberResponse {
    let joined_member_response: JoinedMemberResponse =
        update_candid_as::<(Principal, Option<String>), (Result<JoinedMemberResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "join_group",
            (group_identifier, account_identifier),
        )
        .expect("Failed to call join_group from pocketIC")
        .0
        .expect("Failed to join group");

    joined_member_response
}

pub fn invite_to_group(member_principal: Principal, group_identifier: Principal) -> Member {
    let member: Member = update_candid_as::<(Principal, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "invite_to_group",
        (member_principal, group_identifier),
    )
    .expect("Failed to call invite_to_group from pocketIC")
    .0
    .expect("Failed to invite to group");

    member
}

pub fn accept_user_request_group_invite(
    member_principal: Principal,
    group_identifier: Principal,
) -> Member {
    let member: Member = update_candid_as::<(Principal, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_user_request_group_invite",
        (member_principal, group_identifier),
    )
    .expect("Failed to call accept_user_request_group_invite from pocketIC")
    .0
    .expect("Failed to accept user request group invite");

    member
}

pub fn accept_owner_request_group_invite(group_identifier: Principal) -> Member {
    let member: Member = update_candid_as::<(Principal,), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_owner_request_group_invite",
        (group_identifier,),
    )
    .expect("Failed to call accept_owner_request_group_invite from pocketIC")
    .0
    .expect("Failed to accept owner request group invite");

    member
}

pub fn assign_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Member {
    let member: Member =
        update_candid_as::<(String, Principal, Principal), (Result<Member, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "assign_role",
            (role, member_identifier, group_identifier),
        )
        .expect("Failed to call assign_role from pocketIC")
        .0
        .expect("Failed to assign role");

    member
}

pub fn remove_member_role(
    role: String,
    member_identifier: Principal,
    group_identifier: Principal,
) -> Member {
    let member: Member =
        update_candid_as::<(String, Principal, Principal), (Result<Member, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "remove_member_role",
            (role, member_identifier, group_identifier),
        )
        .expect("Failed to call remove_member_role from pocketIC")
        .0
        .expect("Failed to remove member role");

    member
}

pub fn get_group_member(principal: Principal, group_identifier: Principal) -> JoinedMemberResponse {
    let joined_member_response: JoinedMemberResponse =
        query_candid_as::<(Principal, Principal), (Result<JoinedMemberResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_group_member",
            (principal, group_identifier),
        )
        .expect("Failed to call get_group_member from pocketIC")
        .0
        .expect("Failed to get group member");

    joined_member_response
}

pub fn get_groups_for_members(member_identifiers: Vec<Principal>) -> Vec<JoinedMemberResponse> {
    let joined_member_response: Vec<JoinedMemberResponse> =
        query_candid_as::<(Vec<Principal>,), (Result<Vec<JoinedMemberResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_groups_for_members",
            (member_identifiers,),
        )
        .expect("Failed to call get_groups_for_members from pocketIC")
        .0
        .expect("Failed to get groups for members");

    joined_member_response
}

pub fn get_group_members(group_identifier: Principal) -> Vec<JoinedMemberResponse> {
    let joined_member_response: Vec<JoinedMemberResponse> =
        query_candid_as::<(Principal,), (Result<Vec<JoinedMemberResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_group_members",
            (group_identifier,),
        )
        .expect("Failed to call get_group_members from pocketIC")
        .0
        .expect("Failed to get group members");

    joined_member_response
}

pub fn get_self_group() -> Member {
    let member: Member = query_candid_as::<(), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_self_group",
        (),
    )
    .expect("Failed to call get_self_group from pocketIC")
    .0
    .expect("Failed to get self group");

    member
}

pub fn get_member_roles(
    member_identifier: Principal,
    group_identifier: Principal,
) -> (Principal, Vec<String>) {
    let member_roles: (Principal, Vec<String>) =
        query_candid_as::<(Principal, Principal), (Result<(Principal, Vec<String>), ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_member_roles",
            (member_identifier, group_identifier),
        )
        .expect("Failed to call get_member_roles from pocketIC")
        .0
        .expect("Failed to get member roles");

    member_roles
}

pub fn leave_group(group_identifier: Principal) -> () {
    update_candid_as::<(Principal,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "leave_group",
        (group_identifier,),
    )
    .expect("Failed to call leave_group from pocketIC")
    .0
    .expect("Failed to leave group");
}

pub fn remove_invite(group_identifier: Principal) -> () {
    update_candid_as::<(Principal,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_invite",
        (group_identifier,),
    )
    .expect("Failed to call remove_invite from pocketIC")
    .0
    .expect("Failed to remove invite");
}

pub fn remove_member_from_group(principal: Principal, group_identifier: Principal) -> () {
    update_candid_as::<(Principal, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_member_from_group",
        (principal, group_identifier),
    )
    .expect("Failed to call remove_member_from_group from pocketIC")
    .0
    .expect("Failed to remove member from group");
}

pub fn remove_member_invite_from_group(principal: Principal, group_identifier: Principal) -> () {
    update_candid_as::<(Principal, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_member_invite_from_group",
        (principal, group_identifier),
    )
    .expect("Failed to call remove_member_invite_from_group from pocketIC")
    .0
    .expect("Failed to remove member invite from group");
}

pub fn get_group_invites(group_identifier: Principal) -> Vec<InviteMemberResponse> {
    let invites: Vec<InviteMemberResponse> =
        query_candid_as::<(Principal,), (Result<Vec<InviteMemberResponse>, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "get_group_invites",
            (group_identifier,),
        )
        .expect("Failed to call get_group_invites from pocketIC")
        .0
        .expect("Failed to get group invites");

    invites
}
