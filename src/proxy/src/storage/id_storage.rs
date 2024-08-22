use super::storage_api::{Storage, IDS, IDS_MEMORY_ID};
use catalyze_shared::{api_error::ApiError, StaticStorageRef, StorageRef};
use ic_stable_structures::{memory_manager::MemoryId, Storable};

pub const ID_KIND_FRIEND_REQUESTS: &str = "friend_requests";
pub const ID_KIND_LOGS: &str = "logs";
pub const ID_KIND_NOTIFICATIONS: &str = "notifications";
pub const ID_KIND_REWARDS_BUFFER: &str = "rewards_buffer";

#[derive(Debug, Clone)]
pub enum IDKind {
    FriendRequests,
    Logs,
    Notifications,
    RewardBuffer,
}

impl std::fmt::Display for IDKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDKind::FriendRequests => write!(f, "{ID_KIND_FRIEND_REQUESTS}"),
            IDKind::Logs => write!(f, "{ID_KIND_LOGS}"),
            IDKind::Notifications => write!(f, "{ID_KIND_NOTIFICATIONS}"),
            IDKind::RewardBuffer => write!(f, "{ID_KIND_REWARDS_BUFFER}"),
        }
    }
}

impl std::str::FromStr for IDKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ID_KIND_FRIEND_REQUESTS => Ok(IDKind::FriendRequests),
            ID_KIND_LOGS => Ok(IDKind::Logs),
            ID_KIND_NOTIFICATIONS => Ok(IDKind::Notifications),
            ID_KIND_REWARDS_BUFFER => Ok(IDKind::RewardBuffer),
            _ => Err(format!("Unknown IDKind: {s}")),
        }
    }
}

pub struct IDStore;

impl Storage<String, u64> for IDStore {
    const NAME: &'static str = "ids";

    fn storage() -> StaticStorageRef<String, u64> {
        &IDS
    }

    fn memory_id() -> MemoryId {
        IDS_MEMORY_ID
    }
}

impl IDStore {
    pub fn next(kind: &str) -> Result<u64, ApiError> {
        let kind = kind.parse::<IDKind>().map_err(|e| {
            ApiError::unexpected()
                .add_message(format!("Failed to perform storage action, error: {e}"))
        })?;

        Self::increment(kind)
    }

    pub fn get_all() -> Vec<(String, u64)> {
        Self::storage().with(|data| data.borrow().iter().collect())
    }

    // Use the old id if the id store is empty (only needed for existing data)
    fn increment(k: IDKind) -> Result<u64, ApiError> {
        let id = Self::get(k.clone()).unwrap_or_else(|| Self::get_last(k.clone())) + 1;

        Self::storage().with(|data| {
            data.borrow_mut().insert(k.to_string(), id);
            Ok(id)
        })
    }

    fn get(kind: IDKind) -> Option<u64> {
        Self::storage().with(|data| data.borrow().get(&kind.to_string()))
    }

    fn get_last(kind: IDKind) -> u64 {
        match kind {
            IDKind::FriendRequests => super::FriendRequestStore::storage().with(last_key),
            IDKind::Logs => super::LoggerStore::storage().with(last_key),
            IDKind::Notifications => super::NotificationStore::storage().with(last_key),
            IDKind::RewardBuffer => super::RewardBufferStore::storage().with(last_key),
        }
    }
}

fn last_key<T: Storable>(data: &StorageRef<u64, T>) -> u64 {
    data.borrow().last_key_value().map(|(k, _)| k).unwrap_or(1)
}
