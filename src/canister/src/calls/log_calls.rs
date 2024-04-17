use canister_types::models::{
    api_error::ApiError,
    logger::{Logger, PostLog},
};
use ic_cdk::update;

use crate::storage::LoggerStore;

#[update]
fn log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log(post_log)
}

#[update]
fn log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log_with_caller(post_log)
}
