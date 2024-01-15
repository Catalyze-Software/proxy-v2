use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::member::Member;

pub type MemberStore = StorageRef<String, Member>;

impl StorageMethods<Principal, Member> for MemberStore {
    fn get(&self, key: Principal) -> Result<Member, String> {
        Ok(self
            .borrow()
            .get(&key.to_string())
            .ok_or("Entity not found")?
            .clone())
    }

    fn insert(&self, value: Member) -> Result<Member, String> {
        panic!("This value requires a key to be inserted, use `insert_by_key` instead")
    }

    fn insert_by_key(&self, key: Principal, value: Member) -> Result<Member, String> {
        if self.borrow().contains_key(&key.to_string()) {
            return Err("Key already exists".to_string());
        }

        self.borrow_mut().insert(key.to_string(), value.clone());
        Ok(value)
    }

    fn update(&mut self, key: Principal, value: Member) -> Result<Member, String> {
        self.borrow_mut().insert(key.to_string(), value.clone());
        Ok(value)
    }

    fn remove(&mut self, key: Principal) -> bool {
        self.borrow_mut().remove(&key.to_string()).is_some()
    }
}
