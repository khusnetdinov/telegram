use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChangedMessage {
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

impl From<Inner> for MessageAutoDeleteTimerChangedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            message_auto_delete_timer_changed,
            ..
        } = inner;

        Self {
            message_auto_delete_timer_changed: message_auto_delete_timer_changed.unwrap(),
        }
    }
}
