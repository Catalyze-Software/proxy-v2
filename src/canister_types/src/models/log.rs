use candid::{CandidType, Deserialize, Principal};

use crate::impl_storable_for;

pub type Timestamp = u64;
pub type LogId = u64;

impl_storable_for!(Logger);

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct Logger {
    pub description: String,
    pub source: Option<String>,
    pub principal: Option<Principal>,
    pub data: Option<String>,
    pub created_on: Timestamp,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct PostLog {
    pub log_type: LogType,
    pub description: String,
    pub source: Option<String>,
    pub data: Option<String>,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct HumanReadableLogger {
    pub id: LogId,
    pub description: String,
    pub source: Option<String>,
    pub principal: Option<String>,
    pub data: Option<String>,
    pub created_on: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum LogType {
    Error,
    Warning,
    Info,
}

impl Logger {
    /// Create a new log from a post log
    /// # Arguments
    /// * `post_log` - The post log to create the log from
    /// # Returns
    /// * `Log` - The log created from the post log
    pub fn from_post_log(post_log: PostLog) -> Self {
        let log_type = match post_log.log_type {
            LogType::Error => "Error",
            LogType::Warning => "Warning",
            LogType::Info => "Info",
        };

        let description = format!("[{}] {}", log_type, post_log.description);

        Logger {
            description,
            source: post_log.source,
            data: post_log.data,
            principal: None,
            created_on: ic_cdk::api::time(),
        }
    }

    /// Create a new log from a post log with the caller
    /// # Arguments
    /// * `post_log` - The post log to create the log from
    /// # Returns
    /// * `Log` - The log created from the post log
    pub fn from_post_log_with_caller(post_log: PostLog) -> Self {
        let post_log = Logger::from_post_log(post_log);
        let principal = ic_cdk::caller();

        Logger {
            principal: Some(principal),
            ..post_log
        }
    }
}
