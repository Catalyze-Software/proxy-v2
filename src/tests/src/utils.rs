use std::{path::Path, str::FromStr};

use candid::Principal;
use elliptic_curve::SecretKey;
use ic_agent::{
    identity::{BasicIdentity, Secp256k1Identity},
    Identity,
};

pub fn random_identity() -> impl Identity {
    let private_key = SecretKey::random(&mut rand::thread_rng());
    Secp256k1Identity::from_private_key(private_key)
}

pub struct Context {
    pub agent: ic_agent::Agent,
    pub proxy: Principal,
}

fn default_pem_path() -> String {
    let home_dir = std::env::var("HOME").expect("HOME environment variable is not set");

    format!(
        "{}/.config/dfx/identity/catalyze_development/identity.pem",
        home_dir
    )
}

pub async fn context() -> Context {
    let ic_url = std::env::var("IC_URL").unwrap_or_else(|_| "https://icp0.io".to_string());

    let identity = std::env::var("IDENTITY_PATH").unwrap_or_else(|_| default_pem_path());
    let identity = Path::new(&identity);
    let identity = BasicIdentity::from_pem_file(identity).expect("Failed to get identity");

    let proxy =
        std::env::var("PROXY_ID").unwrap_or_else(|_| "puwkw-6qaaa-aaaap-ahmvq-cai".to_string());
    let proxy = Principal::from_str(&proxy).expect("Failed to parse proxy principal");

    let agent = ic_agent::Agent::builder()
        .with_url(ic_url)
        .with_identity(identity)
        .build()
        .expect("Failed to build agent");

    agent
        .fetch_root_key()
        .await
        .expect("Failed to fetch root key for the icp agent");

    Context { proxy, agent }
}
