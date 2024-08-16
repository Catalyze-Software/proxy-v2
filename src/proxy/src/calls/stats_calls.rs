use crate::helpers::guards::is_monitor;
use crate::storage::*;
use catalyze_shared::{CanisterResult, StorageClient};
use ic_cdk::query;

#[query(composite = true, guard = "is_monitor")]
async fn store_stats() -> CanisterResult<Vec<String>> {
    Ok(vec![
        format!("ProfileStore: {}", profiles().size().await?),
        format!("FriendRequestStore: {}", FriendRequestStore::size()),
        format!("GroupStore: {}", groups().size().await?),
        format!("EventStore: {}", events().size().await?),
        format!("ReportStore: {}", reports().size().await?),
        format!("BoostStore: {}", boosteds().size().await?),
        format!("NotificationStore: {}", NotificationStore::size()),
        format!("UserNotificationsStore: {}", UserNotificationStore::size()),
        format!("LoggerStore: {}", LoggerStore::size()),
        format!("TagStore: {}", TagStore::size()),
        format!("CategoryStore: {}", CategoryStore::size()),
        format!("SkillStore: {}", SkillStore::size()),
    ])
}
