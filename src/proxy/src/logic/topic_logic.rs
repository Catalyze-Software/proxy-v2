use catalyze_shared::{
    api_error::ApiError,
    topic::{Topic, TopicEntry, TopicFilter, TopicKind},
    validation::{ValidateField, ValidationType},
    validator::Validator,
    CanisterResult, StorageClient, StorageClientInsertable,
};

use crate::storage::topics;

pub struct TopicCalls;

impl TopicCalls {
    pub async fn add(kind: TopicKind, topic: String) -> CanisterResult<TopicEntry> {
        let topic = handle_topic(kind.clone(), topic).await?;
        topics().insert(Topic::new(kind, topic)).await
    }

    pub async fn add_many(kind: TopicKind, list: Vec<String>) -> CanisterResult<Vec<TopicEntry>> {
        let mut mapped: Vec<Topic> = vec![];

        for topic in list {
            let topic = handle_topic(kind.clone(), topic).await?;
            mapped.push(Topic::new(kind.clone(), topic));
        }

        topics().insert_many(mapped).await
    }

    pub async fn get_all(kind: TopicKind) -> CanisterResult<Vec<TopicEntry>> {
        topics().filter(TopicFilter::Kind(kind).into()).await
    }

    pub async fn get(id: u64) -> CanisterResult<TopicEntry> {
        topics().get(id).await
    }

    pub async fn get_many(ids: Vec<u64>) -> CanisterResult<Vec<TopicEntry>> {
        topics().get_many(ids).await
    }

    pub async fn remove(id: u64) -> CanisterResult<bool> {
        topics().remove(id).await
    }
}

const TOPIC_MAX_LENGTH: usize = 32;
const TOPIC_MIN_LENGTH: usize = 1;

async fn handle_topic(kind: TopicKind, topic: String) -> CanisterResult<String> {
    let topic = topic.to_lowercase();
    let topic = topic.trim().to_owned();

    Validator::new(vec![ValidateField(
        ValidationType::StringLength(topic.clone(), TOPIC_MIN_LENGTH, TOPIC_MAX_LENGTH),
        kind.to_string(),
    )])
    .validate()?;

    let filters = vec![
        TopicFilter::Kind(kind.clone()),
        TopicFilter::Value(topic.clone()),
    ];

    if let Some((id, existing)) = topics().find(filters).await? {
        return Err(ApiError::duplicate()
            .add_tag(kind.to_string())
            .add_message(format!(
                "{kind} \"{}\" already exists with ID: {}",
                existing.value, id
            )));
    }

    Ok(topic.to_string())
}
