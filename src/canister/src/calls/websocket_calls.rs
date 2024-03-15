use ic_cdk::{query, update};
use ic_websocket_cdk::{
    types::WebsocketMessage, CanisterWsCloseArguments, CanisterWsCloseResult,
    CanisterWsGetMessagesArguments, CanisterWsGetMessagesResult, CanisterWsMessageArguments,
    CanisterWsMessageResult, CanisterWsOpenArguments, CanisterWsOpenResult,
};

#[update]
fn ws_open(args: CanisterWsOpenArguments) -> CanisterWsOpenResult {
    ic_websocket_cdk::ws_open(args)
}

#[update]
fn ws_close(args: CanisterWsCloseArguments) -> CanisterWsCloseResult {
    ic_websocket_cdk::ws_close(args)
}

#[update]
fn ws_message(
    args: CanisterWsMessageArguments,
    msg_type: Option<WebsocketMessage>,
) -> CanisterWsMessageResult {
    ic_websocket_cdk::ws_message(args, msg_type)
}

#[query]
fn ws_get_messages(args: CanisterWsGetMessagesArguments) -> CanisterWsGetMessagesResult {
    ic_websocket_cdk::ws_get_messages(args)
}
