use ic_cdk::{post_upgrade, pre_upgrade};

#[post_upgrade]
pub fn post_upgrade() {
    // start boost timers after upgrade
}

#[pre_upgrade]
pub fn pre_upgrade() {}
