use canister_types::models::{api_error::ApiError, notification::Notification};
use ic_cdk::{caller, query, update};

use crate::logic::notification_logic::NotificationCalls;

#[query]
fn get_notifications() -> Vec<(u64, Notification)> {
    NotificationCalls::get_user_notifications(caller())
}

#[query]
fn get_unread_notifications() -> Result<Vec<(u64, Notification)>, ApiError> {
    NotificationCalls::get_user_unread_notifications(caller())
}

#[update]
fn mark_notifications_as_read(ids: Vec<u64>, is_read: bool) -> Result<Vec<(u64, bool)>, ApiError> {
    NotificationCalls::mark_notifications_as_read(caller(), ids, is_read)
}

#[update]
fn remove_notifications(ids: Vec<u64>) -> Vec<(u64, bool)> {
    NotificationCalls::remove_notifications(caller(), ids)
}
