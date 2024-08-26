use super::notification_logic::NotificationCalls;
use crate::storage::{events, groups, profiles};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    document_details::DocumentDetails,
    helpers::validator::Validator,
    profile_with_refs::{
        PostProfile, ProfileFilter, ProfileResponse, ProfileWithRefs, UpdateProfile,
    },
    relation_type::RelationType,
    subject::{Subject, SubjectResponse, SubjectType},
    user_notifications::UserNotifications,
    validation::{ValidateField, ValidationType},
    wallet::{PostWallet, Wallet},
    CanisterResult, StorageClient, StorageClientInsertableByKey,
};
use ic_cdk::{api::time, caller};

pub struct ProfileCalls;
pub struct ProfileValidation;

impl ProfileCalls {
    pub async fn add_profile(post_profile: PostProfile) -> CanisterResult<ProfileResponse> {
        ProfileValidation::validate_post_profile(&post_profile)?;

        let exists = profiles()
            .find(ProfileFilter::Username(post_profile.username.clone()).into())
            .await?
            .is_some();

        if exists {
            return Err(ApiError::duplicate().add_message("Username already exists"));
        }

        let mut new_profile = ProfileWithRefs::from(post_profile);
        new_profile.references.notifications = UserNotifications::new();
        let stored_profile = profiles().insert(caller(), new_profile).await?;

        ProfileResponse::from(stored_profile).to_result()
    }

    pub async fn update_profile(update_profile: UpdateProfile) -> CanisterResult<ProfileResponse> {
        ProfileValidation::validate_update_profile(&update_profile)?;

        let (_, existing_profile) = profiles().get(caller()).await?;
        let updated_profile = existing_profile.update(update_profile);

        ProfileResponse::from(profiles().update(caller(), updated_profile).await?).to_result()
    }

    pub async fn add_wallet_to_profile(post_wallet: PostWallet) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if existing_profile
            .references
            .wallets
            .contains_key(&post_wallet.principal.to_string())
        {
            return Err(ApiError::duplicate().add_message("Wallet already exists"));
        }

        existing_profile.references.wallets.insert(
            post_wallet.principal.to_string(),
            Wallet {
                provider: post_wallet.provider,
                is_primary: existing_profile.references.wallets.is_empty(),
            },
        );

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn remove_wallet_from_profile(
        principal: Principal,
    ) -> CanisterResult<ProfileResponse> {
        let principal = principal.to_string();
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if !existing_profile.references.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        if existing_profile
            .references
            .wallets
            .get(&principal)
            .is_some_and(|w| w.is_primary)
        {
            return Err(ApiError::bad_request().add_message("Cannot remove primary wallet"));
        }

        existing_profile.references.wallets.remove(&principal);

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn set_wallet_as_primary(principal: Principal) -> CanisterResult<ProfileResponse> {
        let principal = principal.to_string();
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if !existing_profile.references.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        for (_principal, wallet) in existing_profile.references.wallets.iter_mut() {
            wallet.is_primary = false;
        }

        existing_profile
            .references
            .wallets
            .get_mut(&principal)
            .unwrap()
            .is_primary = true;

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn get_profile(principal: Principal) -> CanisterResult<ProfileResponse> {
        ProfileResponse::from(profiles().get(principal).await?).to_result()
    }

    pub async fn get_profiles(principals: Vec<Principal>) -> CanisterResult<Vec<ProfileResponse>> {
        let profiles = profiles()
            .get_many(principals)
            .await?
            .into_iter()
            .map(|profile| profile.into())
            .collect();

        Ok(profiles)
    }

    pub async fn add_starred(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut profile) = profiles().get(caller()).await?;

        if profile.is_starred(&subject) {
            return Err(ApiError::duplicate().add_message("already starred"));
        }

        Self::validate_subject(subject.clone()).await?;
        profile.references.starred.push(subject);

        ProfileResponse::from(profiles().update(caller(), profile).await?).to_result()
    }

    pub async fn remove_starred(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut profile) = profiles().get(caller()).await?;

        if !profile.is_starred(&subject) {
            return Err(ApiError::not_found().add_message("not starred"));
        }

        profile.remove_starred(&subject);

        ProfileResponse::from(profiles().update(caller(), profile).await?).to_result()
    }

    pub async fn get_starred_by_subject(subject: SubjectType) -> Vec<u64> {
        if let Ok((_, profile)) = profiles().get(caller()).await {
            return profile
                .references
                .starred
                .iter()
                .filter(|s| s.get_type() == subject)
                .map(|s| *s.get_id())
                .collect();
        }
        vec![]
    }

    pub async fn add_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut profile) = profiles().get(caller()).await?;

        if profile.is_pinned(&subject) {
            return Err(ApiError::duplicate().add_message("already pinned"));
        }

        Self::validate_subject(subject.clone()).await?;
        profile.references.pinned.push(subject);

        ProfileResponse::from(profiles().update(caller(), profile).await?).to_result()
    }

    pub async fn remove_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut profile) = profiles().get(caller()).await?;

        if !profile.is_pinned(&subject) {
            return Err(ApiError::not_found().add_message("not pinned"));
        }

        profile.remove_pinned(&subject);

        ProfileResponse::from(profiles().update(caller(), profile).await?).to_result()
    }

    pub async fn get_pinned_by_subject(
        subject: SubjectType,
    ) -> CanisterResult<Vec<SubjectResponse>> {
        let (_, profile) = profiles().get(caller()).await?;

        let mut subjects = vec![];

        for s in profile.references.pinned.iter() {
            if s.get_type() == subject {
                subjects.push(Self::get_subject_response_by_subject(s).await);
            }
        }

        Ok(subjects)
    }

    pub async fn remove_friend(principal: Principal) -> CanisterResult<ProfileResponse> {
        // Remove the friend from the caller profile
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        if !caller_profile.references.relations.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        caller_profile.references.relations.remove(&principal);
        let updated_caller_profile = profiles().update(caller(), caller_profile).await?;

        let (_, mut friend_profile) = profiles().get(principal).await?;

        // Remove the caller from the friend profile
        if !friend_profile.references.relations.contains_key(&caller()) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        friend_profile.references.relations.remove(&caller());

        profiles().update(principal, friend_profile).await?;

        NotificationCalls::notification_remove_friend(principal, caller()).await;
        ProfileResponse::from(updated_caller_profile).to_result()
    }

    pub async fn block_user(principal: Principal) -> CanisterResult<ProfileResponse> {
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        caller_profile
            .references
            .relations
            .insert(principal, RelationType::Blocked.to_string());

        let updated_profile = profiles().update(caller(), caller_profile).await?;

        let (_, mut friend_profile) = profiles().get(principal).await?;

        // In case the friend has the caller as a friend, remove it
        if friend_profile.references.relations.contains_key(&caller()) {
            friend_profile.references.relations.remove(&caller());
            let _ = profiles().update(principal, friend_profile).await?;
        }

        ProfileResponse::from(updated_profile).to_result()
    }

    pub async fn unblock_user(principal: Principal) -> CanisterResult<ProfileResponse> {
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        if caller_profile
            .references
            .relations
            .get(&principal)
            .is_some_and(|data| data == &RelationType::Blocked.to_string())
        {
            caller_profile.references.relations.remove(&principal);
            let updated_profile = profiles().update(caller(), caller_profile).await?;
            return ProfileResponse::from(updated_profile).to_result();
        }

        Err(ApiError::not_found().add_message("User not blocked"))
    }

    pub async fn get_relations(relation_type: RelationType) -> CanisterResult<Vec<Principal>> {
        let (_, profile) = profiles().get(caller()).await?;

        let resp = profile
            .references
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

        Ok(resp)
    }

    pub async fn get_relations_with_profiles(
        relation_type: RelationType,
    ) -> CanisterResult<Vec<ProfileResponse>> {
        Self::get_profiles(ProfileCalls::get_relations(relation_type).await?).await
    }

    // TODO: add logic to check the current version of these documents and add something to prompt the user to approve the latest version
    pub async fn approve_code_of_conduct(version: u64) -> CanisterResult<bool> {
        let (_, mut profile) = profiles().get(caller()).await?;

        profile.documents.code_of_conduct = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn approve_privacy_policy(version: u64) -> CanisterResult<bool> {
        let (_, mut profile) = profiles().get(caller()).await?;

        profile.documents.privacy_policy = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn approve_terms_of_service(version: u64) -> CanisterResult<bool> {
        let (_, mut profile) = profiles().get(caller()).await?;

        profile.documents.terms_of_service = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn get_subject_response_by_subject(subject: &Subject) -> SubjectResponse {
        match subject.clone() {
            Subject::Group(id) => SubjectResponse::Group(groups().get(id).await.ok()),
            Subject::Event(id) => SubjectResponse::Event(events().get(id).await.ok()),
            Subject::Profile(id) => SubjectResponse::Profile(profiles().get(id).await.ok()),
            Subject::Member(id) => {
                if let Ok((_, profile)) = profiles().get(id).await {
                    return SubjectResponse::Member(Some((id, profile.references.groups)));
                }
                SubjectResponse::Member(None)
            }
            Subject::Attendee(id) => {
                if let Ok((_, profile)) = profiles().get(id).await {
                    return SubjectResponse::Attendee(Some((id, profile.references.events)));
                }
                SubjectResponse::Attendee(None)
            }
            _ => SubjectResponse::None,
        }
    }

    async fn validate_subject(subject: Subject) -> CanisterResult<()> {
        match subject {
            Subject::Group(id) => {
                let (_, group) = groups().get(id).await?;

                if !group.is_member(caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined groups")
                    );
                }
            }
            Subject::Event(id) => {
                let (_, event) = events().get(id).await?;

                if !event.is_attendee(caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined events")
                    );
                }
            }
            _ => return Err(ApiError::not_implemented().add_message("Subject type not supported")),
        };

        Ok(())
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
