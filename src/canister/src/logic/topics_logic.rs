use std::fmt::{Display, Formatter};

use canister_types::models::{
    api_error::ApiError,
    validation::{ValidateField, ValidationType},
};

use crate::{
    helpers::validator::Validator,
    storage::{InterestsStore, SkillsStore, StorageInsertable, StorageQueryable, TagsStore},
};

#[derive(Clone, Debug)]
pub enum TopicKind {
    Tag,
    Interest,
    Skill,
}

impl Display for TopicKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicKind::Tag => write!(f, "tag"),
            TopicKind::Interest => write!(f, "interest"),
            TopicKind::Skill => write!(f, "skill"),
        }
    }
}

pub struct TopicsCalls;

impl TopicsCalls {
    pub fn add(kind: TopicKind, topic: String) -> Result<(u64, String), ApiError> {
        let topic = handle_topic(kind.clone(), topic)?;

        match kind {
            TopicKind::Tag => TagsStore::insert(topic),
            TopicKind::Interest => InterestsStore::insert(topic),
            TopicKind::Skill => SkillsStore::insert(topic),
        }
    }

    pub fn get_all(kind: TopicKind) -> Result<Vec<(u64, String)>, ApiError> {
        let result = match kind {
            TopicKind::Tag => TagsStore::get_all(),
            TopicKind::Interest => InterestsStore::get_all(),
            TopicKind::Skill => SkillsStore::get_all(),
        };

        Ok(result)
    }

    pub fn get(kind: TopicKind, id: u64) -> Result<(u64, String), ApiError> {
        match kind {
            TopicKind::Tag => TagsStore::get(id),
            TopicKind::Interest => InterestsStore::get(id),
            TopicKind::Skill => SkillsStore::get(id),
        }
    }

    pub fn get_many(kind: TopicKind, ids: Vec<u64>) -> Result<Vec<(u64, String)>, ApiError> {
        let result = match kind {
            TopicKind::Tag => TagsStore::get_many(ids),
            TopicKind::Interest => InterestsStore::get_many(ids),
            TopicKind::Skill => SkillsStore::get_many(ids),
        };

        Ok(result)
    }
}

const TOPIC_MAX_LENGTH: usize = 32;
const TOPIC_MIN_LENGTH: usize = 1;

fn handle_topic(kind: TopicKind, topic: String) -> Result<String, ApiError> {
    let topic = topic.to_lowercase();
    let topic = topic.trim();

    Validator::new(vec![ValidateField(
        ValidationType::StringLength(topic.to_string(), TOPIC_MIN_LENGTH, TOPIC_MAX_LENGTH),
        kind.to_string(),
    )])
    .validate()?;

    let existing = match kind {
        TopicKind::Tag => TagsStore::find(|_, value| value == topic),
        TopicKind::Interest => InterestsStore::find(|_, value| value == topic),
        TopicKind::Skill => SkillsStore::find(|_, value| value == topic),
    };

    if let Some(existing) = existing {
        return Err(ApiError::duplicate()
            .add_tag(&kind.to_string())
            .add_message(&format!(
                "{kind} \"{}\" already exists with ID: {}",
                existing.1, existing.0
            )));
    }

    Ok(topic.to_string())
}
