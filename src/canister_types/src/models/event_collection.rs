use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(EventCollection);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EventCollection {
    pub events: Vec<u64>,
}

impl EventCollection {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn get_events_count(&self) -> u64 {
        self.events.len() as u64
    }

    pub fn add_event(&mut self, id: u64) {
        self.events.push(id);
    }

    pub fn remove_event(&mut self, id: &u64) {
        self.events.retain(|p| p != id);
    }
}
