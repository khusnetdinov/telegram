use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PinnedMessage {
    pub pinned_message: Box<MaybeInaccessibleMessage>,
}

impl From<Inner> for PinnedMessage {
    fn from(inner: Inner) -> Self {
        let Inner { pinned_message, .. } = inner;

        Self {
            pinned_message: pinned_message.unwrap(),
        }
    }
}
