use crate::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PinnedMessage {
    pub pinned_message: Box<MaybeInaccessibleMessage>,
}

impl From<Message> for PinnedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            pinned_message: Some(pinned_message),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self {
            // TODO: #[remote(into)]
            pinned_message: Box::new((*pinned_message).into()),
        }
    }
}
