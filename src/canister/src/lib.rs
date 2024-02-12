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
pub mod models;
pub mod storage;

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use crate::models::api_error::*;
    use crate::models::attendee::*;
    use crate::models::boosted::*;
    use crate::models::event::*;
    use crate::models::filter_type::*;
    use crate::models::friend_request::*;
    use crate::models::group::*;
    use crate::models::member::*;
    use crate::models::paged_response::*;
    use crate::models::permission::*;
    use crate::models::privacy::*;
    use crate::models::profile::*;
    use crate::models::relation_type::*;
    use crate::models::report::*;
    use crate::models::role::*;
    use crate::models::wallet::*;
    use candid::export_service;
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
