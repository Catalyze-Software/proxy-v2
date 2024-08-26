use candid::Principal;
use catalyze_shared::{
    notification::{MultisigNotificationType, NotificationResponse},
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::UserNotificationData,
    CanisterResult,
};
use ic_cdk::{caller, query, update};

use crate::logic::notification_logic::NotificationCalls;

#[query(composite = true)]
async fn get_notifications() -> Vec<NotificationResponse> {
    NotificationCalls::get_user_notifications(caller()).await
}

#[query(composite = true)]
async fn get_unread_notifications() -> Vec<NotificationResponse> {
    NotificationCalls::get_user_unread_notifications(caller()).await
}

#[update]
async fn mark_notifications_as_read(
    ids: Vec<u64>,
    is_read: bool,
) -> CanisterResult<Vec<(u64, UserNotificationData)>> {
    NotificationCalls::mark_notifications_as_read(caller(), ids, is_read).await
}

#[update]
async fn remove_notifications(ids: Vec<u64>) -> Vec<(u64, UserNotificationData)> {
    NotificationCalls::remove_user_notifications(caller(), ids).await
}

#[update]
async fn remove_all_notifications() -> Vec<(u64, UserNotificationData)> {
    NotificationCalls::remove_all_user_notifications(caller()).await
}

#[update]
async fn add_transaction_notification(transaction: TransactionData) -> bool {
    if caller().to_string() != "4bli7-7iaaa-aaaap-ahd4a-cai" {
        return false;
    }

    NotificationCalls::notification_add_transaction(transaction).await
}

#[update]
async fn add_transactions_complete_notification(data: TransactionCompleteData) -> bool {
    if caller().to_string() != "4bli7-7iaaa-aaaap-ahd4a-cai" {
        return false;
    }

    NotificationCalls::notification_add_complete_transaction(data).await
}

#[update]
pub async fn multisig_whitelist_notice_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::WhitelistNotice((multisig_wallet_canister, group_id)),
    )
    .await
}

#[update]
pub async fn multisig_proposal_accept_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::ProposalAccept((multisig_wallet_canister, proposal_id, group_id)),
    )
    .await
}

#[update]
pub async fn multisig_proposal_decline_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::ProposalDecline((
            multisig_wallet_canister,
            proposal_id,
            group_id,
        )),
    )
    .await
}
#[update]
pub async fn multisig_proposal_status_update_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig_silent(
        receivers,
        MultisigNotificationType::ProposalStatusUpdate((
            multisig_wallet_canister,
            proposal_id,
            group_id,
        )),
    )
    .await
}
#[update]
pub async fn multisig_new_proposal_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::NewProposal((multisig_wallet_canister, proposal_id, group_id)),
    )
    .await
}
