use crate::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessagePinned {
    pub pinned_message: Box<MaybeInaccessibleMessage>,
}

impl From<IncomingMessage> for MessagePinned {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { pinned_message, .. } = remote;

        Self {
            pinned_message: Box::new((*pinned_message.unwrap()).into()),
        }
    }
}
