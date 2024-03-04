use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    notification::{Notification, NotificationType},
    unread_count::UnreadNotifications,
};

use crate::storage::{NotificationStore, StorageMethods, UnreadNotificationStore};

pub struct NotificationCalls;

impl NotificationCalls {
    pub fn add_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        let notification = Notification::new(receivers.clone(), notification_type, is_actionable);
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        for receiver in receivers {
            let (_, mut unread_notifications) = UnreadNotificationStore::get(receiver)
                .unwrap_or((receiver, UnreadNotifications::new()));

            unread_notifications.add(new_notification_id.clone());
            let _ = UnreadNotificationStore::update(receiver, unread_notifications);
        }

        Ok((new_notification_id, new_notification))
    }

    pub fn get_unread_notifications(
        principal: Principal,
    ) -> Result<Vec<(u64, Notification)>, ApiError> {
        let (_, unread_notification_ids) = UnreadNotificationStore::get(principal)?;
        Ok(NotificationStore::get_many(
            unread_notification_ids.to_vec(),
        ))
    }

    pub fn get_all_notifications(principal: &Principal) -> Vec<(u64, Notification)> {
        let mut notifications = NotificationStore::filter(|id, notification| {
            notification.receivers.contains(principal)
        });

        notifications.sort_by(|(_, a), (_, b)| b.created_at.cmp(&a.created_at));

        notifications
    }

    pub fn read_notifications(principal: Principal, ids: Vec<u64>) -> Result<Vec<u64>, ApiError> {
        let (_, mut unread_notifications) = UnreadNotificationStore::get(principal)?;

        // We only store unread notification referenced, so we can remove the read ones
        for id in ids {
            unread_notifications.remove(id);
        }

        Ok(unread_notifications.to_vec())
    }
}
