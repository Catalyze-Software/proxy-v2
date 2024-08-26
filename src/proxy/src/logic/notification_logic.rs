use std::collections::HashMap;

use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    attendee::{AttendeeInvite, InviteAttendeeResponse, JoinedAttendeeResponse},
    friend_request::{FriendRequest, FriendRequestResponse},
    notification::{
        EventNotificationType, GroupNotificationType, MultisigNotificationType, Notification,
        NotificationResponse, NotificationType, RelationNotificationType,
        TransactionNotificationType,
    },
    old_member::{InviteMemberResponse, JoinedMemberResponse, MemberInvite},
    profile_with_refs::ProfileWithRefs,
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::{UserNotificationData, UserNotifications},
    websocket_message::WSMessage,
    StorageClient, StorageClientInsertable,
};
use ic_cdk::caller;

use crate::{
    storage::{notifications, profiles},
    MULTISIG_INDEX,
};

use super::websocket_logic::Websocket;

pub struct NotificationCalls;

impl NotificationCalls {
    // Friend request notifications

    /// stores + sends notification
    pub async fn notification_add_friend_request(
        friend_request: FriendRequestResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_notification(
            vec![friend_request.to],
            NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request.clone(),
            )),
            true,
        )
        .await?;

        Self::send_notification(Some(notification_id), notification, friend_request.to).await;
        Ok(notification_id)
    }

    /// stores + sends notification
    pub async fn notification_accept_or_decline_friend_request(
        friend_request_data: (u64, FriendRequest),
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        // get the associated friend request
        let (_, friend_request) = friend_request_data;

        // check if the notification exists
        if let Some(notification_id) = friend_request.notification_id {
            let (_, mut notification) = notifications().get(notification_id).await?;

            // check if the notification is a friend request
            if let NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request,
            )) = &notification.notification_type.clone()
            {
                // mark the notification as accepted, this also marks it as not actionable
                let notification_type = match is_accepted {
                    true => RelationNotificationType::FriendRequestAccept(friend_request.clone()),
                    false => RelationNotificationType::FriendRequestDecline(friend_request.clone()),
                };
                notification
                    .mark_as_accepted(is_accepted, NotificationType::Relation(notification_type));
                let _ = notifications()
                    .update(notification_id, notification.clone())
                    .await;

                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    friend_request.requested_by,
                )
                .await;
                Self::send_notification(None, notification, friend_request.to).await;

                Ok(())
            } else {
                Err(ApiError::bad_request().add_message("Notification is not a friend request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    // sends notification
    pub async fn notification_remove_friend_request(receiver: Principal, friend_request_id: u64) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRequestRemove(
                    friend_request_id,
                )),
                false,
            ),
            receiver,
        )
        .await;
    }

    // sends notification
    pub async fn notification_remove_friend(receiver: Principal, friend_principal: Principal) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRemove(
                    friend_principal,
                )),
                false,
            ),
            receiver,
        )
        .await;
    }

    // Group notifications

    // sends notification
    pub async fn notification_join_public_group(receivers: Vec<Principal>, group_id: u64) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserJoinGroup(group_id)),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    pub async fn notification_leave_group(receivers: Vec<Principal>, group_id: u64) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserLeaveGroup(group_id)),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    // stores + sends notification
    pub async fn notification_user_join_request_group(
        receivers: Vec<Principal>,
        invite_member_response: InviteMemberResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            receivers,
            NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(
                invite_member_response,
            )),
            true,
        )
        .await?;

        Ok(notification_id)
    }

    // sends notifications
    pub async fn notification_user_join_request_group_accept_or_decline(
        invite: MemberInvite,
        is_accepted: bool,
        group_members: Vec<Principal>,
        higher_role_members: Vec<Principal>,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = notifications().get(notification_id).await?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(
                invite_member_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        GroupNotificationType::JoinGroupUserRequestAccept(invite_member_response)
                    }
                    false => {
                        GroupNotificationType::JoinGroupUserRequestDecline(invite_member_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Group(notification_type));
                let _ = notifications()
                    .update(notification_id, notification.clone())
                    .await;

                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    notification.sender, // the person who request to join
                )
                .await;

                match is_accepted {
                    true => {
                        for r in higher_role_members {
                            Self::send_notification(None, notification.clone(), r).await;
                        }
                    }
                    false => {
                        for r in group_members {
                            Self::send_notification(None, notification.clone(), r).await;
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // stores + sends notification
    pub async fn notification_owner_join_request_group(
        invitee_principal: Principal,
        invite_member_response: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_and_send_notification(
            vec![invitee_principal],
            NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                invite_member_response,
            )),
            true,
        )
        .await?;

        for r in receivers {
            Self::send_notification(None, notification.clone(), r).await;
        }

        Ok(notification_id)
    }

    // sends notification
    pub async fn notification_owner_join_request_group_accept_or_decline(
        invitee_principal: Principal,
        invite: MemberInvite,
        is_accepted: bool,
        group_members: Vec<Principal>,
        higher_role_members: Vec<Principal>,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = notifications().get(notification_id).await?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                invite_member_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        GroupNotificationType::JoinGroupOwnerRequestAccept(invite_member_response)
                    }
                    false => {
                        GroupNotificationType::JoinGroupOwnerRequestDecline(invite_member_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Group(notification_type));
                let _ = notifications()
                    .update(notification_id, notification.clone())
                    .await;

                match is_accepted {
                    true => {
                        Self::send_notification(None, notification.clone(), invitee_principal)
                            .await;
                        for r in group_members {
                            if notification.sender == r {
                                Self::send_notification(
                                    Some(notification_id),
                                    notification.clone(),
                                    r,
                                )
                                .await;
                            } else {
                                Self::send_notification(None, notification.clone(), r).await;
                            }
                        }
                    }
                    false => {
                        for r in higher_role_members {
                            if notification.sender == r {
                                Self::send_notification(
                                    Some(notification_id),
                                    notification.clone(),
                                    r,
                                )
                                .await;
                            } else {
                                Self::send_notification(None, notification.clone(), r).await;
                            }
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    pub async fn notification_change_group_member_role(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RoleAssignByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    pub async fn notification_remove_group_member(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Group(GroupNotificationType::RemoveMemberByOwner(member.clone())),
                false,
            ),
            member.principal,
        )
        .await;

        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RemoveMemberByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    pub async fn notification_remove_group_invite(
        invite: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) {
        if let Some(_invite) = invite.invite.clone() {
            if let Some(notification_id) = _invite.notification_id {
                if let Ok((_, mut notification)) = notifications().get(notification_id).await {
                    notification.mark_as_accepted(
                        false,
                        NotificationType::Group(GroupNotificationType::RemoveInviteByOwner(
                            invite.clone(),
                        )),
                    );
                    let _ = notifications()
                        .update(notification_id, notification.clone())
                        .await;

                    Self::send_notification(None, notification.clone(), invite.principal).await;

                    for receiver in receivers {
                        Self::send_notification(None, notification.clone(), receiver).await;
                    }
                }
            }
        }
    }

    // Event notifications

    // sends notification
    pub async fn notification_join_public_event(
        receivers: Vec<Principal>,
        group_id: u64,
        event_id: u64,
    ) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Event(EventNotificationType::UserJoinEvent((
                        group_id, event_id,
                    ))),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    // store + sends notification
    pub async fn notification_user_join_request_event(
        receivers: Vec<Principal>,
        invite_attendee_response: InviteAttendeeResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            receivers,
            NotificationType::Event(EventNotificationType::JoinEventUserRequest(
                invite_attendee_response,
            )),
            true,
        )
        .await?;

        Ok(notification_id)
    }

    // sends notifications
    pub async fn notification_user_join_request_event_accept_or_decline(
        receiver: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = notifications().get(notification_id).await?;

            if let NotificationType::Event(EventNotificationType::JoinEventUserRequest(
                invite_attendee_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        EventNotificationType::JoinEventUserRequestAccept(invite_attendee_response)
                    }
                    false => {
                        EventNotificationType::JoinEventUserRequestDecline(invite_attendee_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Event(notification_type));
                let _ = notifications()
                    .update(notification_id, notification.clone())
                    .await;

                // send notification to the users who could have accepted the request
                Self::send_notification(None, notification.clone(), receiver).await;

                if is_accepted {
                    for r in event_attendees {
                        if notification.sender == r {
                            Self::send_notification(Some(notification_id), notification.clone(), r)
                                .await;
                        } else {
                            Self::send_notification(None, notification.clone(), r).await;
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // sends notification
    pub async fn notification_owner_join_request_event_accept_or_decline(
        invitee_principal: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = notifications().get(notification_id).await?;

            if let NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                event_attendee_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        EventNotificationType::JoinEventOwnerRequestAccept(event_attendee_response)
                    }
                    false => {
                        EventNotificationType::JoinEventOwnerRequestDecline(event_attendee_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Event(notification_type));
                let _ = notifications()
                    .update(notification_id, notification.clone())
                    .await;

                // send notification to the users who could have accepted the request
                Self::send_notification(None, notification.clone(), invitee_principal).await;

                if is_accepted {
                    for r in event_attendees {
                        if notification.sender == r {
                            Self::send_notification(Some(notification_id), notification.clone(), r)
                                .await;
                        } else {
                            Self::send_notification(None, notification.clone(), r).await;
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // stores + sends notification
    pub async fn notification_owner_join_request_event(
        invitee_principal: Principal,
        invite_attendee_response: InviteAttendeeResponse,
        receivers: Vec<Principal>,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_and_send_notification(
            vec![invitee_principal],
            NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                invite_attendee_response,
            )),
            true,
        )
        .await?;

        for r in receivers {
            Self::send_notification(None, notification.clone(), r).await;
        }

        Ok(notification_id)
    }

    pub async fn notification_remove_event_invite(
        notification_id: u64,
        invite: InviteAttendeeResponse,
    ) {
        if let Ok((_, mut notification)) = notifications().get(notification_id).await {
            notification.mark_as_accepted(
                false,
                NotificationType::Event(EventNotificationType::RemoveInviteByOwner(invite.clone())),
            );
            let _ = notifications()
                .update(notification_id, notification.clone())
                .await;

            Self::send_notification(None, notification.clone(), invite.principal).await;
        }
    }

    pub async fn notification_remove_event_attendee(
        attendee: JoinedAttendeeResponse,
        receivers: Vec<Principal>,
    ) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Event(EventNotificationType::RemoveAttendeeByOwner(
                    attendee.clone(),
                )),
                false,
            ),
            attendee.principal,
        )
        .await;

        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Event(EventNotificationType::RemoveAttendeeByOwner(
                        attendee.clone(),
                    )),
                    false,
                ),
                receiver,
            )
            .await;
        }
    }

    // Transaction notifications
    pub async fn notification_add_transaction(transaction: TransactionData) -> bool {
        let _ = Self::add_and_send_notification(
            vec![transaction.receiver],
            NotificationType::Transaction(TransactionNotificationType::SingleTransaction(
                transaction,
            )),
            false,
        )
        .await;
        true
    }

    pub async fn notification_add_complete_transaction(data: TransactionCompleteData) -> bool {
        let _ = Self::add_and_send_notification(
            vec![data.sender],
            NotificationType::Transaction(TransactionNotificationType::TransactionsComplete(data)),
            false,
        )
        .await;
        true
    }

    pub async fn notification_add_multisig(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }
        let _ = Self::add_and_send_notification(
            receivers,
            NotificationType::Multisig(notification),
            false,
        )
        .await;
        true
    }

    pub async fn notification_add_multisig_silent(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }

        for r in receivers {
            Self::send_notification(
                None,
                Notification::new(NotificationType::Multisig(notification.clone()), false),
                r,
            )
            .await;
        }
        true
    }

    // sends notification
    pub async fn get_user_unread_notifications(principal: Principal) -> Vec<NotificationResponse> {
        let user_notifications = Self::get_user_notification_ids(principal).await;

        let notifications = notifications()
            .get_many(user_notifications.get_unread_ids())
            .await
            .unwrap_or_default();

        notifications
            .into_iter()
            .map(|(id, data)| {
                NotificationResponse::new(Some(id), data, user_notifications.get(&id))
            })
            .collect()
    }

    pub async fn get_user_notification_ids(principal: Principal) -> UserNotifications {
        let result = profiles().get(principal).await;
        match result {
            Ok(data) => data.1.references.notifications,
            Err(_) => UserNotifications::new(),
        }
    }

    pub async fn get_user_notifications(principal: Principal) -> Vec<NotificationResponse> {
        let user_notifications = Self::get_user_notification_ids(principal).await;

        let notifications = notifications()
            .get_many(user_notifications.ids())
            .await
            .unwrap_or_default();

        let mut notification_responses: Vec<NotificationResponse> = vec![];

        for (notification_id, notification_data) in notifications {
            notification_responses.push(NotificationResponse::new(
                Some(notification_id),
                notification_data,
                user_notifications.get(&notification_id),
            ));
        }

        notification_responses
    }

    pub async fn mark_notifications_as_read(
        principal: Principal,
        ids: Vec<u64>,
        is_read: bool,
    ) -> Result<Vec<(u64, UserNotificationData)>, ApiError> {
        let (_, mut profile) = profiles().get(principal).await?;

        profile
            .references
            .notifications
            .mark_as_read_many(ids, is_read);

        let _ = profiles().update(principal, profile.clone()).await;
        Ok(profile.references.notifications.to_vec())
    }

    pub async fn remove_user_notifications(
        principal: Principal,
        ids: Vec<u64>,
    ) -> Vec<(u64, UserNotificationData)> {
        match profiles().get(principal).await {
            Ok((_, mut profile)) => {
                profile.references.notifications.remove_many(ids);

                let _ = profiles().update(principal, profile.clone()).await;
                profile.references.notifications.to_vec()
            }
            Err(_) => vec![],
        }
    }

    pub async fn remove_all_user_notifications(
        principal: Principal,
    ) -> Vec<(u64, UserNotificationData)> {
        match profiles().get(principal).await {
            Ok((_, mut profile)) => {
                profile.references.notifications.clear();

                let _ = profiles().update(principal, profile.clone()).await;
                profile.references.notifications.to_vec()
            }
            Err(_) => vec![],
        }
    }

    pub async fn add_notification(
        receiver_principals: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = notifications().insert(notification).await?;

        if let Ok((principal, mut profile)) = profiles().get(caller()).await {
            profile
                .references
                .notifications
                .add(new_notification_id, false, true);

            profiles().update(principal, profile).await?;
        }

        let mut updated_receivers: HashMap<Principal, ProfileWithRefs> = HashMap::new();

        for (receiver_principal, mut receiver_profile) in
            profiles().get_many(receiver_principals.clone()).await?
        {
            receiver_profile
                .references
                .notifications
                .add(new_notification_id, false, true);

            updated_receivers.insert(receiver_principal, receiver_profile);
        }

        profiles()
            .update_many(updated_receivers.into_iter().collect())
            .await?;

        Ok((new_notification_id, new_notification))
    }

    pub async fn send_notification(
        // If the notification is silent, the notification id is not required as its not stored in the user's notifications
        notification_id: Option<u64>,
        notification: Notification,
        receiver: Principal,
    ) {
        let (_, user_notifications) = profiles()
            .get(receiver)
            .await
            .map(|(_, profile)| (receiver, profile.references.notifications))
            .unwrap_or((receiver, UserNotifications::new()));

        match notification_id {
            Some(notification_id) => {
                let user_notification_data = user_notifications.get(&notification_id);
                let notification_response = NotificationResponse::new(
                    Some(notification_id),
                    notification,
                    user_notification_data,
                );

                Websocket::send_message(
                    receiver,
                    WSMessage::Notification(notification_response.clone()),
                );
            }
            None => {
                Websocket::send_message(
                    receiver,
                    WSMessage::Notification(NotificationResponse::new(None, notification, None)),
                );
            }
        }
    }

    pub async fn add_and_send_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        let (new_notification_id, new_notification) =
            Self::add_notification(receivers.clone(), notification_type, is_actionable).await?;

        for receiver in receivers {
            Self::send_notification(
                Some(new_notification_id),
                new_notification.clone(),
                receiver,
            )
            .await;
        }

        Ok((new_notification_id, new_notification))
    }
}
