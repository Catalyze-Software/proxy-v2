use std::cell::RefCell;

use ic_stable_structures::StableBTreeMap;

use crate::{
    entities::{attendee::Attendee, event::Event},
    stores::main_store::{ATTENDEES_MEMORY_ID, EVENTS_MEMORY_ID, MEMORY_MANAGER},
};

use super::main_store::Memory;

thread_local! {
    /// The `events` store.
    /// # Note
    /// This store is used to keep track of the events that have been created.
    pub static EVENTS: RefCell<StableBTreeMap<u64, Event, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENTS_MEMORY_ID)),
        )
    );

    /// The `attendees` store.
    /// # Note
    /// This store is used to keep track of the attendees of events.
    pub static ATTENDEES: RefCell<StableBTreeMap<u64, Attendee, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ATTENDEES_MEMORY_ID)),
        )
    );
}

/// The `Events` struct.
/// # Note
/// This struct is used to interact with the `events` store.
pub struct Events;

impl Events {
    /// Returns the `event` associated with the given key.
    /// # Arguments
    /// * `key` - The key to get the `event` at in `u64` format.
    /// # Returns
    /// * `Some(Event)` - The `event` associated with the given key.
    /// * `None` - If no `event` is associated with the given key.
    /// # Panics
    /// Panics if the `event` associated with the given key is not a `event`.
    pub fn get(key: &u64) -> Option<Event> {
        EVENTS.with(|p| p.borrow().get(&key))
    }

    /// Inserts or updates a `event` into the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to insert the `event` at in `u64` format.
    /// * `value` - The `event` to insert.
    /// # Panics
    /// Panics if the `event` associated with the given key is not a `event`.
    /// # Note
    /// This function will overwrite the `event` at the given key's position if one already exists.
    pub fn insert(key: u64, value: Event) {
        EVENTS.with(|p| p.borrow_mut().insert(key, value));
    }

    /// Removes a `event` from the store at a given key's position.
    /// # Arguments
    /// * `key` - The key to insert the `event` at in `u64` format.
    /// # Panics
    /// Panics if the `event` associated with the given key is not a `event`.
    /// # Note
    /// This function will do nothing if no `event` is associated with the given key.
    pub fn remove(key: &u64) {
        EVENTS.with(|p| p.borrow_mut().remove(&key));
    }
}
