use canister_types::models::{api_error::ApiError, notification::Notification};
use ic_cdk::{caller, query, update};

use crate::logic::notification_logic::NotificationCalls;

#[query]
fn get_notifications() -> Vec<(u64, Notification)> {
    NotificationCalls::get_all_notifications(&caller())
}

#[query]
fn get_unread_notifications() -> Result<Vec<(u64, Notification)>, ApiError> {
    NotificationCalls::get_unread_notifications(caller())
}

#[update]
fn read_notifications(ids: Vec<u64>) -> Result<Vec<u64>, ApiError> {
    NotificationCalls::read_notifications(caller(), ids)
}

// #[update]
// fn remove_notifications(ids: Vec<u64>) -> Vec<(u64, bool)> {
//     Store::remove_notifications(&caller(), ids)
// }

// #[query]
// fn get_notification_count() -> u64 {
//     Store::get_notification_count()
// }

// #[update]
// fn clear_notifications() {
//     Store::clear_notifications();
// }
