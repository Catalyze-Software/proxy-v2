use candid::Principal;
use canister_types::models::notification::NotificationType;

pub struct NotificationCalls;

impl NotificationCalls {
    pub fn new(to: Vec<Principal>, silent: bool, notification_type: NotificationType) -> () {}
}
