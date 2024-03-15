use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Identity;

pub fn member_test_id() -> Principal {
    let principal: Principal = Secp256k1Identity::from_pem_file("src/mocks/identity.pem")
        .expect("failed to read pem file")
        .sender()
        .expect("failed to get principal");

    principal
}

pub fn member_test_id2() -> Principal {
    let principal: Principal = Secp256k1Identity::from_pem_file("src/mocks/identity2.pem")
        .expect("failed to read pem file")
        .sender()
        .expect("failed to get principal");

    principal
}

pub fn canister_test_id() -> Principal {
    // Dapps 0
    Principal::from_text("7xnbj-wqaaa-aaaap-aa4ea-cai").expect("Failed to parse canister id")
}

pub fn canister_test_id2() -> Principal {
    // Dapps 1
    Principal::from_text("5escj-6iaaa-aaaap-aa4kq-cai").expect("Failed to parse canister id")
}

pub fn wallet_test_id() -> Principal {
    // Dapps 2
    Principal::from_text("443xk-qiaaa-aaaap-aa4oq-cai").expect("Failed to parse canister id")
}

pub fn wallet_test_id2() -> Principal {
    // Dapps 3
    Principal::from_text("4sz2c-lyaaa-aaaap-aa4pq-cai").expect("Failed to parse canister id")
}
