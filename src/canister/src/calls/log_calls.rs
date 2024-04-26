use crate::storage::LoggerStore;
use crate::{helpers::guards::has_access, logic::logger_logic::LoginEvent};
use canister_types::models::{
    api_error::ApiError,
    log::{Logger, PostLog},
};
use ic_cdk::{query, update};

// Update functions
#[update(guard = "has_access")]
fn log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log(post_log)
}

#[update(guard = "has_access")]
fn log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log_with_caller(post_log)
}

#[update(guard = "has_access")]
fn log_login() -> Result<(u64, Logger), ApiError> {
    LoginEvent::new()
}

// Query functions
#[query(guard = "has_access")]
fn get_latest_logs(count: u64) -> Vec<Logger> {
    LoggerStore::get_latest_logs(count)
}

#[query(guard = "has_access")]
fn log_size() -> u64 {
    LoggerStore::size()
}
