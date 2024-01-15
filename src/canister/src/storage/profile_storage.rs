use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::models::profile::Profile;

pub type ProfileStore = StorageRef<String, Profile>;

impl StorageMethods<Principal, Profile> for ProfileStore {
    fn get(&self, key: Principal) -> Result<Profile, String> {
        Ok(self
            .borrow()
            .get(&key.to_string())
            .ok_or("Profile not found")?
            .clone())
    }

    fn insert(&self, _value: Profile) -> Result<Profile, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    fn insert_by_key(&self, key: Principal, value: Profile) -> Result<Profile, String> {
        if self.borrow().contains_key(&key.to_string()) {
            return Err("Key already exists".to_string());
        }

        self.borrow_mut().insert(key.to_string(), value.clone());
        Ok(value)
    }

    fn update(&mut self, key: Principal, value: Profile) -> Result<Profile, String> {
        if !self.borrow().contains_key(&key.to_string()) {
            return Err("Key does not exists".to_string());
        }

        self.borrow_mut().insert(key.to_string(), value.clone());
        Ok(value)
    }

    fn remove(&mut self, key: Principal) -> bool {
        self.borrow_mut().remove(&key.to_string()).is_some()
    }
}
