use super::storage_api::TOPIC_CANISTER;
use candid::Principal;
use catalyze_shared::{
    topic::{Topic, TopicFilter, TopicSort},
    StorageClient, StorageClientInsertable,
};

#[derive(Default)]
pub struct TopicStorageClient;

impl StorageClient<u64, Topic, TopicFilter, TopicSort> for TopicStorageClient {
    fn name(&self) -> String {
        "topics".to_string()
    }

    fn storage_canister_id(&self) -> catalyze_shared::StaticCellStorageRef<Principal> {
        &TOPIC_CANISTER
    }
}

impl StorageClientInsertable<Topic, TopicFilter, TopicSort> for TopicStorageClient {}

pub fn topics() -> impl StorageClientInsertable<Topic, TopicFilter, TopicSort> {
    TopicStorageClient
}
