use candid::Principal;
use ic_cdk::caller;

use crate::{
    helpers::validator::Validator,
    models::{
        api_error::ApiError,
        profile::{PostProfile, Profile, ProfileMethods, ProfileResponse, UpdateProfile},
        validation::{ValidateField, ValidationType},
        wallet::{PostWallet, Wallet, WalletResponse},
    },
    storage::storage_api::{profiles, StorageMethods},
};

pub struct ProfileCalls;
pub struct ProfileValidation;
pub struct ProfileMapper;

impl ProfileCalls {
    pub fn add_profile(post_profile: PostProfile) -> Result<ProfileResponse, ApiError> {
        if let Err(err) = ProfileValidation::validate_post_profile(&post_profile) {
            return Err(err);
        }

        let new_profile = Profile::from(post_profile);
        let stored_profile = profiles().insert_by_key(caller(), new_profile);
        ProfileMapper::to_response(stored_profile)
    }

    pub fn update_profile(update_profile: UpdateProfile) -> Result<ProfileResponse, ApiError> {
        if let Err(err) = ProfileValidation::validate_update_profile(&update_profile) {
            return Err(err);
        }

        let existing_profile = profiles().get(caller())?;
        let updated_profile = existing_profile.update(update_profile);

        let updated_profile_result = profiles().update(caller(), updated_profile);
        ProfileMapper::to_response(updated_profile_result)
    }

    pub fn add_wallet_to_profile(post_wallet: PostWallet) -> Result<ProfileResponse, ApiError> {
        let mut existing_profile = profiles().get(caller())?;
        existing_profile
            .wallets
            .insert(post_wallet.principal, Wallet::from(post_wallet));

        let updated_profile = profiles().update(caller(), existing_profile);

        ProfileMapper::to_response(updated_profile)
    }

    pub fn get_profile(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let profile_result = profiles().get(principal);
        ProfileMapper::to_response(profile_result)
    }

    pub fn get_profiles(principals: Vec<Principal>) -> Vec<ProfileResponse> {
        let profiles_result = profiles().get_many(principals);
        profiles_result
            .into_iter()
            .map(|profile| ProfileMapper::to_response(Ok(profile)).unwrap())
            .collect()
    }
}

impl ProfileMapper {
    pub fn to_response(
        profile_result: Result<Profile, ApiError>,
    ) -> Result<ProfileResponse, ApiError> {
        match profile_result {
            Err(err) => Err(err),
            Ok(profile) => {
                let wallets = profile
                    .wallets
                    .into_iter()
                    .map(|(principal, wallet)| WalletResponse {
                        provider: wallet.provider,
                        principal,
                        is_primary: wallet.is_primary,
                    })
                    .collect();

                let result = ProfileResponse {
                    username: profile.username,
                    display_name: profile.display_name,
                    about: profile.about,
                    city: profile.city,
                    country: profile.country,
                    website: profile.website,
                    skills: profile.skills,
                    interests: profile.interests,
                    causes: profile.causes,
                    email: profile.email,
                    identifier: profile.principal, // chage
                    principal: profile.principal,
                    member_identifier: profile.member_identifier,
                    application_role: profile.application_role,
                    first_name: profile.first_name,
                    last_name: profile.last_name,
                    privacy: profile.privacy,
                    date_of_birth: profile.date_of_birth,
                    state_or_province: profile.state_or_province,
                    profile_image: profile.profile_image,
                    banner_image: profile.banner_image,
                    code_of_conduct: profile.code_of_conduct,
                    privacy_policy: profile.privacy_policy,
                    terms_of_service: profile.terms_of_service,
                    wallets,
                    extra: profile.extra,
                    updated_on: profile.updated_on,
                    created_on: profile.created_on,
                };
                Ok(result)
            }
        }
    }
}

impl ProfileValidation {
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
}
