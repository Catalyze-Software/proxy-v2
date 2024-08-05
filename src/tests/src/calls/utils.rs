use candid::Principal;
use eyre::Context as _;
use ic_agent::Identity;

use crate::utils::Context;

pub async fn query(
    ctx: &Context,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>> {
    let response = ctx
        .agent
        .query(canister_id, method)
        .with_arg(args)
        .await
        .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

    Ok(response)
}

#[allow(dead_code)]
pub async fn update(
    ctx: &Context,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>> {
    update_call(ctx.agent.clone(), canister_id, method, args).await
}

pub async fn update_with_identity<I>(
    ctx: &Context,
    identity: I,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>>
where
    I: 'static + Identity,
{
    let mut agent = ctx.agent.clone();
    agent.set_identity(identity);

    update_call(agent, canister_id, method, args).await
}

async fn update_call(
    agent: ic_agent::Agent,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>> {
    agent
        .update(canister_id, method)
        .with_arg(args)
        .await
        .wrap_err_with(|| format!("Failed to perform update \"{}\" request", method))
}
