use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;

use crate::models::{
    asset::Asset, date_range::DateRange, location::Location, privacy::Privacy, role::Role,
    sort_direction::SortDirection,
};

pub trait GroupMethods {
    // Instead of using the `Default` trait we added the method here so we have it all in one place
    fn default() -> Self;
    // Instead of using the `From` trait we added the method here so we have it all in one place
    fn from(&self, group: PostGroup) -> Self;
    fn update(&self, group: UpdateGroup) -> Self;
    // How are we going to handle this? Are we going to fetch the combined data from the different stores?
    // Or are we going to fetch the data before calling this method?
    // fn to_response()
}

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
    pub member_count: HashMap<Principal, usize>,
    pub wallets: HashMap<Principal, String>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl GroupMethods for Group {
    fn default() -> Self {
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

    fn from(&self, group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            owner: Principal::anonymous(),
            created_by: Principal::anonymous(),
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            member_count: Default::default(),
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            privacy_gated_type_amount: group.privacy_gated_type_amount,
        }
    }

    fn update(&self, group: UpdateGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            owner: self.owner,
            created_by: self.created_by,
            matrix_space_id: self.matrix_space_id.clone(),
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            roles: self.roles.clone(),
            is_deleted: self.is_deleted,
            member_count: self.member_count.clone(),
            wallets: self.wallets.clone(),
            updated_on: self.updated_on,
            created_on: self.created_on,
        }
    }
}

// Stable storage
impl Storable for Group {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
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
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupSort {
    Name(SortDirection),
    MemberCount(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupFilter {
    Name(String),
    Owner(Principal),
    MemberCount((usize, usize)),
    Identifiers(Vec<Principal>),
    Tag(u32),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}
