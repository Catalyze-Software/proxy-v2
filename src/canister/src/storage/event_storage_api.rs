use candid::Principal;

use super::storage_api::{StorageMethods, StorageRef};
use crate::entities::{attendee::Attendee, event::Event};

pub type EventStore = StorageRef<u64, Event>;
pub type AttendeeStore = StorageRef<String, Attendee>;

impl StorageMethods<u64, Event> for EventStore {
    fn get(&self, id: u64) -> Result<Event, String> {
        Ok(self.borrow().get(&id).ok_or("Event not found")?.clone())
    }

    fn insert(&self, entity: Event) -> Result<Event, String> {
        let id = self
            .borrow()
            .last_key_value()
            .map(|(k, _)| k + 1)
            .unwrap_or(0);

        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn insert_by_key(&self, id: u64, entity: Event) -> Result<Event, String> {
        panic!("This entity does not require a key to be inserted, use `insert` instead")
    }

    fn update(&mut self, id: u64, entity: Event) -> Result<Event, String> {
        self.borrow_mut().insert(id, entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: u64) -> bool {
        self.borrow_mut().remove(&id).is_some()
    }
}

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    fn get(&self, id: Principal) -> Result<Attendee, String> {
        Ok(self
            .borrow()
            .get(&id.to_string())
            .ok_or("Attendee not found")?
            .clone())
    }

    fn insert(&self, entity: Attendee) -> Result<Attendee, String> {
        panic!("This entity requires a key to be inserted, use `insert_by_key` instead")
    }

    fn insert_by_key(&self, principal: Principal, entity: Attendee) -> Result<Attendee, String> {
        self.borrow_mut()
            .insert(principal.to_string(), entity.clone());
        Ok(entity)
    }

    fn update(&mut self, id: Principal, entity: Attendee) -> Result<Attendee, String> {
        self.borrow_mut().insert(id.to_string(), entity.clone());
        Ok(entity)
    }

    fn remove(&mut self, id: Principal) -> bool {
        self.borrow_mut().remove(&id.to_string()).is_some()
    }
}
