use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}

impl From<IncomingMessage> for MessageAutoDeleteTimerChanged {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            message_auto_delete_timer_changed,
            ..
        } = remote;

        Self::from(message_auto_delete_timer_changed.unwrap())
    }
}
