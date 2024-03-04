use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(UnreadNotifications);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UnreadNotifications(Vec<u64>);

impl UnreadNotifications {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, id: u64) {
        self.0.push(id);
    }

    pub fn remove(&mut self, id: u64) {
        self.0.retain(|&x| x != id);
    }

    pub fn contains(&self, id: u64) -> bool {
        self.0.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn to_vec(&self) -> Vec<u64> {
        self.0.clone()
    }
}
