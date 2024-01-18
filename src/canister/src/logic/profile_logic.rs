use ic_cdk::caller;

use crate::{
    helpers::validator::Validator,
    models::{
        api_error::ApiError,
        profile::{PostProfile, Profile, ProfileMethods, UpdateProfile},
        validation::{ValidateField, ValidationType},
    },
    storage::storage_api::{profiles, StorageMethods},
};

pub fn add_profile(post_profile: PostProfile) -> Result<Profile, String> {
    if let Err(err) = validate_post_profile(&post_profile) {
        return Err("Validation error".to_string());
    }

    let new_profile = Profile::from(post_profile);
    profiles().insert_by_key(caller(), new_profile)
}

pub fn update_profile(update_profile: UpdateProfile) -> Result<Profile, String> {
    if let Err(err) = validate_update_profile(&update_profile) {
        return Err("Validation error".to_string());
    }

    let existing_profile = profiles().get(caller())?;

    let updated_profile = Profile::update(existing_profile, update_profile);
    profiles().update(caller(), updated_profile)
}

// VALIDATION
pub fn validate_post_profile(post_profile: &PostProfile) -> Result<(), ApiError> {
    let validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(post_profile.username.clone(), 3, 64),
            "username".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(post_profile.display_name.clone(), 3, 64),
            "display_name".to_string(),
        ),
    ];

    Validator::new(validator_fields).validate()
}

pub fn validate_update_profile(update_profile: &UpdateProfile) -> Result<(), ApiError> {
    let mut validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(update_profile.display_name.clone(), 3, 32),
            "display_name".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_profile.about.clone(), 0, 1000),
            "about".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_profile.city.clone(), 0, 64),
            "city".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_profile.country.clone(), 0, 64),
            "country".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(update_profile.website.clone(), 0, 200),
            "website".to_string(),
        ),
        ValidateField(
            ValidationType::Count(update_profile.skills.len(), 0, 50),
            "skills".to_string(),
        ),
        ValidateField(
            ValidationType::Count(update_profile.interests.len(), 0, 50),
            "interests".to_string(),
        ),
        ValidateField(
            ValidationType::Count(update_profile.causes.len(), 0, 50),
            "causes".to_string(),
        ),
    ];

    match update_profile.email.clone() {
        None => {}
        Some(_email) => validator_fields.push(ValidateField(
            ValidationType::Email(_email),
            "email".to_string(),
        )),
    }

    Validator::new(validator_fields).validate()
}
