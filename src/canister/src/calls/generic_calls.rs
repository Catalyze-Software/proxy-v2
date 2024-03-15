use crate::logic::{boost_logic::BoostCalls, websocket_logic::Websocket};
use ic_cdk::{init, post_upgrade, pre_upgrade};

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
