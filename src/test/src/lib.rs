use candid::Principal;
use ic_cdk::api::management_canister::provisional::CanisterId;
use lazy_static::lazy_static;
use pocket_ic::PocketIc;
use crate::mock_ids::member_test_id;

pub mod mock_ids;

#[cfg(test)]
mod calls;
#[cfg(test)]
mod mock_models;
#[cfg(test)]
mod flow1;

pub struct TestEnv {
    pub pic: PocketIc,
    pub canister_id: CanisterId,
}

lazy_static! {
    pub static ref ENV: TestEnv = init_pocket_ic();

    pub static ref SENDER: Principal = member_test_id();
}

fn init_pocket_ic() -> TestEnv {
    let pic = PocketIc::new();

    // Create an empty canister as the anonymous principal.
    let canister_id = pic.create_canister();

    // Add cycles to the canister.
    pic.add_cycles(canister_id, 2_000_000_000_000); // 2T cycles

    // Install the wasm module into the canister.
    let wasm_bytes =
        std::fs::read("../../wasm/canister.wasm.gz").expect("Failed to read wasm file");
    pic.install_canister(canister_id, wasm_bytes, vec![], None);

    TestEnv { pic, canister_id }
}
