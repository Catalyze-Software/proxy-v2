use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, PROFILES};
use crate::models::profile::Profile;

pub type ProfileStore = StorageRef<String, Profile>;

impl StorageMethods<Principal, Profile> for ProfileStore {
    fn get(key: Principal) -> Result<Profile, String> {
        PROFILES.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Profile not found".to_string())
        })
    }

    fn insert(_value: Profile) -> Result<Profile, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    fn insert_by_key(key: Principal, value: Profile) -> Result<Profile, String> {
        PROFILES.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn update(key: Principal, value: Profile) -> Result<Profile, String> {
        PROFILES.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn remove(key: Principal) -> bool {
        PROFILES.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
