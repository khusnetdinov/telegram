use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PinnedMessage {
    pub pinned_message: Box<MaybeInaccessibleMessage>,
}

impl From<Message> for PinnedMessage {
    fn from(remote: Message) -> Self {
        let Message { pinned_message, .. } = remote;

        Self {
            pinned_message: pinned_message.unwrap(),
        }
    }
}
