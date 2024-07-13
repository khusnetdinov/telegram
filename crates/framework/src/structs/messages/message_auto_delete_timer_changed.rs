use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}
impl From<Remote> for MessageAutoDeleteTimerChanged {
    fn from(remote: Remote) -> Self {
        Self {
            message_auto_delete_time: remote.message_auto_delete_time,
        }
    }
}

impl From<Message> for MessageAutoDeleteTimerChanged {
    fn from(remote: Message) -> Self {
        let Message {
            message_auto_delete_timer_changed: Some(message_auto_delete_timer_changed),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(message_auto_delete_timer_changed)
    }
}
