use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::poll::Poll;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollMessage {
    pub poll: Poll,
}

impl From<Inner> for PollMessage {
    fn from(inner: Inner) -> Self {
        let Inner { poll, .. } = inner;

        Self {
            poll: poll.unwrap(),
        }
    }
}
