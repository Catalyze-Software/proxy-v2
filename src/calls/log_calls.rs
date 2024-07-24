use crate::storage::LoggerStore;
use crate::{
    helpers::guards::{has_access, is_developer, is_monitor},
    logic::logger_logic::LoginEvent,
};
use catalyze_shared::{
    guards::is_not_anonymous,
    log::{LogType, Logger, PostLog},
    CanisterResult,
};
use ic_cdk::{query, update};

// Update functions
#[update(guard = "is_not_anonymous")]
async fn log(post_log: PostLog) -> CanisterResult<(u64, Logger)> {
    has_access().await?;
    LoggerStore::new_from_post_log(post_log)
}

#[update(guard = "is_not_anonymous")]
async fn log_with_caller(post_log: PostLog) -> CanisterResult<(u64, Logger)> {
    has_access().await?;
    LoggerStore::new_from_post_log_with_caller(post_log)
}

#[update(guard = "is_not_anonymous")]
async fn log_login() -> CanisterResult<(u64, Logger)> {
    has_access().await?;
    LoginEvent::log_event()
}

#[update(guard = "is_developer")]
fn test_log() {
    let log = PostLog {
        log_type: LogType::Info,
        description: "Test log".to_string(),
        source: None,
        data: None,
    };
    LoggerStore::new_from_post_log_with_caller(log).expect("Failed to log test log");
}

// Query functions
#[query(guard = "is_developer")]
fn get_latest_logs(count: u64) -> Vec<Logger> {
    LoggerStore::get_latest_logs(count)
}

#[query(guard = "is_monitor")]
fn log_size() -> u64 {
    LoggerStore::size()
}
