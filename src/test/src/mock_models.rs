use models::models::{asset::Asset, group::PostGroup, location::Location, privacy::Privacy, profile::PostProfile, profile_privacy::ProfilePrivacy};

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