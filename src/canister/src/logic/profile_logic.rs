use candid::Principal;
use ic_cdk::{api::time, caller};

use crate::{
    helpers::validator::Validator,
    models::{
        api_error::ApiError,
        document_details::DocumentDetails,
        identifier::{Identifier, IdentifierKind},
        profile::{PostProfile, Profile, ProfileMethods, ProfileResponse, UpdateProfile},
        relation_type::RelationType,
        validation::{ValidateField, ValidationType},
        wallet::{PostWallet, Wallet, WalletResponse},
    },
    storage::storage_api::{members, profiles, IdentifierRefMethods, StorageMethods},
};

use super::member_logic::MemberCalls;

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

        //////////////////////////////////////////////////////////////////////////////////////////
        // TODO: REMOVE THIS SECTION
        // ADDED FOR BACKWARD COMPATIBILITY
        // SHOULD BE REMOVED IN THE FUTURE
        //////////////////////////////////////////////////////////////////////////////////////////

        // generate and store an profile identifier
        let profile_identifier = profiles().new_identifier();
        let _ = profiles().insert_identifier_ref(profile_identifier);

        // generate and store an member identifier
        let member_identifier = members().new_identifier();
        let _ = members().insert_identifier_ref(member_identifier);

        //////////////////////////////////////////////////////////////////////////////////////////
        //////////////////////////////////////////////////////////////////////////////////////////

        let _ = MemberCalls::create_empty_member(caller(), profile_identifier);
        ProfileMapper::to_response(stored_profile)
    }

    pub fn update_profile(update_profile: UpdateProfile) -> Result<ProfileResponse, ApiError> {
        if let Err(err) = ProfileValidation::validate_update_profile(&update_profile) {
            return Err(err);
        }

        let (_, existing_profile) = profiles().get(caller())?;
        let updated_profile = existing_profile.update(update_profile);

        let updated_profile_result = profiles().update(caller(), updated_profile);
        ProfileMapper::to_response(updated_profile_result)
    }

    pub fn add_wallet_to_profile(post_wallet: PostWallet) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = profiles().get(caller())?;

        if existing_profile
            .wallets
            .contains_key(&post_wallet.principal)
        {
            return Err(ApiError::duplicate().add_message("Wallet already exists"));
        }

        existing_profile.wallets.insert(
            post_wallet.principal,
            Wallet {
                provider: post_wallet.provider,
                is_primary: existing_profile.wallets.len() == 0,
            },
        );

        let updated_profile = profiles().update(caller(), existing_profile);

        ProfileMapper::to_response(updated_profile)
    }

    pub fn remove_wallet_from_profile(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = profiles().get(caller())?;

        if !existing_profile.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        if existing_profile
            .wallets
            .get(&principal)
            .is_some_and(|w| w.is_primary)
        {
            return Err(ApiError::bad_request().add_message("Cannot remove primary wallet"));
        }

        existing_profile.wallets.remove(&principal);

        let updated_profile = profiles().update(caller(), existing_profile);

        ProfileMapper::to_response(updated_profile)
    }

    pub fn set_wallet_as_primary(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = profiles().get(caller())?;

        if !existing_profile.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        for (_principal, wallet) in existing_profile.wallets.iter_mut() {
            wallet.is_primary = false;
        }

        existing_profile
            .wallets
            .get_mut(&principal)
            .unwrap()
            .is_primary = true;

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

    pub fn add_starred(identifier_principal: Principal) -> Result<ProfileResponse, ApiError> {
        let identifier = Identifier::from(identifier_principal);

        if !identifier.is_valid() {
            return Err(ApiError::bad_request().add_message("Invalid identifier"));
        }

        let (_, mut existing_profile) = profiles().get(caller())?;

        if existing_profile.starred.contains_key(&identifier_principal) {
            return Err(ApiError::duplicate()
                .add_message(format!("{} already starred", identifier.kind()).as_str()));
        }

        existing_profile
            .starred
            .insert(identifier_principal, identifier.kind());

        let updated_profile = profiles().update(caller(), existing_profile);

        ProfileMapper::to_response(updated_profile)
    }

    pub fn remove_starred(identifier_principal: Principal) -> Result<ProfileResponse, ApiError> {
        let identifier = Identifier::from(identifier_principal);

        if !identifier.is_valid() {
            return Err(ApiError::bad_request().add_message("Invalid identifier"));
        }

        let (_, mut existing_profile) = profiles().get(caller())?;

        if !existing_profile.starred.contains_key(&identifier_principal) {
            return Err(ApiError::not_found()
                .add_message(format!("{} not starred", identifier.kind()).as_str()));
        }

        existing_profile.starred.remove(&identifier_principal);

        let updated_profile = profiles().update(caller(), existing_profile);

        ProfileMapper::to_response(updated_profile)
    }

    pub fn get_starred_by_kind(kind: &str) -> Vec<Principal> {
        if let Ok((_, profile)) = profiles().get(caller()) {
            return profile
                .starred
                .iter()
                .filter_map(
                    |(principal, k)| {
                        if k == &kind {
                            Some(*principal)
                        } else {
                            None
                        }
                    },
                )
                .collect();
        }
        vec![]
    }

    pub fn remove_friend(principal: Principal) -> Result<ProfileResponse, ApiError> {
        // Remove the friend from the caller profile
        let (_, mut caller_profile) = profiles().get(caller())?;

        if !caller_profile.relations.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        caller_profile.relations.remove(&principal);

        let updated_caller_profile = profiles().update(caller(), caller_profile);

        let (_, mut friend_profile) = profiles().get(principal)?;

        // Remove the caller from the friend profile
        if !friend_profile.relations.contains_key(&caller()) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        friend_profile.relations.remove(&caller());

        let updated_friend_profile = profiles().update(principal, friend_profile);

        ProfileMapper::to_response(updated_caller_profile)
    }

    pub fn block_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut caller_profile) = profiles().get(caller())?;

        caller_profile
            .relations
            .insert(principal, RelationType::Blocked.to_string());

        let updated_profile = profiles().update(caller(), caller_profile);

        let (_, mut friend_profile) = profiles().get(principal)?;

        // In case the friend has the caller as a friend, remove it
        if friend_profile.relations.contains_key(&caller()) {
            friend_profile.relations.remove(&caller());
            let _ = profiles().update(principal, friend_profile);
        }

        ProfileMapper::to_response(updated_profile)
    }

    pub fn unblock_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut caller_profile) = profiles().get(caller())?;

        if caller_profile
            .relations
            .get(&principal)
            .is_some_and(|data| data == &RelationType::Blocked.to_string())
        {
            caller_profile.relations.remove(&principal);
            let updated_profile = profiles().update(caller(), caller_profile);
            return ProfileMapper::to_response(updated_profile);
        }

        return Err(ApiError::not_found().add_message("User not blocked"));
    }

    pub fn get_relations(relation_type: RelationType) -> Vec<Principal> {
        if let Ok((_, profile)) = profiles().get(caller()) {
            return profile
                .relations
                .iter()
                .filter_map(|(principal, r)| {
                    if r == &relation_type.to_string() {
                        Some(*principal)
                    } else {
                        None
                    }
                })
                .collect();
        }
        vec![]
    }

    // TODO: add logic to check the current version of these documents and add something to prompt the user to approve the latest version
    pub fn approve_code_of_conduct(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = profiles().get(caller())?;

        profile.code_of_conduct = DocumentDetails::new(version, time());
        let updated_profile = profiles().update(caller(), profile);

        Ok(updated_profile.is_ok())
    }

    pub fn approve_privacy_policy(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = profiles().get(caller())?;

        profile.privacy_policy = Some(DocumentDetails::new(version, time()));
        let updated_profile = profiles().update(caller(), profile);

        Ok(updated_profile.is_ok())
    }

    pub fn approve_terms_of_service(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = profiles().get(caller())?;

        profile.terms_of_service = Some(DocumentDetails::new(version, time()));
        let updated_profile = profiles().update(caller(), profile);

        Ok(updated_profile.is_ok())
    }
}

impl ProfileMapper {
    pub fn to_response(
        profile_result: Result<(Principal, Profile), ApiError>,
    ) -> Result<ProfileResponse, ApiError> {
        match profile_result {
            Err(err) => Err(err),
            Ok((_, profile)) => {
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
