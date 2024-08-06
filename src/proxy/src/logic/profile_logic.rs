use super::{
    attendee_logic::AttendeeCalls, member_logic::MemberCalls, notification_logic::NotificationCalls,
};
use crate::storage::{
    profiles, AttendeeStore, EventAttendeeStore, EventStore, GroupMemberStore, GroupStore,
    MemberStore, StorageInsertableByKey, StorageQueryable, UserNotificationStore,
};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    document_details::DocumentDetails,
    helpers::validator::Validator,
    member_collection::MemberCollection,
    profile::{PostProfile, Profile, ProfileFilter, ProfileResponse, UpdateProfile},
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

        let new_profile = Profile::from(post_profile);
        let stored_profile = profiles().insert(caller(), new_profile).await?;

        let _ = MemberCalls::create_empty_member(caller());
        let _ = AttendeeCalls::create_empty_attendee(caller());
        let _ = UserNotificationStore::insert_by_key(caller(), UserNotifications::new());

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
            .wallets
            .contains_key(&post_wallet.principal)
        {
            return Err(ApiError::duplicate().add_message("Wallet already exists"));
        }

        existing_profile.wallets.insert(
            post_wallet.principal,
            Wallet {
                provider: post_wallet.provider,
                is_primary: existing_profile.wallets.is_empty(),
            },
        );

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn remove_wallet_from_profile(
        principal: Principal,
    ) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

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

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn set_wallet_as_primary(principal: Principal) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

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
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if existing_profile.starred.contains(&subject) {
            return Err(ApiError::duplicate().add_message("already starred"));
        }

        match subject {
            Subject::Group(id) => {
                let group_members = GroupMemberStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !group_members.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined groups")
                    );
                }
            }
            Subject::Event(id) => {
                let event_attendees = EventAttendeeStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !event_attendees.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined events")
                    );
                }
            }
            _ => return Err(ApiError::not_implemented().add_message("Subject type not supported")),
        };

        existing_profile.starred.push(subject);

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn remove_starred(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if !existing_profile.starred.contains(&subject) {
            return Err(ApiError::not_found().add_message("not starred"));
        }

        existing_profile.starred.retain(|s| s != &subject);

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn get_starred_by_subject(subject: SubjectType) -> Vec<u64> {
        if let Ok((_, profile)) = profiles().get(caller()).await {
            return profile
                .starred
                .iter()
                .filter(|s| s.get_type() == subject)
                .map(|s| *s.get_id())
                .collect();
        }
        vec![]
    }

    pub async fn add_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if existing_profile.pinned.contains(&subject) {
            return Err(ApiError::duplicate().add_message("already pinned"));
        }

        match subject {
            Subject::Group(id) => {
                let group_members = GroupMemberStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !group_members.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only pin joined groups")
                    );
                }
            }
            Subject::Event(id) => {
                let event_attendees = EventAttendeeStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !event_attendees.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only pin joined events")
                    );
                }
            }
            _ => return Err(ApiError::not_implemented().add_message("Subject type not supported")),
        };

        existing_profile.pinned.push(subject);

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn remove_pinned(subject: Subject) -> CanisterResult<ProfileResponse> {
        let (_, mut existing_profile) = profiles().get(caller()).await?;

        if !existing_profile.pinned.contains(&subject) {
            return Err(ApiError::not_found().add_message("not pinned"));
        }

        existing_profile.pinned.retain(|s| s != &subject);

        ProfileResponse::from(profiles().update(caller(), existing_profile).await?).to_result()
    }

    pub async fn get_pinned_by_subject(
        subject: SubjectType,
    ) -> CanisterResult<Vec<SubjectResponse>> {
        let (_, profile) = profiles().get(caller()).await?;

        let mut subjects = vec![];

        for s in profile.pinned.iter() {
            if s.get_type() == subject {
                subjects.push(Self::get_subject_response_by_subject(s).await);
            }
        }

        Ok(subjects)
    }

    pub async fn remove_friend(principal: Principal) -> CanisterResult<ProfileResponse> {
        // Remove the friend from the caller profile
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        if !caller_profile.relations.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        caller_profile.relations.remove(&principal);
        let updated_caller_profile = profiles().update(caller(), caller_profile).await?;

        let (_, mut friend_profile) = profiles().get(principal).await?;

        // Remove the caller from the friend profile
        if !friend_profile.relations.contains_key(&caller()) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        friend_profile.relations.remove(&caller());

        profiles().update(principal, friend_profile).await?;

        NotificationCalls::notification_remove_friend(principal, caller());
        ProfileResponse::from(updated_caller_profile).to_result()
    }

    pub async fn block_user(principal: Principal) -> CanisterResult<ProfileResponse> {
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        caller_profile
            .relations
            .insert(principal, RelationType::Blocked.to_string());

        let updated_profile = profiles().update(caller(), caller_profile).await?;

        let (_, mut friend_profile) = profiles().get(principal).await?;

        // In case the friend has the caller as a friend, remove it
        if friend_profile.relations.contains_key(&caller()) {
            friend_profile.relations.remove(&caller());
            let _ = profiles().update(principal, friend_profile).await?;
        }

        ProfileResponse::from(updated_profile).to_result()
    }

    pub async fn unblock_user(principal: Principal) -> CanisterResult<ProfileResponse> {
        let (_, mut caller_profile) = profiles().get(caller()).await?;

        if caller_profile
            .relations
            .get(&principal)
            .is_some_and(|data| data == &RelationType::Blocked.to_string())
        {
            caller_profile.relations.remove(&principal);
            let updated_profile = profiles().update(caller(), caller_profile).await?;
            return ProfileResponse::from(updated_profile).to_result();
        }

        Err(ApiError::not_found().add_message("User not blocked"))
    }

    pub async fn get_relations(relation_type: RelationType) -> CanisterResult<Vec<Principal>> {
        let (_, profile) = profiles().get(caller()).await?;

        let resp = profile
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

        profile.code_of_conduct = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn approve_privacy_policy(version: u64) -> CanisterResult<bool> {
        let (_, mut profile) = profiles().get(caller()).await?;

        profile.privacy_policy = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn approve_terms_of_service(version: u64) -> CanisterResult<bool> {
        let (_, mut profile) = profiles().get(caller()).await?;

        profile.terms_of_service = Some(DocumentDetails::new(version, time()));
        Ok(profiles().update(caller(), profile).await.is_ok())
    }

    pub async fn get_subject_response_by_subject(subject: &Subject) -> SubjectResponse {
        match subject.clone() {
            Subject::Group(id) => SubjectResponse::Group(GroupStore::get(id).ok()),
            Subject::Event(id) => SubjectResponse::Event(EventStore::get(id).ok()),
            Subject::Profile(id) => SubjectResponse::Profile(profiles().get(id).await.ok()),
            Subject::Member(id) => SubjectResponse::Member(MemberStore::get(id).ok()),
            Subject::Attendee(id) => SubjectResponse::Attendee(AttendeeStore::get(id).ok()),
            Subject::None => SubjectResponse::None,
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
