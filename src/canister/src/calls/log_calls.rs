use crate::storage::LoggerStore;
use canister_types::models::{
    api_error::ApiError,
    logger::{LogType, Logger, PostLog},
};
use ic_cdk::{query, update};

#[update]
fn log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log(post_log)
}

#[update]
fn log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log_with_caller(post_log)
}

#[query]
fn get_latest_logs(count: u64) -> Result<Vec<Logger>, ApiError> {
    LoggerStore::get_latest_logs(count)
}

#[update]
fn fill_logs(n: u64) {
    for i in 1..n {
        let post_log = PostLog {
            log_type: LogType::Info,
            description: i.to_string(),
            source: None,
            data: None,
        };

        LoggerStore::new_from_post_log(post_log).expect("Failed to create log");
    }
}

#[query]
fn size() -> u64 {
    LoggerStore::size()
}
