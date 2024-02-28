use canister_types::models::{
    asset::Asset,
    group::PostGroup,
    location::Location,
    privacy::Privacy,
    profile::{PostProfile, UpdateProfile},
    profile_privacy::ProfilePrivacy,
    wallet::PostWallet,
};

use super::principals::{wallet_test_id, wallet_test_id2};

pub fn mock_post_profile() -> PostProfile {
    PostProfile {
        username: "test_username".to_string(),
        display_name: "test_display_name".to_string(),
        first_name: "test_first_name".to_string(),
        last_name: "test_last_name".to_string(),
        privacy: ProfilePrivacy::Public,
        extra: "test_extra".to_string(),
    }
}

pub fn mock_update_profile() -> UpdateProfile {
    UpdateProfile {
        display_name: "test_update_display_name".to_string(),
        first_name: "test_update_first_name".to_string(),
        last_name: "test_update_last_name".to_string(),
        privacy: ProfilePrivacy::Public,
        about: "test_update_about".to_string(),
        email: Some("test_update_email@test_domain.com".to_string()),
        date_of_birth: 0,
        city: "test_update_city".to_string(),
        state_or_province: "test_update_state_or_province".to_string(),
        country: "test_update_country".to_string(),
        profile_image: Asset::None,
        banner_image: Asset::None,
        skills: vec![],
        interests: vec![],
        causes: vec![],
        website: "test_update_website".to_string(),
        extra: "test_update_extra".to_string(),
    }
}

pub fn mock_post_group() -> PostGroup {
    PostGroup {
        name: "test_name".to_string(),
        description: "test_description".to_string(),
        privacy: Privacy::Public,
        website: "test_website".to_string(),
        matrix_space_id: "test_matrix_space_id".to_string(),
        location: Location::None,
        privacy_gated_type_amount: None,
        image: Asset::None,
        banner_image: Asset::None,
        tags: vec![],
    }
}

pub fn mock_post_wallet() -> PostWallet {
    PostWallet {
        provider: "test_provider".to_string(),
        principal: wallet_test_id(),
    }
}

pub fn mock_post_wallet2() -> PostWallet {
    PostWallet {
        provider: "test_provider2".to_string(),
        principal: wallet_test_id2(),
    }
}
