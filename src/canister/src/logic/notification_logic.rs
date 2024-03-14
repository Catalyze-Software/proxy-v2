use std::clone;

use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    friend_request::{self, FriendRequest},
    member::{Member, MemberInvite},
    notification::{
        self, GroupNotificationType, Notification, NotificationType, RelationNotificationType,
    },
    user_notifications::{UserNotificationData, UserNotifications},
    websocket_message::WSMessage,
};
use ic_cdk::caller;

use crate::storage::{
    FriendRequestStore, MemberStore, NotificationStore, StorageMethods, UsernotificationStore,
};

use super::websocket_logic::Websocket;

pub struct NotificationCalls;

impl NotificationCalls {
    pub fn add_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        // add the notification reference to the user's notifications
        if let Ok((_, mut caller_notifications)) = UsernotificationStore::get(caller()) {
            caller_notifications.add(new_notification_id.clone(), false, true);
            let _ = UsernotificationStore::update(caller(), caller_notifications);
        } else {
            let mut caller_notifications = UserNotifications::new();
            caller_notifications.add(new_notification_id.clone(), false, true);
            let _ = UsernotificationStore::insert_by_key(caller(), caller_notifications);
        }

        // send the notification to the receivers
        for receiver in receivers {
            if let Ok((_, mut notifications)) = UsernotificationStore::get(caller()) {
                notifications.add(new_notification_id.clone(), false, true);
                let _ = UsernotificationStore::update(receiver, notifications);
            } else {
                let mut notifications = UserNotifications::new();
                notifications.add(new_notification_id.clone(), false, true);
                let _ = UsernotificationStore::insert_by_key(receiver, notifications);
            }
        }

        Ok((new_notification_id, new_notification))
    }

    pub fn send_notification(
        notification: Notification,
        receiver: Principal,
        is_silent: bool,
    ) -> () {
        if !is_silent {
            Websocket::send_message(receiver, WSMessage::Notification(notification.clone()));
        } else {
            Websocket::send_message(
                receiver,
                WSMessage::SilentNotification(notification.clone()),
            );
        }
    }

    pub fn add_and_send_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
        is_silent: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        // add the notification reference to the user's notifications
        if let Ok((_, mut caller_notifications)) = UsernotificationStore::get(caller()) {
            caller_notifications.add(new_notification_id.clone(), false, true);
            let _ = UsernotificationStore::update(caller(), caller_notifications);
        } else {
            let mut caller_notifications = UserNotifications::new();
            caller_notifications.add(new_notification_id.clone(), false, true);
            let _ = UsernotificationStore::insert_by_key(caller(), caller_notifications);
        }

        // send the notification to the receivers
        for receiver in receivers {
            if let Ok((_, mut notifications)) = UsernotificationStore::get(caller()) {
                notifications.add(new_notification_id.clone(), false, true);
                let _ = UsernotificationStore::update(receiver, notifications);
            } else {
                let mut notifications = UserNotifications::new();
                notifications.add(new_notification_id.clone(), false, true);
                let _ = UsernotificationStore::insert_by_key(receiver, notifications);
            }

            // send the notification to the receiver
            Self::send_notification(new_notification.clone(), receiver, is_silent);
        }

        Ok((new_notification_id, new_notification))
    }

    // Friend request notifications

    /// stores + sends notification
    pub fn notification_add_friend_request(friend_request: FriendRequest) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_notification(
            vec![friend_request.to],
            NotificationType::Relation(RelationNotificationType::FriendRequest(friend_request)),
            true,
        )?;

        Self::send_notification(notification, friend_request.to, false);
        Ok(notification_id)
    }

    /// stores + sends notification
    pub fn notification_accept_or_decline_friend_request(
        friend_request_data: (u64, FriendRequest),
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        // get the associated friend request
        let (friend_request_id, friend_request) = friend_request_data;

        // check if the notification exists
        if let Some(notification_id) = friend_request.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            // check if the notification is a friend request
            if let NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request,
            )) = &notification.notification_type
            {
                // mark the notification as accepted, this also marks it as not actionable
                notification.mark_as_accepted(is_accepted);
                let _ = NotificationStore::update(notification_id, notification.clone());

                let notification_type = match is_accepted {
                    true => RelationNotificationType::FriendRequestAccept(friend_request_id),
                    false => RelationNotificationType::FriendRequestDecline(friend_request_id),
                };

                Self::send_notification(notification, friend_request.requested_by, false);
                Self::send_notification(notification, friend_request.to, true);

                Ok(())
            } else {
                Err(ApiError::bad_request().add_message("Notification is not a friend request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    // sends notification
    pub fn notification_remove_friend_request(
        receiver: Principal,
        friend_request_id: u64,
    ) -> Result<(), ApiError> {
        Self::send_notification(
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRequestRemove(
                    friend_request_id,
                )),
                false,
            ),
            receiver,
            true,
        );
        Ok(())
    }

    // sends notification
    pub fn notification_remove_friend(receiver: Principal, friend_principal: Principal) -> () {
        Self::send_notification(
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRemove(
                    friend_principal,
                )),
                false,
            ),
            receiver,
            true,
        );
    }

    // Group notifications

    // sends notification
    pub fn notification_join_public_group(receivers: Vec<Principal>, group_id: u64) -> () {
        for receiver in receivers {
            Self::send_notification(
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserJoinGroup(group_id)),
                    false,
                ),
                receiver,
                true,
            );
        }
    }

    // stores + sends notification
    pub fn notification_user_join_request_group(
        receivers: Vec<Principal>,
        group_id: u64,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            receivers,
            NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(group_id)),
            true,
            false,
        )?;

        Ok(notification_id)
    }

    // sends notifications
    pub fn notification_user_join_request_group_accept_or_decline(
        receivers: Vec<Principal>,
        invite: MemberInvite,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(group_id)) =
                notification.notification_type.clone()
            {
                notification.mark_as_accepted(is_accepted);
                let _ = NotificationStore::update(notification_id, notification.clone());

                let notification_type = match is_accepted {
                    true => GroupNotificationType::JoinGroupUserRequestAccept(group_id),
                    false => GroupNotificationType::JoinGroupUserRequestDecline(group_id),
                };

                // send notification to the person who requested to join
                let _ = Self::send_notification(
                    notification,
                    notification.sender, // the person who request to join
                    false,
                );

                // send notification to the users who could have accepted the request
                for r in receivers {
                    let _ = Self::send_notification(notification.clone(), r, true);
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // stores + sends notification
    pub fn notification_owner_join_request_group(
        invitee_principal: Principal,
        group_id: u64,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            vec![invitee_principal],
            NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(group_id)),
            true,
            false,
        )?;

        Ok(notification_id)
    }

    pub fn notification_owner_join_request_group_accept_or_decline(
        invitee_principal: Principal,
        invite: MemberInvite,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(group_id)) =
                notification.notification_type.clone()
            {
                notification.mark_as_accepted(is_accepted);
                let _ = NotificationStore::update(notification_id, notification.clone());

                let notification_type = match is_accepted {
                    true => GroupNotificationType::JoinGroupOwnerRequestAccept(group_id),
                    false => GroupNotificationType::JoinGroupOwnerRequestDecline(group_id),
                };

                // send notification to the person who requested to join
                let _ = Self::send_notification(
                    notification,
                    notification.sender, // the person who requested the user to join
                    false,
                );

                // send notification to the users who could have accepted the request
                Self::send_notification(notification.clone(), invitee_principal, true);
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    pub fn get_user_unread_notifications(
        principal: Principal,
    ) -> Result<Vec<(u64, Notification)>, ApiError> {
        let (_, unread_notification_ids) = UsernotificationStore::get(principal)?;
        Ok(NotificationStore::get_many(unread_notification_ids.ids()))
    }

    pub fn get_user_notification_ids(principal: Principal) -> Vec<u64> {
        let (_, notification_ids) =
            UsernotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        notification_ids.ids()
    }

    pub fn get_user_notifications(principal: Principal) -> Vec<(u64, Notification)> {
        NotificationStore::get_many(Self::get_user_notification_ids(principal))
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
    ) -> Result<Vec<(u64, UserNotificationData)>, ApiError> {
        let (_, mut user_notifications) = UsernotificationStore::get(principal)?;
        user_notifications.mark_as_read_many(ids, is_read);
        Ok(user_notifications.to_vec())
    }

    pub fn remove_notifications(
        principal: Principal,
        ids: Vec<u64>,
    ) -> Vec<(u64, UserNotificationData)> {
        let (_, mut user_notifications) =
            UsernotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        user_notifications.remove_many(ids);
        let _ = UsernotificationStore::update(principal, user_notifications.clone());
        user_notifications.to_vec()
    }
}
