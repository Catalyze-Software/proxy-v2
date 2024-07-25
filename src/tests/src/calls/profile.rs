use candid::{Encode, Principal};
use catalyze_shared::profile::{PostProfile, ProfileResponse};
use ic_agent::Identity;

use crate::{result::CanisterResult, utils::Context};

use super::utils;

pub async fn add_profile<I: 'static + Identity>(
    ctx: &Context,
    identity: I,
    input: PostProfile,
) -> eyre::Result<ProfileResponse> {
    let resp =
        utils::update_with_identity(ctx, identity, &ctx.proxy, "add_profile", Encode!(&input)?)
            .await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn get_profile(ctx: &Context, principal: Principal) -> eyre::Result<ProfileResponse> {
    let resp = utils::query(ctx, &ctx.proxy, "get_profile", Encode!(&principal)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn get_profiles(
    ctx: &Context,
    principals: Vec<Principal>,
) -> eyre::Result<Vec<ProfileResponse>> {
    let resp = utils::query(ctx, &ctx.proxy, "get_profiles", Encode!(&principals)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}
