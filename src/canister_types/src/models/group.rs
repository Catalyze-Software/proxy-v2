use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::default_roles,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy, role::Role,
        sort_direction::SortDirection,
    },
};

use super::{
    api_error::ApiError,
    boosted::Boosted,
    identifier::{Identifier, IdentifierKind, MEMBER_CANISTER_ID},
    member::{InviteMemberResponse, JoinedMemberResponse},
    permission::Permission,
};

impl_storable_for!(Group);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub owner: Principal,
    pub created_by: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub privacy_gated_type_amount: Option<u64>,
    pub roles: Vec<Role>,
    pub is_deleted: bool,
    // Shouldn't be used, use the member store data instead
    pub member_count: HashMap<Principal, usize>,
    pub wallets: HashMap<Principal, String>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Group {
    pub fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            website: Default::default(),
            location: Default::default(),
            privacy: Default::default(),
            owner: Principal::anonymous(),
            created_by: Principal::anonymous(),
            matrix_space_id: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            member_count: Default::default(),
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            privacy_gated_type_amount: Default::default(),
        }
    }

    pub fn from(group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            owner: caller(),
            created_by: caller(),
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            member_count: Default::default(),
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: false,
            updated_on: time(),
            created_on: time(),
            privacy_gated_type_amount: group.privacy_gated_type_amount,
        }
    }

    pub fn update(&mut self, group: UpdateGroup) {
        self.name = group.name;
        self.description = group.description;
        self.website = group.website;
        self.location = group.location;
        self.privacy = group.privacy;
        self.image = group.image;
        self.banner_image = group.banner_image;
        self.tags = group.tags;
        self.privacy_gated_type_amount = group.privacy_gated_type_amount;
        self.updated_on = time();
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_roles(&self) -> Vec<Role> {
        // set the default roles
        let mut roles = self.roles.clone();

        // append the custom roles stored on the group
        roles.append(&mut default_roles());
        roles
    }

    pub fn get_role_permissions(&self, role: String) -> Vec<Permission> {
        let roles = self.get_roles();
        let role = roles.iter().find(|r| r.name == role);
        if let Some(role) = role {
            return role.permissions.clone();
        }
        vec![]
    }
}

#[derive(Clone, CandidType, Deserialize)]
pub struct PostGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub matrix_space_id: String,
    pub location: Location,
    pub privacy: Privacy,
    pub privacy_gated_type_amount: Option<u64>,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct UpdateGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub image: Asset,
    pub privacy_gated_type_amount: Option<u64>,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupCallerData {
    pub joined: Option<JoinedMemberResponse>,
    pub invite: Option<InviteMemberResponse>,
    pub is_starred: bool,
}

impl GroupCallerData {
    pub fn new(
        joined: Option<JoinedMemberResponse>,
        invite: Option<InviteMemberResponse>,
        is_starred: bool,
    ) -> Self {
        Self {
            joined,
            invite,
            is_starred,
        }
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupResponse {
    pub identifier: Principal,
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub created_by: Principal,
    pub owner: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub roles: Vec<Role>,
    pub member_count: usize,
    pub wallets: Vec<(Principal, String)>,
    pub is_deleted: bool,
    pub privacy_gated_type_amount: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
    pub boosted: Option<Boosted>,
    pub caller_data: Option<GroupCallerData>,
}

impl GroupResponse {
    pub fn new(
        id: u64,
        group: Group,
        boosted: Option<Boosted>,
        caller_data: Option<GroupCallerData>,
    ) -> Self {
        let identifier = Identifier::generate(IdentifierKind::Group(id));
        let member_count: usize = group.member_count.into_iter().map(|(_, value)| value).sum();

        Self {
            identifier: identifier.to_principal().unwrap(),
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            created_by: group.created_by,
            owner: group.owner,
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            roles: group.roles,
            member_count: member_count.clone(),
            wallets: group.wallets.into_iter().collect(),
            is_deleted: group.is_deleted,
            caller_data,
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            boosted,
            updated_on: group.updated_on,
            created_on: group.created_on,
        }
    }

    pub fn from_result(
        group_result: Result<(u64, Group), ApiError>,
        boosted: Option<Boosted>,
        caller_data: Option<GroupCallerData>,
    ) -> Result<Self, ApiError> {
        match group_result {
            Err(err) => Err(err),
            Ok((id, group)) => Ok(Self::new(id, group, boosted, caller_data)),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupSort {
    Name(SortDirection),
    MemberCount(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

impl GroupSort {
    pub fn sort(&self, groups: HashMap<u64, Group>) -> Vec<(u64, Group)> {
        let mut groups: Vec<(u64, Group)> = groups.into_iter().collect();
        let member_canister_id = Principal::from_text(MEMBER_CANISTER_ID).unwrap();
        use GroupSort::*;
        use SortDirection::*;
        match self {
            Name(Asc) => {
                groups.sort_by(|(_, a), (_, b)| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            }
            Name(Desc) => {
                groups.sort_by(|(_, a), (_, b)| b.name.to_lowercase().cmp(&a.name.to_lowercase()))
            }
            MemberCount(Asc) => groups.sort_by(|(_, a), (_, b)| {
                a.member_count
                    .get(&member_canister_id)
                    .unwrap()
                    .cmp(b.member_count.get(&member_canister_id).unwrap())
            }),
            MemberCount(Desc) => groups.sort_by(|(_, a), (_, b)| {
                b.member_count
                    .get(&member_canister_id)
                    .unwrap()
                    .cmp(a.member_count.get(&member_canister_id).unwrap())
            }),
            CreatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.created_on.cmp(&b.created_on)),
            CreatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.created_on.cmp(&a.created_on)),
            UpdatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.updated_on.cmp(&b.updated_on)),
            UpdatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.updated_on.cmp(&a.updated_on)),
        }
        groups
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupFilter {
    None,
    Name(String),
    Owner(Principal),
    MemberCount((usize, usize)),
    Ids(Vec<u64>),
    Tag(u32),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl Default for GroupFilter {
    fn default() -> Self {
        GroupFilter::None
    }
}

impl GroupFilter {
    pub fn is_match(&self, id: &u64, group: &Group) -> bool {
        match self {
            GroupFilter::None => true,
            GroupFilter::Name(name) => group.name.to_lowercase().contains(&name.to_lowercase()),
            GroupFilter::Owner(owner) => group.owner == *owner,
            GroupFilter::MemberCount((min, max)) => {
                // This gets the first value of the hashmap (should always be only one value)
                let count = group
                    .member_count
                    .get(&Principal::from_text(MEMBER_CANISTER_ID).unwrap())
                    .map_or(0, |f| *f);
                count >= *min && count <= *max
            }
            GroupFilter::Ids(ids) => ids.contains(&id),
            GroupFilter::Tag(tag) => group.tags.contains(tag),
            GroupFilter::UpdatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after(group.updated_on)
                }
            }
            GroupFilter::CreatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after(group.updated_on)
                }
            }
        }
    }
}
