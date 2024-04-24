use crate::{
    logic::{boost_logic::BoostCalls, websocket_logic::Websocket},
    storage::{
        GroupMemberStore, MemberStore, NotificationStore, StorageMethods, UsernotificationStore,
    },
};
use candid::Principal;
use canister_types::models::http_types::{HttpRequest, HttpResponse};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};

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

#[update]
pub fn _dev_clear_notifications(super_secret_password: String) -> bool {
    if super_secret_password != "super_secret_password" {
        return false;
    } else {
        UsernotificationStore::clear();
        NotificationStore::clear();
        return true;
    }
}

#[query]
pub fn _dev_check_member_sync(
    principal: Principal,
    group_id: u64,
) -> ((String, bool), (String, bool)) {
    let mut member_store_check: (String, bool) = ("MemberStore".to_string(), false);
    let mut group_member_store_check: (String, bool) = ("GroupMemberStore".to_string(), false);

    member_store_check.1 = MemberStore::get(principal).is_ok();
    let group_members = GroupMemberStore::get(group_id);
    group_member_store_check.1 = match group_members {
        Ok((_, group_members)) => group_members.is_member(&principal),
        Err(_) => false,
    };

    (member_store_check, group_member_store_check)
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
