use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::{group::Group, member::Member};

pub type GroupStore = StorageRef<u64, Group>;
pub type MemberStore = StorageRef<String, Member>;

impl StorageMethods<u64, Group> for GroupStore {
    fn get(&self, id: u64) -> Result<Group, String> {
        Ok(self.borrow().get(&id).ok_or("Group not found")?.clone())
    }

    fn insert(&self, entity: Group) -> Result<Group, String> {
        let id = self
            .borrow()
            .last_key_value()
            .map(|(k, _)| k + 1)
            .unwrap_or(0);
        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn insert_by_key(&self, id: u64, entity: Group) -> Result<Group, String> {
        panic!("This entity does not require a key to be inserted, use `insert` instead")
    }

    fn update(&mut self, id: u64, entity: Group) -> Result<Group, String> {
        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: u64) -> bool {
        self.borrow_mut().remove(&id).is_some()
    }
}

impl StorageMethods<Principal, Member> for MemberStore {
    fn get(&self, id: Principal) -> Result<Member, String> {
        Ok(self
            .borrow()
            .get(&id.to_string())
            .ok_or("Member not found")?
            .clone())
    }

    fn insert(&self, entity: Member) -> Result<Member, String> {
        panic!("This entity requires a key to be inserted, use `insert_by_key` instead")
    }

    fn insert_by_key(&self, principal: Principal, entity: Member) -> Result<Member, String> {
        self.borrow_mut()
            .insert(principal.to_string(), entity.clone());
        Ok(entity)
    }

    fn update(&mut self, id: Principal, entity: Member) -> Result<Member, String> {
        self.borrow_mut().insert(id.to_string(), entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: Principal) -> bool {
        self.borrow_mut().remove(&id.to_string()).is_some()
    }
}
