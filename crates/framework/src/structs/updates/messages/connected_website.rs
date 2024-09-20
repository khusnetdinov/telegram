use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectedWebsite {
    pub connected_website: String,
}

impl From<Message> for ConnectedWebsite {
    fn from(remote: Message) -> Self {
        let Message {
            connected_website, ..
        } = remote;

        Self {
            connected_website: connected_website.unwrap(),
        }
    }
}
