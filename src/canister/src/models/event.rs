use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy,
        sort_direction::SortDirection,
    },
};

impl_storable_for!(Event);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Event {
    name: String,
    description: String,
    date: DateRange,
    privacy: Privacy,
    group_identifier: Principal,
    created_by: Principal,
    owner: Principal,
    website: String,
    location: Location,
    image: Asset,
    banner_image: Asset,
    tags: Vec<u32>,
    is_canceled: (bool, String),
    is_deleted: bool,
    attendee_count: HashMap<Principal, usize>,
    metadata: Option<String>,
    updated_on: u64,
    created_on: u64,
}

impl From<PostEvent> for Event {
    fn from(post_event: PostEvent) -> Self {
        Self {
            name: post_event.name,
            description: post_event.description,
            date: post_event.date,
            privacy: post_event.privacy,
            group_identifier: Principal::anonymous(),
            created_by: caller(),
            owner: post_event.owner,
            website: post_event.website,
            location: post_event.location,
            image: post_event.image,
            banner_image: post_event.banner_image,
            tags: post_event.tags,
            is_canceled: (false, "".to_string()),
            is_deleted: false,
            attendee_count: Default::default(),
            metadata: post_event.metadata,
            updated_on: time(),
            created_on: time(),
        }
    }
}

impl Event {
    pub fn update(&mut self, update_event: UpdateEvent) -> Self {
        self.name = update_event.name;
        self.description = update_event.description;
        self.date = update_event.date;
        self.privacy = update_event.privacy;
        self.website = update_event.website;
        self.location = update_event.location;
        self.image = update_event.image;
        self.banner_image = update_event.banner_image;
        self.tags = update_event.tags;
        self.metadata = update_event.metadata;
        self.updated_on = time();
        self.clone()
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.updated_on = time();
        self.clone()
    }

    pub fn cancel(&mut self, reason: String) -> Self {
        self.is_canceled = (true, reason);
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            date: Default::default(),
            privacy: Default::default(),
            group_identifier: Principal::anonymous(),
            created_by: Principal::anonymous(),
            owner: Principal::anonymous(),
            website: Default::default(),
            location: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            is_canceled: Default::default(),
            is_deleted: Default::default(),
            attendee_count: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            metadata: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PostEvent {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub owner: Principal,
    pub banner_image: Asset,
    pub metadata: Option<String>,
    pub tags: Vec<u32>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpdateEvent {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub owner: Principal,
    pub banner_image: Asset,
    pub metadata: Option<String>,
    pub tags: Vec<u32>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    StartDate(SortDirection),
    EndDate(SortDirection),
    AttendeeCount(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventFilter {
    None,
    Name(String),
    StartDate(DateRange),
    EndDate(DateRange),
    Owner(Principal),
    Identifiers(Vec<Principal>),
    Tag(u32),
    IsCanceled(bool),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl Default for EventFilter {
    fn default() -> Self {
        EventFilter::None
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EventResponse {
    pub identifier: Principal,
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub created_by: Principal,
    pub owner: Principal,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
    pub attendee_count: usize,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub tags: Vec<u32>,
    pub metadata: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
    pub group_identifier: Principal,
}
