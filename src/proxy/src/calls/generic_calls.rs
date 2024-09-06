use crate::{helpers::guards::is_prod_developer, logic::websocket_logic::Websocket};
use candid::Principal;
use catalyze_shared::http_types::{HttpRequest, HttpResponse};
use ic_cdk::{
    api::{
        canister_balance128,
        management_canister::main::{create_canister, CanisterSettings, CreateCanisterArgument},
    },
    init, post_upgrade, pre_upgrade, query, update,
};

#[post_upgrade]
pub async fn post_upgrade() {
    Websocket::init();
}

#[pre_upgrade]
pub fn pre_upgrade() {}

#[init]
pub fn init() {
    Websocket::init();
}

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[update(guard = "is_prod_developer")]
async fn _dev_create_canister(controllers: Vec<Principal>) -> Result<Principal, String> {
    let arg = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            wasm_memory_limit: None,
            log_visibility: None,
        }),
    };

    let current_cycles = canister_balance128();
    if current_cycles < 10_000_000_000_000 {
        return Err("Pleas make sure there are more then 10T cycles available".to_string());
    }

    let cycles: u128 = 3_000_000_000_000;

    match create_canister(arg, cycles).await {
        Ok((canister,)) => Ok(canister.canister_id),
        Err((_, err)) => Err(err),
    }
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let mut path: Vec<&str> = req.url.as_str().split('/').collect();
    path.retain(|p| !p.is_empty());

    let permission_denied = HttpResponse {
        status_code: 403,
        headers: vec![],
        body: vec![],
    };
    match req.method.as_str() {
        "GET" => match path[0] {
            "version" => HttpResponse {
                status_code: 200,
                headers: vec![],
                body: env!("CARGO_PKG_VERSION").as_bytes().to_vec(),
            },
            "changelog" => HttpResponse {
                status_code: 200,
                headers: vec![],
                body: include_bytes!("../../../../CHANGELOG.md").to_vec(),
            },
            _ => permission_denied,
        },
        _ => permission_denied,
    }
}
