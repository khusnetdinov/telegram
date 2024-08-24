use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}

impl From<Message> for MessageAutoDeleteTimerChanged {
    fn from(remote: Message) -> Self {
        let Message {
            message_auto_delete_timer_changed,
            ..
        } = remote;

        Self::from(message_auto_delete_timer_changed.unwrap())
    }
}
