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
    profile_with_refs::{ProfileEntry, ProfileWithRefs},
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::{UserNotificationData, UserNotifications},
    websocket_message::WSMessage,
    CanisterResult, StorageClient, StorageClientInsertable,
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
    ) -> CanisterResult<u64> {
        let to = profiles().get(friend_request.to).await?;
        let (notification_id, notification) = Self::add_notification(
            vec![to.clone()],
            NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request.clone(),
            )),
            true,
        )
        .await?;

        Self::send_notification(Some(notification_id), notification, to);
        Ok(notification_id)
    }

    /// stores + sends notification
    pub async fn notification_accept_or_decline_friend_request(
        friend_request_data: (u64, FriendRequest),
        is_accepted: bool,
    ) -> CanisterResult<()> {
        // get the associated friend request
        let (_, friend_request) = friend_request_data;

        let notification_id = friend_request
            .notification_id
            .ok_or_else(ApiError::not_found)?;

        // check if the notification exists
        let (_, mut notification) = notifications().get(notification_id).await?;

        let friend_request = match notification.notification_type.clone() {
            NotificationType::Relation(RelationNotificationType::FriendRequest(friend_request)) => {
                friend_request
            }
            _ => {
                return Err(
                    ApiError::bad_request().add_message("Notification is not a friend request")
                )
            }
        };

        // mark the notification as accepted, this also marks it as not actionable
        let notification_type = match is_accepted {
            true => RelationNotificationType::FriendRequestAccept(friend_request.clone()),
            false => RelationNotificationType::FriendRequestDecline(friend_request.clone()),
        };

        notification.mark_as_accepted(is_accepted, NotificationType::Relation(notification_type));

        let _ = notifications()
            .update(notification_id, notification.clone())
            .await?;

        let profiles = profiles()
            .get_many(vec![friend_request.requested_by, friend_request.to])
            .await?;

        for (principal, profile) in profiles {
            let mut notification_id = Some(notification_id);

            if principal == friend_request.to {
                notification_id = None;
            }

            if principal != friend_request.requested_by && principal != friend_request.to {
                continue;
            }

            Self::send_notification(
                notification_id,
                notification.clone(),
                (principal, profile.clone()),
            )
        }

        Ok(())
    }

    // sends notification
    pub async fn notification_remove_friend_request(receiver: Principal, friend_request_id: u64) {
        if let Ok(profile) = profiles().get(receiver).await {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Relation(RelationNotificationType::FriendRequestRemove(
                        friend_request_id,
                    )),
                    false,
                ),
                profile,
            );
        }
    }

    // sends notification
    pub async fn notification_remove_friend(receiver: Principal, friend_principal: Principal) {
        if let Ok(profile) = profiles().get(receiver).await {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Relation(RelationNotificationType::FriendRemove(
                        friend_principal,
                    )),
                    false,
                ),
                profile,
            );
        }
    }

    // Group notifications

    // sends notification
    pub async fn notification_join_public_group(receivers: Vec<Principal>, group_id: u64) {
        let profiles = profiles().get_many(receivers).await.unwrap_or_default();
        for receiver in profiles {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserJoinGroup(group_id)),
                    false,
                ),
                receiver,
            );
        }
    }

    pub async fn notification_leave_group(receivers: Vec<Principal>, group_id: u64) {
        let profiles = profiles().get_many(receivers).await.unwrap_or_default();
        for receiver in profiles {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserLeaveGroup(group_id)),
                    false,
                ),
                receiver,
            );
        }
    }

    // stores + sends notification
    pub async fn notification_user_join_request_group(
        receivers: Vec<Principal>,
        invite_member_response: InviteMemberResponse,
    ) -> CanisterResult<u64> {
        let (notification_id, _) = Self::add_and_send_notification(
            profiles().get_many(receivers).await.unwrap_or_default(),
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
    ) -> CanisterResult<()> {
        let notification_id = invite.notification_id.ok_or_else(|| {
            ApiError::bad_request().add_message("Notification is not a user join group request")
        })?;

        let (_, mut notification) = notifications().get(notification_id).await?;

        let invite_member_response = match notification.notification_type.clone() {
            NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(
                invite_member_response_resp,
            )) => invite_member_response_resp,
            _ => {
                return Err(ApiError::bad_request()
                    .add_message("Notification is not a user join group request"))
            }
        };

        let notification_type = match is_accepted {
            true => GroupNotificationType::JoinGroupUserRequestAccept(invite_member_response),
            false => GroupNotificationType::JoinGroupUserRequestDecline(invite_member_response),
        };

        notification.mark_as_accepted(is_accepted, NotificationType::Group(notification_type));

        let _ = notifications()
            .update(notification_id, notification.clone())
            .await?;

        let principals = [
            vec![notification.sender],
            group_members.clone(),
            higher_role_members.clone(),
        ]
        .concat();

        let profiles = profiles().get_many(principals).await?;

        for (principal, profile) in profiles {
            if notification.sender == principal {
                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    (principal, profile.clone()), // the person who request to join
                );
            }

            if is_accepted && higher_role_members.contains(&principal) {
                Self::send_notification(
                    None,
                    notification.clone(),
                    (principal, profile.clone()), // the group members
                );

                continue;
            }

            if group_members.contains(&principal) {
                Self::send_notification(
                    None,
                    notification.clone(),
                    (principal, profile.clone()), // the group members
                );
            }
        }

        Ok(())
    }

    // stores + sends notification
    pub async fn notification_owner_join_request_group(
        invitee_principal: Principal,
        invite_member_response: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) -> CanisterResult<u64> {
        let mut data: Option<(u64, Notification)> = None;

        let profiles = profiles()
            .get_many([vec![invitee_principal], receivers.clone()].concat())
            .await?;

        for (principal, profile) in profiles {
            if principal == invitee_principal {
                let notification = Self::add_and_send_notification(
                    vec![(principal, profile.clone())],
                    NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                        invite_member_response.clone(),
                    )),
                    true,
                )
                .await?;

                data = Some(notification);
                continue;
            }

            if receivers.contains(&principal) {
                if let Some((_, notification)) = data.clone() {
                    Self::send_notification(None, notification, (principal, profile.clone()));
                }
            }
        }

        Ok(data.ok_or_else(ApiError::not_found)?.0)
    }

    // sends notification
    pub async fn notification_owner_join_request_group_accept_or_decline(
        invitee_principal: Principal,
        invite: MemberInvite,
        is_accepted: bool,
        group_members: Vec<Principal>,
        higher_role_members: Vec<Principal>,
    ) -> CanisterResult<()> {
        let notification_id = invite.notification_id.ok_or_else(|| {
            ApiError::bad_request().add_message("Notification is not a user join group request")
        })?;

        let (_, mut notification) = notifications().get(notification_id).await?;

        let keys = [
            vec![invitee_principal],
            group_members.clone(),
            higher_role_members.clone(),
        ]
        .concat();

        let profiles = profiles().get_many(keys).await?;

        let invite_member_response = match notification.notification_type.clone() {
            NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                invite_member_response,
            )) => invite_member_response,
            _ => return Err(ApiError::bad_request().add_message("")),
        };

        let notification_type = match is_accepted {
            true => GroupNotificationType::JoinGroupOwnerRequestAccept(invite_member_response),
            false => GroupNotificationType::JoinGroupOwnerRequestDecline(invite_member_response),
        };

        notification.mark_as_accepted(is_accepted, NotificationType::Group(notification_type));

        let _ = notifications()
            .update(notification_id, notification.clone())
            .await;

        for (principal, profile) in profiles {
            let mut notification_id = Some(notification_id);

            if notification.sender != principal || principal == invitee_principal {
                notification_id = None;
            }

            Self::send_notification(
                notification_id,
                notification.clone(),
                (principal, profile.clone()),
            );
        }

        Ok(())
    }

    pub async fn notification_change_group_member_role(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        let profiles = profiles().get_many(receivers).await.unwrap_or_default();
        for receiver in profiles {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RoleAssignByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                receiver,
            );
        }
    }

    pub async fn notification_remove_group_member(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        let profiles = profiles()
            .get_many([vec![member.principal], receivers.clone()].concat())
            .await
            .unwrap_or_default();

        for (principal, profile) in profiles {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RemoveMemberByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                (principal, profile.clone()),
            );
        }
    }

    pub async fn notification_remove_group_invite(
        invite: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) {
        let member_invite = match invite.invite.clone() {
            Some(member_invite) => member_invite,
            None => return,
        };

        let notification_id = match member_invite.notification_id {
            Some(notification_id) => notification_id,
            None => return,
        };

        let mut notification = match notifications().get(notification_id).await {
            Ok((_, notification)) => notification,
            Err(_) => return,
        };

        notification.mark_as_accepted(
            false,
            NotificationType::Group(GroupNotificationType::RemoveInviteByOwner(invite.clone())),
        );

        let _ = notifications()
            .update(notification_id, notification.clone())
            .await;

        profiles()
            .get_many([vec![invite.principal], receivers].concat())
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|receiver| {
                Self::send_notification(None, notification.clone(), receiver);
            });
    }

    // Event notifications

    // sends notification
    pub async fn notification_join_public_event(
        receivers: Vec<Principal>,
        group_id: u64,
        event_id: u64,
    ) {
        let profiles = profiles().get_many(receivers).await.unwrap_or_default();
        for receiver in profiles {
            let event = EventNotificationType::UserJoinEvent((group_id, event_id));

            Self::send_notification(
                None,
                Notification::new(NotificationType::Event(event), false),
                receiver,
            );
        }
    }

    // store + sends notification
    pub async fn notification_user_join_request_event(
        receivers: Vec<Principal>,
        invite_attendee_response: InviteAttendeeResponse,
    ) -> CanisterResult<u64> {
        let profiles = profiles().get_many(receivers).await?;
        let event = EventNotificationType::JoinEventUserRequest(invite_attendee_response);

        Self::add_and_send_notification(profiles, NotificationType::Event(event), true)
            .await
            .map(|(id, _)| id)
    }

    // sends notifications
    pub async fn notification_user_join_request_event_accept_or_decline(
        receiver: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> CanisterResult<()> {
        let notification_id = invite.notification_id.ok_or_else(|| {
            ApiError::bad_request().add_message("Notification is not a user join group request")
        })?;

        let (_, mut notification) = notifications().get(notification_id).await?;

        let invite_attendee_response = match notification.notification_type.clone() {
            NotificationType::Event(EventNotificationType::JoinEventUserRequest(
                invite_attendee_response,
            )) => invite_attendee_response,
            _ => {
                return Err(ApiError::bad_request()
                    .add_message("Notification is not a user join group request"))
            }
        };

        let notification_type = match is_accepted {
            true => EventNotificationType::JoinEventUserRequestAccept(invite_attendee_response),
            false => EventNotificationType::JoinEventUserRequestDecline(invite_attendee_response),
        };

        notification.mark_as_accepted(is_accepted, NotificationType::Event(notification_type));

        let _ = notifications()
            .update(notification_id, notification.clone())
            .await;

        let profiles = profiles()
            .get_many([vec![receiver], event_attendees.clone()].concat())
            .await?;

        for (principal, profile) in profiles {
            if principal == receiver {
                Self::send_notification(None, notification.clone(), (principal, profile));
            } else if is_accepted {
                if notification.sender == principal {
                    Self::send_notification(
                        Some(notification_id),
                        notification.clone(),
                        (principal, profile),
                    );
                } else {
                    Self::send_notification(None, notification.clone(), (principal, profile));
                }
            }
        }

        Ok(())
    }

    // sends notification
    pub async fn notification_owner_join_request_event_accept_or_decline(
        invitee_principal: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> CanisterResult<()> {
        let notification_id = invite.notification_id.ok_or_else(|| {
            ApiError::bad_request().add_message("Notification is not a user join group request")
        })?;

        let (_, mut notification) = notifications().get(notification_id).await?;

        let event_attendee_response = match notification.notification_type.clone() {
            NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                event_attendee_response,
            )) => event_attendee_response,
            _ => {
                return Err(ApiError::bad_request()
                    .add_message("Notification is not a user join group request"))
            }
        };

        let notification_type = match is_accepted {
            true => EventNotificationType::JoinEventOwnerRequestAccept(event_attendee_response),
            false => EventNotificationType::JoinEventOwnerRequestDecline(event_attendee_response),
        };

        notification.mark_as_accepted(is_accepted, NotificationType::Event(notification_type));
        let _ = notifications()
            .update(notification_id, notification.clone())
            .await;

        let profiles = profiles()
            .get_many([vec![invitee_principal], event_attendees.clone()].concat())
            .await?;

        for (principal, profile) in profiles {
            if invitee_principal == principal || is_accepted && notification.sender != principal {
                Self::send_notification(None, notification.clone(), (principal, profile.clone()));
            }

            if is_accepted && notification.sender == principal {
                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    (principal, profile),
                );
            }
        }

        Ok(())
    }

    // stores + sends notification
    pub async fn notification_owner_join_request_event(
        invitee_principal: Principal,
        invite_attendee_response: InviteAttendeeResponse,
        receivers: Vec<Principal>,
    ) -> CanisterResult<u64> {
        let profiles = profiles()
            .get_many([vec![invitee_principal], receivers.clone()].concat())
            .await?;

        let mut notification: Option<(u64, Notification)> = None;

        for (principal, profile) in profiles {
            if principal == invitee_principal {
                let data = Self::add_and_send_notification(
                    vec![(principal, profile.clone())],
                    NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                        invite_attendee_response.clone(),
                    )),
                    true,
                )
                .await?;

                notification = Some(data);
            } else if receivers.contains(&principal) {
                if let Some((_, notification)) = notification.clone() {
                    Self::send_notification(
                        None,
                        notification.clone(),
                        (principal, profile.clone()),
                    );
                }
            }
        }

        Ok(notification.ok_or_else(ApiError::not_found)?.0)
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

            if let Ok(profile) = profiles().get(invite.principal).await {
                Self::send_notification(None, notification.clone(), profile);
            }
        }
    }

    pub async fn notification_remove_event_attendee(
        attendee: JoinedAttendeeResponse,
        receivers: Vec<Principal>,
    ) {
        let profiles: Vec<(Principal, ProfileWithRefs)> = profiles()
            .get_many([vec![attendee.principal], receivers.clone()].concat())
            .await
            .unwrap_or_default();

        for (principal, profile) in profiles {
            let event = EventNotificationType::RemoveAttendeeByOwner(attendee.clone());

            Self::send_notification(
                None,
                Notification::new(NotificationType::Event(event), false),
                (principal, profile.clone()),
            );
        }
    }

    // Transaction notifications
    pub async fn notification_add_transaction(transaction: TransactionData) -> bool {
        if let Ok(profile) = profiles().get(transaction.receiver).await {
            let _ = Self::add_and_send_notification(
                vec![profile],
                NotificationType::Transaction(TransactionNotificationType::SingleTransaction(
                    transaction,
                )),
                false,
            )
            .await
            .is_ok();
        }

        false
    }

    pub async fn notification_add_complete_transaction(data: TransactionCompleteData) -> bool {
        if let Ok(profile) = profiles().get(data.sender).await {
            let _ = Self::add_and_send_notification(
                vec![profile],
                NotificationType::Transaction(TransactionNotificationType::TransactionsComplete(
                    data,
                )),
                false,
            )
            .await
            .is_ok();
        }

        false
    }

    pub async fn notification_add_multisig(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }

        Self::add_and_send_notification(
            profiles().get_many(receivers).await.unwrap_or_default(),
            NotificationType::Multisig(notification),
            false,
        )
        .await
        .is_ok()
    }

    pub async fn notification_add_multisig_silent(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }

        for profile in profiles().get_many(receivers).await.unwrap_or_default() {
            Self::send_notification(
                None,
                Notification::new(NotificationType::Multisig(notification.clone()), false),
                profile,
            );
        }
        true
    }

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
    ) -> CanisterResult<Vec<(u64, UserNotificationData)>> {
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
        receivers: Vec<ProfileEntry>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> CanisterResult<(u64, Notification)> {
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

        for (receiver_principal, mut receiver_profile) in receivers {
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

    pub fn send_notification(
        // If the notification is silent, the notification id is not required as its not stored in the user's notifications
        notification_id: Option<u64>,
        notification: Notification,
        (receiver, profile): ProfileEntry,
    ) {
        match notification_id {
            Some(notification_id) => {
                let user_notification_data = profile.references.notifications.get(&notification_id);
                let notification_response = NotificationResponse::new(
                    Some(notification_id),
                    notification,
                    user_notification_data,
                );

                Websocket::send_message(receiver, WSMessage::Notification(notification_response));
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
        receivers: Vec<ProfileEntry>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> CanisterResult<(u64, Notification)> {
        let (new_notification_id, new_notification) =
            Self::add_notification(receivers.clone(), notification_type, is_actionable).await?;

        for receiver in receivers {
            Self::send_notification(
                Some(new_notification_id),
                new_notification.clone(),
                receiver,
            );
        }

        Ok((new_notification_id, new_notification))
    }
}
