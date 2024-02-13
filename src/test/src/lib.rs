use candid::{self, encode_one, Principal};
use pocket_ic::{self, PocketIc, WasmResult};
use ic_cdk::api::management_canister::main::CanisterId;


 #[test]
 fn test_proxy() {
    let pic = PocketIc::new();

    // Create an empty canister as the anonymous principal.
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000); // 2T cycles

    let wasm_bytes = std::fs::read("../../wasm/canister.wasm.gz").expect("Failed to read wasm file");
    pic.install_canister(canister_id, wasm_bytes, vec![], None);

    let reply = call(&pic, canister_id, "get_boosted_groups");
    // assert_eq!(reply, WasmResult::Reply(vec![0]));
    println!("{:?}", reply);
 }

fn call(pic: &PocketIc, canister_id: CanisterId, method: &str) -> WasmResult {
    pic.update_call(canister_id, Principal::anonymous(), method, encode_one(()).unwrap())
        .expect("Failed to call counter canister")
}