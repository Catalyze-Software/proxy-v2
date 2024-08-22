use crate::{
    helpers::guards::{is_developer, is_prod_developer},
    logic::{boost_logic::BoostCalls, id_logic::IDLogic, websocket_logic::Websocket},
    storage::{
        history_canister, reward_canister, FriendRequestStore, LoggerStore, NotificationStore,
        RewardBufferStore, RewardTimerStore, StorageUpdateable, UserNotificationStore,
    },
};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    http_types::{HttpRequest, HttpResponse},
    CellStorage,
};
use ic_cdk::{
    api::{
        canister_balance128,
        management_canister::main::{create_canister, CanisterSettings, CreateCanisterArgument},
    },
    id, init, post_upgrade, pre_upgrade, query, update,
};

#[post_upgrade]
pub async fn post_upgrade() {
    Websocket::init();
    RewardTimerStore::start();
    let _ = BoostCalls::start_timers_after_upgrade().await;
}

#[pre_upgrade]
pub fn pre_upgrade() {}

#[init]
pub fn init() {
    Websocket::init();
    RewardTimerStore::start();
}

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[update(guard = "is_developer")]
pub fn _dev_clear_notifications() {
    UserNotificationStore::clear();
    NotificationStore::clear();
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

#[query(guard = "is_developer")]
fn _dev_get_all_ids() -> Vec<(String, u64)> {
    IDLogic::get_all()
}

#[update(guard = "is_prod_developer")]
fn _dev_prod_init() -> Result<(), ApiError> {
    if id().to_string() != "2jvhk-5aaaa-aaaap-ahewa-cai" {
        return Err(
            ApiError::unsupported().add_message("This canister is not the production canister")
        );
    }

    let _ = history_canister().set(Principal::from_text("inc34-eqaaa-aaaap-ahl2a-cai").unwrap());
    let _ = reward_canister().set(Principal::from_text("zgfl7-pqaaa-aaaap-accpa-cai").unwrap());
    Ok(())
}

#[update(guard = "is_prod_developer")]
fn _dev_clear() {
    FriendRequestStore::clear();
    NotificationStore::clear();
    UserNotificationStore::clear();
    LoggerStore::clear();
    RewardBufferStore::clear();
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
