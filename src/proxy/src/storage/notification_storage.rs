use super::storage_api::NOTIFICATION_CANISTER;
use candid::Principal;
use catalyze_shared::{
    notification::{Notification, NotificationFilter, NotificationSort},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct NotificationStorageClient;

impl StorageClient<u64, Notification, NotificationFilter, NotificationSort>
    for NotificationStorageClient
{
    fn name(&self) -> String {
        "notification".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &NOTIFICATION_CANISTER
    }
}

impl StorageClientInsertable<Notification, NotificationFilter, NotificationSort>
    for NotificationStorageClient
{
}

pub fn notifications() -> NotificationStorageClient {
    NotificationStorageClient
}
