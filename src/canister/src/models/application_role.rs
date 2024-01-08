use core::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApplicationRole {
    Owner,
    Admin,
    Moderator,
    Leader,
    Member,
    Watcher,
    Guest,
    Blocked,
    Banned,
}

impl Default for ApplicationRole {
    fn default() -> Self {
        ApplicationRole::Member
    }
}

impl fmt::Display for ApplicationRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApplicationRole::*;
        match self {
            Owner => write!(f, "Owner"),
            Admin => write!(f, "Admin"),
            Moderator => write!(f, "Moderator"),
            Leader => write!(f, "Leader"),
            Member => write!(f, "Member"),
            Watcher => write!(f, "OwneWatcherr"),
            Guest => write!(f, "Guest"),
            Blocked => write!(f, "Blocked"),
            Banned => write!(f, "Banned"),
        }
    }
}
