use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::profile::Profile;

pub type ProfileStore = StorageRef<String, Profile>;

impl StorageMethods<Principal, Profile> for ProfileStore {
    fn get(&self, id: Principal) -> Result<Profile, String> {
        Ok(self
            .borrow()
            .get(&id.to_string())
            .ok_or("Profile not found")?
            .clone())
    }

    fn insert(&self, entity: Profile) -> Result<Profile, String> {
        panic!("This entity requires a key to be inserted, use `insert_by_key` instead")
    }

    fn insert_by_key(&self, principal: Principal, entity: Profile) -> Result<Profile, String> {
        self.borrow_mut()
            .insert(principal.to_string(), entity.clone());
        Ok(entity)
    }

    fn update(&mut self, id: Principal, entity: Profile) -> Result<Profile, String> {
        self.borrow_mut().insert(id.to_string(), entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: Principal) -> bool {
        self.borrow_mut().remove(&id.to_string()).is_some()
    }
}
