use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(UserNotifications);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserNotifications(HashMap<u64, bool>);

impl UserNotifications {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add(&mut self, id: u64, is_read: bool) {
        if !self.0.contains_key(&id) {
            self.0.insert(id, is_read);
        }
    }

    pub fn remove(&mut self, id: &u64) {
        self.0.remove(id);
    }

    pub fn mark_as_read(&mut self, id: &u64, is_read: bool) {
        self.0.insert(*id, is_read);
    }

    pub fn mark_as_read_many(&mut self, ids: Vec<u64>, is_read: bool) {
        for id in ids {
            self.0.insert(id, is_read);
        }
    }

    pub fn remove_many(&mut self, ids: Vec<u64>) {
        for id in ids {
            self.0.remove(&id);
        }
    }

    pub fn contains(&self, id: &u64) -> bool {
        self.0.contains_key(id)
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

    pub fn to_vec(&self) -> Vec<(u64, bool)> {
        self.0.clone().into_iter().collect()
    }

    pub fn ids(&self) -> Vec<u64> {
        self.0.clone().into_iter().map(|(id, _)| id).collect()
    }
}
