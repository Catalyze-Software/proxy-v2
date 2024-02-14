use crate::ENV;
use candid::Principal;
use models::models::boosted::Boosted;
use pocket_ic::query_candid;

#[test]
fn get_boosted_groups() {
    query_candid::<(), (Vec<Boosted>,)>(&ENV.pic, ENV.canister_id, "get_boosted_groups", ())
        .expect("Failed to get boosted groups");
}

#[test]
fn get_boosted_events() {
    query_candid::<(), (Vec<Boosted>,)>(&ENV.pic, ENV.canister_id, "get_boosted_events", ())
        .expect("Failed to get boosted events");
}

#[test]
fn get_e8s_per_day_boost_cost() {
    query_candid::<(), (u64,)>(&ENV.pic, ENV.canister_id, "get_e8s_per_day_boost_cost", ())
        .expect("Failed to get e8s per day boost cost");
}

// TODO: async fn boost

// ANON should fail?
#[test]
fn get_remaining_boost_time_in_seconds() {
    query_candid::<(Principal,), (u64,)>(
        &ENV.pic,
        ENV.canister_id,
        "get_remaining_boost_time_in_seconds",
        (Principal::anonymous(),),
    )
    .expect("Failed to get remaining boost time in seconds");
}
