use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Identity;

pub fn member_test_id() -> Principal {
    let principal: Principal = Secp256k1Identity::from_pem_file("src/identity.pem")
        .expect("failed to read pem file")
        .sender()
        .expect("failed to get principal");

    principal
}

pub fn canister_test_id() -> Principal {
    // Dashboard frontend canister id
    Principal::from_text("ca77u-aiaaa-aaaap-abxiq-cai").expect("Failed to parse canister id")
}
