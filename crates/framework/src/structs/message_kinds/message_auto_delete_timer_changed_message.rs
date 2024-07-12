use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChangedMessage {
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

impl From<Message> for MessageAutoDeleteTimerChangedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            message_auto_delete_timer_changed,
            ..
        } = remote;

        Self {
            message_auto_delete_timer_changed: message_auto_delete_timer_changed.unwrap(),
        }
    }
}
