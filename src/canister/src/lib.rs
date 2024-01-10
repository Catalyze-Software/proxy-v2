#![allow(deprecated)]

// The different stores with subject specific business logic and storage
pub mod stores {
    pub mod event_store;
    pub mod group_store;
    pub mod main_store;
    pub mod profile_store;
    pub mod report_store;
}

// The different exposed methods that can be called on the canister
pub mod methods {
    pub mod event_methods;
    pub mod group_methods;
    pub mod profile_methods;
    pub mod report_methods;
}

// The base entities that are used for storage
pub mod entities {
    pub mod attendee;
    pub mod event;
    pub mod group;
    pub mod invite;
    pub mod member;
    pub mod profile;
    pub mod report;
}

// Shared structs that are used by different entities and models
pub mod models {
    pub mod api_error;
    pub mod application_role;
    pub mod asset;
    pub mod date_range;
    pub mod filter_type;
    pub mod location;
    pub mod paged_response;
    pub mod privacy;
    pub mod role;
    pub mod sort_direction;
    pub mod storage;
    pub mod validation;
}

pub mod helpers {
    pub mod auth_helper;
}
