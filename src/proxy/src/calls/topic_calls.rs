use crate::{helpers::guards::is_developer, logic::topic_logic::TopicCalls};
use catalyze_shared::{
    topic::{TopicEntry, TopicKind},
    CanisterResult,
};
use ic_cdk::{query, update};

/// Add a topic to the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `value` - The topic to add
/// # Returns
/// * `Topic` - The added topic
/// # Errors
/// * `ApiError` - If something went wrong while adding the topic
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn add_topic(kind: TopicKind, value: String) -> CanisterResult<TopicEntry> {
    TopicCalls::add(kind, value).await
}

/// Remove a topic from the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `id` - The topic id to remove
/// # Returns
/// * `bool` - if the removal was successful
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn remove_topic(id: u64) -> CanisterResult<bool> {
    TopicCalls::remove(id).await
}

/// Add many topics to the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `Vec<value>` - The topics to add
/// # Returns
/// * `Result<Topic, ApiError>` - The result for each added topic
/// # Errors
/// * `ApiError` - If something went wrong while adding the topic
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn add_topics(kind: TopicKind, value: Vec<String>) -> CanisterResult<Vec<TopicEntry>> {
    TopicCalls::add_many(kind, value).await
}

/// Get a topic - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `id` - The identifier of the topic
/// # Returns
/// * `Topic` - The topic
/// # Errors
/// * `ApiError` - If something went wrong while getting the topic
#[query(composite = true)]
pub async fn get_topic(id: u64) -> CanisterResult<TopicEntry> {
    TopicCalls::get(id).await
}

/// Get topics by their identifiers and kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Category, Skill)
/// * `ids` - The identifiers of the topics
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query(composite = true)]
pub async fn get_topics(ids: Vec<u64>) -> CanisterResult<Vec<TopicEntry>> {
    TopicCalls::get_many(ids).await
}

/// Get all topics by kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Category, Skill)
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query(composite = true)]
pub async fn get_all_topics(kind: TopicKind) -> CanisterResult<Vec<TopicEntry>> {
    TopicCalls::get_all(kind).await
}
