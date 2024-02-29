use crate::logic::boost_logic::BoostCalls;
use ic_cdk::{post_upgrade, pre_upgrade};

#[post_upgrade]
pub fn post_upgrade() {
    let _ = BoostCalls::start_timers_after_upgrade();
}

#[pre_upgrade]
pub fn pre_upgrade() {}
