use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    notification::{Notification, NotificationType},
    user_notifications::UserNotifications,
    websocket_message::WSMessage,
};

use crate::storage::{NotificationStore, StorageMethods, UsernotificationStore};

use super::websocket_logic::Websocket;

pub struct NotificationCalls;

impl NotificationCalls {
    pub fn add_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
        is_silent: bool,
    ) -> Result<(u64, Notification), ApiError> {
        let notification = Notification::new(notification_type, is_actionable);
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        for receiver in receivers {
            let (_, mut user_notifications) = UsernotificationStore::get(receiver)
                .unwrap_or((receiver, UserNotifications::new()));

            user_notifications.add(new_notification_id.clone(), false);
            let _ = UsernotificationStore::update(receiver, user_notifications);

            if !is_silent {
                Websocket::send_message(
                    receiver,
                    WSMessage::Notification(new_notification.clone()),
                );
            } else {
                Websocket::send_message(
                    receiver,
                    WSMessage::SilentNotification(new_notification.clone()),
                );
            }
        }

        Ok((new_notification_id, new_notification))
    }

    pub fn get_user_unread_notifications(
        principal: Principal,
    ) -> Result<Vec<(u64, Notification)>, ApiError> {
        let (_, unread_notification_ids) = UsernotificationStore::get(principal)?;
        Ok(NotificationStore::get_many(unread_notification_ids.ids()))
    }

    pub fn get_user_notifaction_ids(principal: Principal) -> Vec<u64> {
        let (_, notification_ids) =
            UsernotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        notification_ids.ids()
    }

    pub fn get_user_notifications(principal: Principal) -> Vec<(u64, Notification)> {
        NotificationStore::get_many(Self::get_user_notifaction_ids(principal))
    }

    pub fn set_notification_as_accepted(
        principal: Principal,
        notification_id: u64,
        is_accepted: bool,
    ) -> Result<Notification, ApiError> {
        if let Some((_, mut notification)) = Self::get_user_notifications(principal)
            .into_iter()
            .find(|(id, _)| id == &notification_id)
        {
            if !notification.is_actionable {
                return Err(ApiError::bad_request().add_message("Notification is not actionable"));
            }

            notification.mark_as_accepted(is_accepted);
            let _ = NotificationStore::update(notification_id, notification.clone());
            Ok(notification)
        } else {
            Err(ApiError::not_found())
        }
    }

    pub fn mark_notifications_as_read(
        principal: Principal,
        ids: Vec<u64>,
        is_read: bool,
    ) -> Result<Vec<(u64, bool)>, ApiError> {
        let (_, mut user_notifications) = UsernotificationStore::get(principal)?;
        user_notifications.mark_as_read_many(ids, is_read);
        Ok(user_notifications.to_vec())
    }

    pub fn remove_notifications(principal: Principal, ids: Vec<u64>) -> Vec<(u64, bool)> {
        let (_, mut user_notifications) =
            UsernotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        user_notifications.remove_many(ids);
        let _ = UsernotificationStore::update(principal, user_notifications.clone());
        user_notifications.to_vec()
    }
}
