use std::{cell::RefCell, collections::HashMap};

use candid::Principal;
use canister_types::models::{
    unread_count::UnreadNotifications, websocket_message::WebsocketMessage,
};
use ic_cdk::api::time;
use ic_websocket_cdk::{
    ws_send, OnCloseCallbackArgs, OnMessageCallbackArgs, OnOpenCallbackArgs, WsHandlers,
    WsInitParams,
};

use crate::storage::{StorageMethods, UnreadNotificationStore};

// type TimeInNanos = u64;

thread_local! {
   pub static CONNECTED_CLIENTS: RefCell<HashMap<Principal, u64>> = RefCell::new(HashMap::new());
   pub static WS_ERRORS: RefCell<HashMap<u64, (u64, String)>> = RefCell::new(HashMap::new());
}

pub struct Websocket;

impl Websocket {
    pub fn init() {
        let handlers = WsHandlers {
            on_open: Some(Websocket::on_open),
            on_message: Some(Websocket::on_message),
            on_close: Some(Websocket::on_close),
        };

        let params = WsInitParams::new(handlers);
        ic_websocket_cdk::init(params);
    }

    pub fn on_open(args: OnOpenCallbackArgs) {
        Self::add_connected_to_clients(args.client_principal);

        let (_, notification_ids) = UnreadNotificationStore::get(args.client_principal)
            .unwrap_or((args.client_principal, UnreadNotifications::new()));
        // ::get_unread_notifications(&args.client_principal).len() as u64;
        Self::send_app_message(
            args.client_principal,
            WebsocketMessage::UnreadCount(notification_ids.len() as u64),
        );
    }

    pub fn on_close(args: OnCloseCallbackArgs) {
        Self::remove_connected_from_clients(&args.client_principal);
    }

    pub fn on_message(args: OnMessageCallbackArgs) {
        match WebsocketMessage::deserialize(&args.message) {
            WebsocketMessage::Notification(value) => {
                for receiver in value.receivers.clone() {
                    if Self::is_connected(&receiver) {
                        Self::send_app_message(
                            receiver,
                            WebsocketMessage::Notification(value.clone()),
                        );
                    }
                }
            }
            _ => Self::log_error("Unknown message type".to_string()),
        };
    }

    pub fn send_app_message(principal: Principal, msg: WebsocketMessage) {
        match ws_send(principal, msg.serialize()) {
            Ok(_) => {}
            Err(e) => {
                Self::log_error(e.to_string());
            }
        }
    }

    fn add_connected_to_clients(principal: Principal) {
        CONNECTED_CLIENTS.with(|c| c.borrow_mut().insert(principal, time()));
    }

    fn remove_connected_from_clients(principal: &Principal) {
        CONNECTED_CLIENTS.with(|c| c.borrow_mut().remove(&principal));
    }

    pub fn is_connected(principal: &Principal) -> bool {
        CONNECTED_CLIENTS.with(|c| c.borrow().contains_key(&principal))
    }

    pub fn log_error(error: String) {
        const MAX_LOGS: usize = 500;

        WS_ERRORS.with(|errors| {
            let mut errors = errors.borrow_mut();

            let next_id = errors.len() as u64 + 1;

            errors.insert(next_id, (time(), error));

            if errors.len() > MAX_LOGS {
                let oldest_log_id = *errors.keys().min().unwrap(); // This is safe given that we always have at least one log.
                errors.remove(&oldest_log_id);
            }
        });
    }
}
