use crate::{
    helpers::validation::Validator,
    models::{
        api_error::ApiError,
        profile::PostProfile,
        validation::{ValidateField, ValidationType},
    },
};

// pub struct ProfileMethods;

// pub trait ProfileTrait {
//     fn validate_post_profile(post_profile: PostProfile) -> Result<(), ApiError>;
// }

// impl ProfileTrait for ProfileMethods {
pub fn add_profile(post_profile: PostProfile) -> Result<(), ApiError> {
    if let Err(err) = validate_post_profile(post_profile) {
        return Err(err);
    }

    Ok(())
}

pub fn validate_post_profile(post_profile: PostProfile) -> Result<(), ApiError> {
    let validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(post_profile.username, 3, 64),
            "username".to_string(),
        ),
        ValidateField(
            ValidationType::StringLength(post_profile.display_name, 3, 64),
            "display_name".to_string(),
        ),
    ];

    Validator(validator_fields).validate()
}
// }
