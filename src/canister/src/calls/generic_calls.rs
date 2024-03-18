use crate::logic::{boost_logic::BoostCalls, websocket_logic::Websocket};
use canister_types::models::http_types::{HttpRequest, HttpResponse};
use ic_cdk::{init, post_upgrade, pre_upgrade, query};

#[post_upgrade]
pub fn post_upgrade() {
    let _ = BoostCalls::start_timers_after_upgrade();
    Websocket::init();
}

#[pre_upgrade]
pub fn pre_upgrade() {}

#[init]
pub fn init() {
    Websocket::init();
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let mut path: Vec<&str> = req.url.as_str().split('/').collect();
    path = path.iter().filter(|p| !p.is_empty()).cloned().collect();

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
