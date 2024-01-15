use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef, MEMBERS};
use crate::models::member::Member;

pub type MemberStore = StorageRef<String, Member>;

impl StorageMethods<Principal, Member> for MemberStore {
    fn get(key: Principal) -> Result<Member, String> {
        MEMBERS.with(|data| {
            data.borrow()
                .get(&key.to_string())
                .ok_or("Entity not found".to_string())
        })
    }

    fn insert(_value: Member) -> Result<Member, String> {
        Err("This value requires a key to be inserted, use `insert_by_key` instead".to_string())
    }

    fn insert_by_key(key: Principal, value: Member) -> Result<Member, String> {
        MEMBERS.with(|data| {
            if data.borrow().contains_key(&key.to_string()) {
                return Err("Key already exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn update(key: Principal, value: Member) -> Result<Member, String> {
        MEMBERS.with(|data| {
            if !data.borrow().contains_key(&key.to_string()) {
                return Err("Key does not exists".to_string());
            }

            data.borrow_mut().insert(key.to_string(), value.clone());
            Ok(value)
        })
    }

    fn remove(key: Principal) -> bool {
        MEMBERS.with(|data| data.borrow_mut().remove(&key.to_string()).is_some())
    }
}
