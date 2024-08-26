use crate::helpers::guards::is_monitor;
use crate::storage::*;
use catalyze_shared::{CanisterResult, StorageClient};
use ic_cdk::query;

#[query(composite = true, guard = "is_monitor")]
async fn store_stats() -> CanisterResult<Vec<String>> {
    Ok(vec![
        format!("ProfileStore: {}", profiles().size().await?),
        format!("FriendRequestStore: {}", friend_requests().size().await?),
        format!("GroupStore: {}", groups().size().await?),
        format!("EventStore: {}", events().size().await?),
        format!("ReportStore: {}", reports().size().await?),
        format!("BoostStore: {}", boosts().size().await?),
        format!("NotificationStore: {}", NotificationStore::size()),
        format!("UserNotificationsStore: {}", UserNotificationStore::size()),
        format!("TopicStore: {}", topics().size().await?),
    ])
}
