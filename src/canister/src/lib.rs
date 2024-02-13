// should all be removed after implementation
#![allow(deprecated)]
#![allow(unused_variables)]

use candid::Principal;
use ic_cdk::query;

pub static CATALYZE_MULTI_SIG: &str = "fcygz-gqaaa-aaaap-abpaa-cai";
pub static E8S_PER_DAY_BOOST_COST: u64 = 3500000;

pub mod calls;
pub mod helpers;
pub mod logic;
pub mod misc;
pub mod storage;

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    use models::models::api_error::*;
    use models::models::attendee::*;
    use models::models::boosted::*;
    use models::models::event::*;
    use models::models::filter_type::*;
    use models::models::friend_request::*;
    use models::models::group::*;
    use models::models::member::*;
    use models::models::paged_response::*;
    use models::models::permission::*;
    use models::models::privacy::*;
    use models::models::profile::*;
    use models::models::relation_type::*;
    use models::models::report::*;
    use models::models::role::*;
    use models::models::wallet::*;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().parent().unwrap().join("candid");
    write(dir.join(format!("canister.did")), __export_did_tmp_()).expect("Write failed.");
}
