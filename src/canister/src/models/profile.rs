use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::{api::time, caller};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;

use crate::models::{
    application_role::ApplicationRole, asset::Asset, date_range::DateRange,
    sort_direction::SortDirection,
};

use super::{
    document_details::DocumentDetails,
    profile_privacy::ProfilePrivacy,
    wallet::{Wallet, WalletResponse},
};

pub trait ProfileMethods {
    // Instead of using the `Default` trait we added the method here so we have it all in one place
    fn default() -> Self;
    // Instead of using the `From` trait we added the method here so we have it all in one place
    fn update(self, group: UpdateProfile) -> Self;
    // How are we going to handle this? Are we going to fetch the combined data from the different stores?
    // Or are we going to fetch the data before calling this method?
    // fn to_response()
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Profile {
    pub principal: Principal,
    pub member_identifier: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: DocumentDetails,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub wallets: HashMap<Principal, Wallet>,
    pub starred: HashMap<Principal, String>,
    pub relations: HashMap<Principal, String>,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Storable for Profile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl ProfileMethods for Profile {
    fn default() -> Self {
        Self {
            principal: Principal::anonymous(),
            member_identifier: Principal::anonymous(),
            username: Default::default(),
            display_name: Default::default(),
            application_role: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            privacy: Default::default(),
            about: Default::default(),
            email: Default::default(),
            date_of_birth: Default::default(),
            city: Default::default(),
            state_or_province: Default::default(),
            country: Default::default(),
            profile_image: Default::default(),
            banner_image: Default::default(),
            skills: Default::default(),
            interests: Default::default(),
            causes: Default::default(),
            website: Default::default(),
            code_of_conduct: Default::default(),
            wallets: Default::default(),
            starred: Default::default(),
            relations: Default::default(),
            extra: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            privacy_policy: None,
            terms_of_service: None,
        }
    }

    fn update(self, profile: UpdateProfile) -> Self {
        Self {
            principal: self.principal,
            username: self.username,
            display_name: profile.display_name,
            application_role: self.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: profile.about,
            email: profile.email.unwrap_or("".to_string()),
            date_of_birth: profile.date_of_birth,
            city: profile.city,
            state_or_province: profile.state_or_province,
            country: profile.country,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            website: profile.website,
            wallets: self.wallets,
            starred: self.starred,
            relations: self.relations,
            code_of_conduct: self.code_of_conduct,
            extra: profile.extra,
            updated_on: time(),
            created_on: self.created_on,
            member_identifier: self.member_identifier,
            privacy_policy: self.privacy_policy,
            terms_of_service: self.terms_of_service,
        }
    }
}

impl From<PostProfile> for Profile {
    fn from(profile: PostProfile) -> Self {
        Self {
            principal: caller(),
            username: profile.username,
            display_name: profile.display_name,
            application_role: ApplicationRole::default(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: "".to_string(),
            email: "".to_string(),
            date_of_birth: 0,
            city: "".to_string(),
            state_or_province: "".to_string(),
            country: "".to_string(),
            profile_image: Asset::None,
            banner_image: Asset::None,
            skills: vec![],
            interests: vec![],
            causes: vec![],
            website: "".to_string(),
            wallets: HashMap::new(),
            starred: HashMap::new(),
            relations: HashMap::new(),
            code_of_conduct: DocumentDetails::new(0, 0),
            extra: profile.extra,
            updated_on: time(),
            created_on: time(),
            member_identifier: Principal::anonymous(),
            privacy_policy: None,
            terms_of_service: None,
        }
    }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PostProfile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub extra: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateProfile {
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: Option<String>,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub extra: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub identifier: Principal,
    pub principal: Principal,
    pub member_identifier: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: DocumentDetails,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub wallets: Vec<WalletResponse>,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileSort {
    Username(SortDirection),
    DisplayName(SortDirection),
    FirstName(SortDirection),
    LastName(SortDirection),
    Email(SortDirection),
    City(SortDirection),
    StateOrProvince(SortDirection),
    Country(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileFilter {
    Username(String),
    DisplayName(String),
    FirstName(String),
    LastName(String),
    Email(String),
    City(String),
    StateOrProvince(String),
    Country(String),
    UpdatedOn(DateRange),
    Skill(u32),
    Interest(u32),
    Cause(u32),
    CreatedOn(DateRange),
}
