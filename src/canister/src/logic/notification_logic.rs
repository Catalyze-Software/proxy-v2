use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    friend_request::FriendRequest,
    notification::{
        GroupNotificationType, Notification, NotificationType, RelationNotificationType,
    },
    user_notifications::UserNotifications,
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

    // Friend request notifications

    /// Send a friend request notification to the receiver
    ///
    /// This is a actionable notification to accept or decline the friend request
    pub fn notification_add_friend_request(friend_request: FriendRequest) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_notification(
            vec![friend_request.to],
            NotificationType::Relation(RelationNotificationType::FriendRequest(friend_request)),
            true,
            false,
        )?;

        Ok(notification_id)
    }

    /// Notification that gets send when a friend request is accepted or declined
    ///
    /// Based on the `is_accepted` parameter, the notification is either a friend request accept or silent decline
    pub fn notification_accept_or_decline_friend_request(
        friend_request_id: u64,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        // get the associated friend request
        let (_, friend_request) = FriendRequestStore::get(friend_request_id)?;

        // check if the notification exists
        if let Some(notification_id) = friend_request.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            // check if the notification is a friend request
            if let NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request,
            )) = &notification.notification_type.clone()
            {
                // mark the notification as accepted, this also marks it as not actionable
                notification.mark_as_accepted(is_accepted);
                let _ = NotificationStore::update(notification_id, notification.clone());

                let notification_type = match is_accepted {
                    true => RelationNotificationType::FriendRequestAccept(friend_request_id),
                    false => RelationNotificationType::FriendRequestDecline(friend_request_id),
                };

                // add the notification to the user's notifications and send a websocket message
                let _ = Self::add_notification(
                    vec![friend_request.requested_by],
                    NotificationType::Relation(notification_type),
                    false,
                    // If the friend request is accepted, the notification is not silent
                    !is_accepted,
                );

                Ok(())
            } else {
                Err(ApiError::bad_request().add_message("Notification is not a friend request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    /// Silent notification that gets send when a friend request is removed
    ///
    /// This is a silent notification to remove the friend request from the user's notifications
    pub fn notification_remove_friend_request(friend_request_id: u64) -> Result<(), ApiError> {
        // get the associated friend request
        let (_, friend_request) = FriendRequestStore::get(friend_request_id)?;

        // check if the notification exists
        if let Some(notification_id) = friend_request.notification_id {
            // get the associated notification and check if it is a friend request
            let (_, notification) = NotificationStore::get(notification_id)?;
            if let NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request,
            )) = &notification.notification_type
            {
                // check if the notification is for the caller
                if friend_request.requested_by != caller() {
                    return Err(ApiError::unauthorized());
                }

                let _ = Self::add_notification(
                    vec![friend_request.requested_by, friend_request.to],
                    NotificationType::Relation(RelationNotificationType::FriendRequestRemove(
                        friend_request_id,
                    )),
                    false,
                    true,
                );
                Ok(())
            } else {
                Err(ApiError::bad_request().add_message("Notification is not a friend request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    pub fn notification_remove_friend(
        receiver: Principal,
        friend_principal: Principal,
    ) -> Result<(), ApiError> {
        Self::add_notification(
            vec![receiver],
            NotificationType::Relation(RelationNotificationType::FriendRemove(friend_principal)),
            false,
            true,
        )?;

        Ok(())
    }

    // Group notifications
    /// This notification is send when a user joins a group
    ///
    /// This is a silent update to update the members in real-time for the group members (receivers)
    pub fn notification_join_group(
        receivers: Vec<Principal>,
        group_id: u64,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_notification(
            receivers,
            NotificationType::Group(GroupNotificationType::UserJoinGroup(group_id)),
            false,
            true,
        )?;

        Ok(notification_id)
    }

    /// This notification is send when a user gets invites to the groups
    ///
    /// This is a actionable notification to accept or decline the invite to the group
    pub fn notification_invite_to_group(
        invitee_principal: Principal,
        group_id: u64,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_notification(
            vec![invitee_principal],
            NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(group_id)),
            true,
            false,
        )?;

        Ok(notification_id)
    }

    /// Notification that gets send when a friend request is accepted or declined
    ///
    /// Based on the `is_accepted` parameter, the notification is either a friend request accept or silent decline
    pub fn notification_invite_to_group_accept_or_decline(
        member_principal: Principal,
        group_id: u64,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        // get the associated friend request
        let (_, member) = MemberStore::get(member_principal)?;
        let invite = member.get_invite(&group_id);

        // check if the invite exists
        if let Some(_invite) = invite {
            // check if the notification exists
            if let Some(notification_id) = _invite.notification_id {
                let (_, mut notification) = NotificationStore::get(notification_id)?;

                // check if the notification is a friend request
                if let NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                    group_id,
                )) = &notification.notification_type.clone()
                {
                    // mark the notification as accepted, this also marks it as not actionable
                    notification.mark_as_accepted(is_accepted);
                    let _ = NotificationStore::update(notification_id, notification.clone());

                    let notification_type = match is_accepted {
                        true => GroupNotificationType::JoinGroupOwnerRequestAccept(*group_id),
                        false => GroupNotificationType::JoinGroupOwnerRequestDecline(*group_id),
                    };

                    // add the notification to the user's notifications and send a websocket message
                    let _ = Self::add_notification(
                        vec![member_principal],
                        NotificationType::Group(notification_type),
                        false,
                        false,
                    );
                }
                Ok(())
            } else {
                Err(ApiError::bad_request()
                    .add_message("Notification is not a group invite request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    pub fn notification_join_request_to_group(
        receivers: Vec<Principal>,
        group_id: u64,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_notification(
            receivers,
            NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(group_id)),
            true,
            false,
        )?;

        Ok(notification_id)
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
