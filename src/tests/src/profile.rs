use catalyze_shared::{profile::PostProfile, profile_privacy::ProfilePrivacy};

use crate::{
    calls::profile,
    utils::{context, random_identity},
};

#[tokio::test]
async fn test_insert_profiles() {
    let ctx = context().await;

    let profiles = (0..5)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| {
            let name = format!("user_{}", x);

            let req = PostProfile {
                username: name.clone(),
                display_name: name.clone(),
                first_name: name.clone(),
                last_name: name.clone(),
                privacy: ProfilePrivacy::Public,
                extra: "".to_owned(),
            };

            (random_identity(), req)
        })
        .collect::<Vec<_>>();

    let mut ids = vec![];

    for (identity, profile) in profiles {
        let now = std::time::Instant::now();

        let add_resp = profile::add_profile(&ctx, identity, profile)
            .await
            .expect("Failed to add profile");

        println!(
            "Created profile: principal {}, name: {}, elapsed: {:.2?}",
            add_resp.principal,
            add_resp.display_name,
            now.elapsed()
        );

        let now = std::time::Instant::now();

        let get_resp = profile::get_profile(&ctx, add_resp.principal)
            .await
            .expect("Failed to get profile");

        println!(
            "Got profile by id: principal {}, name: {}, elapsed: {:.2?}",
            get_resp.principal,
            get_resp.display_name,
            now.elapsed()
        );

        ids.push(add_resp.principal);
    }

    let now = std::time::Instant::now();
    let _ = profile::get_profiles(&ctx, ids)
        .await
        .expect("Failed to get profiles");

    println!("Got profiles by ids: elapsed: {:.2?}", now.elapsed());
}
